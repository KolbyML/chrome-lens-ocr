pub mod constants;
pub mod image_processor;
pub mod proto;

use std::f32::consts::PI;
use std::time::Duration;

use anyhow::{Result, anyhow};
use prost::Message;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue, USER_AGENT};

use crate::{constants::*, proto::*};

// --- Public Data Structures ---

#[derive(Debug, Clone)]
pub struct LensResult {
    /// The full text combined with newlines.
    pub full_text: String,
    /// Detailed paragraph structure.
    pub paragraphs: Vec<Paragraph>,
    /// Translated text if available (requires target language).
    pub translation: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Paragraph {
    pub text: String,
    pub lines: Vec<Line>,
    pub geometry: Option<GeometryData>,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub text: String,
    pub words: Vec<Word>,
    pub geometry: Option<GeometryData>,
}

#[derive(Debug, Clone)]
pub struct Word {
    pub text: String,
    pub separator: String,
    pub geometry: Option<GeometryData>,
}

#[derive(Debug, Clone)]
pub struct GeometryData {
    pub center_x: f32,
    pub center_y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation_z: f32,
    pub angle_deg: f32,
}

// --- Client Implementation ---

pub struct LensClient {
    client: reqwest::Client,
    api_key: String,
}

impl LensClient {
    pub fn new(api_key: Option<String>) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap_or_default();

        Self {
            client,
            api_key: api_key.unwrap_or_else(|| DEFAULT_API_KEY.to_string()),
        }
    }

    pub async fn process_image_path(&self, path: &str, lang: Option<&str>) -> Result<LensResult> {
        let processed = image_processor::process_image_from_path(path)?;
        self.send_request(processed, lang).await
    }

    pub async fn process_image_bytes(
        &self,
        bytes: &[u8],
        lang: Option<&str>,
    ) -> Result<LensResult> {
        let processed = image_processor::process_image_from_bytes(bytes)?;
        self.send_request(processed, lang).await
    }

    async fn send_request(
        &self,
        image: image_processor::ProcessedImage,
        lang: Option<&str>,
    ) -> Result<LensResult> {
        let request_id_val = rand::random::<u64>();

        let req_proto = LensOverlayServerRequest {
            objects_request: Some(LensOverlayObjectsRequest {
                request_context: Some(LensOverlayRequestContext {
                    request_id: Some(LensOverlayRequestId {
                        uuid: request_id_val,
                        sequence_id: 1,
                        image_sequence_id: 1,
                    }),
                    client_context: Some(LensOverlayClientContext {
                        platform: Platform::Web as i32,
                        surface: Surface::Chromium as i32,
                        locale_context: Some(LocaleContext {
                            language: lang.unwrap_or("en").to_string(),
                            region: DEFAULT_CLIENT_REGION.to_string(),
                            time_zone: DEFAULT_CLIENT_TIME_ZONE.to_string(),
                        }),
                    }),
                }),
                image_data: Some(ImageData {
                    payload: Some(ImagePayload {
                        image_bytes: image.bytes,
                    }),
                    image_metadata: Some(ImageMetadata {
                        width: image.width,
                        height: image.height,
                    }),
                }),
            }),
        };

        let mut payload_bytes = Vec::new();
        req_proto.encode(&mut payload_bytes)?;

        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-protobuf"),
        );
        headers.insert(USER_AGENT, HeaderValue::from_static(DEFAULT_USER_AGENT));
        headers.insert("X-Goog-Api-Key", HeaderValue::from_str(&self.api_key)?);

        let response = self
            .client
            .post(LENS_CRUPLOAD_ENDPOINT)
            .headers(headers)
            .body(payload_bytes)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            return Err(anyhow!("API Error {}: {}", status, text));
        }

        let resp_bytes = response.bytes().await?;

        let server_response = LensOverlayServerResponse::decode(resp_bytes)
            .map_err(|e| anyhow!("Failed to decode protobuf response: {}", e))?;

        self.parse_response(server_response)
    }

    // --- Parsing Logic (Ported from api.py) ---

    fn parse_response(&self, response: LensOverlayServerResponse) -> Result<LensResult> {
        let mut paragraphs_list = Vec::new();
        let mut full_text_buffer = String::new();

        // Extract OCR Data
        if let Some(objects_res) = &response.objects_response {
            if let Some(text_struct) = &objects_res.text {
                if let Some(layout) = &text_struct.text_layout {
                    for p in &layout.paragraphs {
                        let parsed_para = self.parse_paragraph(p);

                        full_text_buffer.push_str(&parsed_para.text);
                        full_text_buffer.push('\n'); // Standardize paragraph separation

                        paragraphs_list.push(parsed_para);
                    }
                }
            }
        }

        // Extract Translation
        let translation = self.extract_translation(&response);

        if full_text_buffer.trim().is_empty() && translation.is_none() {
            return Err(anyhow!("No text or translation found in image"));
        }

        Ok(LensResult {
            full_text: full_text_buffer.trim().to_string(),
            paragraphs: paragraphs_list,
            translation,
        })
    }

    fn parse_paragraph(&self, p: &TextLayoutParagraph) -> Paragraph {
        let mut lines_list = Vec::new();
        let mut para_text_parts = Vec::new();

        for l in &p.lines {
            let parsed_line = self.parse_line(l);
            para_text_parts.push(parsed_line.text.clone());
            lines_list.push(parsed_line);
        }

        let full_para_text = para_text_parts.join("\n");
        let geometry = p.geometry.as_ref().and_then(|g| self.parse_geometry(g));

        Paragraph {
            text: full_para_text,
            lines: lines_list,
            geometry,
        }
    }

    fn parse_line(&self, l: &TextLayoutLine) -> Line {
        let mut words_list = Vec::new();
        let mut line_text_buffer = String::new();

        for w in &l.words {
            let parsed_word = self.parse_word(w);
            line_text_buffer.push_str(&parsed_word.text);
            line_text_buffer.push_str(&parsed_word.separator);
            words_list.push(parsed_word);
        }

        let geometry = l.geometry.as_ref().and_then(|g| self.parse_geometry(g));

        Line {
            text: line_text_buffer.trim().to_string(),
            words: words_list,
            geometry,
        }
    }

    fn parse_word(&self, w: &TextLayoutWord) -> Word {
        let sep = w.text_separator.clone().unwrap_or_default();
        let geometry = w.geometry.as_ref().and_then(|g| self.parse_geometry(g));

        Word {
            text: w.plain_text.clone(),
            separator: sep,
            geometry,
        }
    }

    fn parse_geometry(&self, g: &Geometry) -> Option<GeometryData> {
        let bb = g.bounding_box.as_ref()?;
        let angle_deg = bb.rotation_z * (180.0 / PI);

        Some(GeometryData {
            center_x: bb.center_x,
            center_y: bb.center_y,
            width: bb.width,
            height: bb.height,
            rotation_z: bb.rotation_z,
            angle_deg,
        })
    }

    fn extract_translation(&self, response: &LensOverlayServerResponse) -> Option<String> {
        let mut translations = Vec::new();

        if let Some(objects_res) = &response.objects_response {
            for gleam in &objects_res.deep_gleams {
                if let Some(trans_data) = &gleam.translation {
                    if let Some(status) = &trans_data.status {
                        // TranslationStatus::Success is enum value 1
                        if status.code == TranslationStatus::Success as i32 {
                            if !trans_data.translation.is_empty() {
                                translations.push(trans_data.translation.clone());
                            }
                        }
                    }
                }
            }
        }

        if translations.is_empty() {
            None
        } else {
            Some(translations.join("\n"))
        }
    }
}
