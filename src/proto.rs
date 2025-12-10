use prost::Message;

// --- Enums ---

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Platform {
    Unspecified = 0,
    Web = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Surface {
    Unspecified = 0,
    Chromium = 4,
}

// --- Messages ---

#[derive(Clone, PartialEq, Message)]
pub struct LensOverlayServerRequest {
    #[prost(message, optional, tag = "1")]
    pub objects_request: Option<LensOverlayObjectsRequest>,
}

#[derive(Clone, PartialEq, Message)]
pub struct LensOverlayObjectsRequest {
    #[prost(message, optional, tag = "1")]
    pub request_context: Option<LensOverlayRequestContext>,
    #[prost(message, optional, tag = "3")]
    pub image_data: Option<ImageData>,
}

#[derive(Clone, PartialEq, Message)]
pub struct LensOverlayRequestContext {
    #[prost(message, optional, tag = "3")]
    pub request_id: Option<LensOverlayRequestId>,
    #[prost(message, optional, tag = "4")]
    pub client_context: Option<LensOverlayClientContext>,
}

#[derive(Clone, PartialEq, Message)]
pub struct LensOverlayRequestId {
    #[prost(uint64, tag = "1")]
    pub uuid: u64,
    #[prost(int32, tag = "2")]
    pub sequence_id: i32,
    #[prost(int32, tag = "3")]
    pub image_sequence_id: i32,
}

#[derive(Clone, PartialEq, Message)]
pub struct LensOverlayClientContext {
    #[prost(enumeration = "Platform", tag = "1")]
    pub platform: i32,
    #[prost(enumeration = "Surface", tag = "2")]
    pub surface: i32,
    #[prost(message, optional, tag = "4")]
    pub locale_context: Option<LocaleContext>,
}

#[derive(Clone, PartialEq, Message)]
pub struct LocaleContext {
    #[prost(string, tag = "1")]
    pub language: String,
    #[prost(string, tag = "2")]
    pub region: String,
    #[prost(string, tag = "3")]
    pub time_zone: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct ImageData {
    #[prost(message, optional, tag = "1")]
    pub payload: Option<ImagePayload>,
    #[prost(message, optional, tag = "3")]
    pub image_metadata: Option<ImageMetadata>,
}

#[derive(Clone, PartialEq, Message)]
pub struct ImagePayload {
    #[prost(bytes = "vec", tag = "1")]
    pub image_bytes: Vec<u8>,
}

#[derive(Clone, PartialEq, Message)]
pub struct ImageMetadata {
    #[prost(int32, tag = "1")]
    pub width: i32,
    #[prost(int32, tag = "2")]
    pub height: i32,
}

// --- Response Messages ---

#[derive(Clone, PartialEq, Message)]
pub struct LensOverlayServerResponse {
    #[prost(message, optional, tag = "2")]
    pub objects_response: Option<LensOverlayObjectsResponse>,
}

#[derive(Clone, PartialEq, Message)]
pub struct LensOverlayObjectsResponse {
    #[prost(message, optional, tag = "3")]
    pub text: Option<Text>,
}

#[derive(Clone, PartialEq, Message)]
pub struct Text {
    #[prost(message, optional, tag = "1")]
    pub text_layout: Option<TextLayout>,
    #[prost(string, tag = "2")]
    pub content_language: String,
}

#[derive(Clone, PartialEq, Message)]
pub struct TextLayout {
    #[prost(message, repeated, tag = "1")]
    pub paragraphs: Vec<TextLayoutParagraph>,
}

#[derive(Clone, PartialEq, Message)]
pub struct TextLayoutParagraph {
    #[prost(message, repeated, tag = "2")]
    pub lines: Vec<TextLayoutLine>,
}

#[derive(Clone, PartialEq, Message)]
pub struct TextLayoutLine {
    #[prost(message, repeated, tag = "1")]
    pub words: Vec<TextLayoutWord>,
}

#[derive(Clone, PartialEq, Message)]
pub struct TextLayoutWord {
    #[prost(string, tag = "2")]
    pub plain_text: String,
    #[prost(string, optional, tag = "3")]
    pub text_separator: Option<String>,
}
