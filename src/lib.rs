pub mod constants;
pub mod image_processor;
pub mod proto;

use std::time::Duration;

use anyhow::{Result, anyhow};
use prost::Message;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue, USER_AGENT};

use crate::{constants::*, proto::*};

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

    pub async fn process_image_path(&self, path: &str, lang: Option<&str>) -> Result<String> {
        let processed = image_processor::process_image_from_path(path)?;
        self.send_request(processed, lang).await
    }

    pub async fn process_image_bytes(&self, bytes: &[u8], lang: Option<&str>) -> Result<String> {
        let processed = image_processor::process_image_from_bytes(bytes)?;
        self.send_request(processed, lang).await
    }

    async fn send_request(
        &self,
        image: image_processor::ProcessedImage,
        lang: Option<&str>,
    ) -> Result<String> {
        let request_id_val = rand::random::<u64>();

        // 1. Build Protobuf Request
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

        // 2. Prepare Headers
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-protobuf"),
        );
        headers.insert(USER_AGENT, HeaderValue::from_static(DEFAULT_USER_AGENT));
        headers.insert("X-Goog-Api-Key", HeaderValue::from_str(&self.api_key)?);

        // 3. Send Request
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

        // 4. Decode Response
        let server_response = LensOverlayServerResponse::decode(resp_bytes)
            .map_err(|e| anyhow!("Failed to decode protobuf response: {}", e))?;

        // 5. Extract Text
        self.extract_text(server_response)
    }

    fn extract_text(&self, response: LensOverlayServerResponse) -> Result<String> {
        let mut full_text = String::new();

        if let Some(objects_res) = response.objects_response
            && let Some(text_struct) = objects_res.text
            && let Some(layout) = text_struct.text_layout
        {
            for paragraph in layout.paragraphs {
                for line in paragraph.lines {
                    for word in line.words {
                        full_text.push_str(&word.plain_text);
                        if let Some(sep) = word.text_separator {
                            full_text.push_str(&sep);
                        }
                    }
                    full_text.push('\n');
                }
                full_text.push('\n');
            }
        }

        if full_text.trim().is_empty() {
            return Err(anyhow!("No text found in image"));
        }

        Ok(full_text.trim().to_string())
    }
}
