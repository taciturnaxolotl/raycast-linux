use crate::error::AppError;
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

pub const INLINE_CONTENT_THRESHOLD_BYTES: i64 = 10_000; // 10 KB
pub const PREVIEW_LENGTH_CHARS: usize = 500;

pub static COLOR_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$").unwrap());
pub static URL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(https?|ftp)://[^\s/$.?#].[^\s]*$").unwrap());

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClipboardItem {
    pub id: i64,
    pub hash: String,
    pub content_type: ContentType,
    pub content_value: Option<String>,
    pub preview: Option<String>,
    pub content_size_bytes: i64,
    pub source_app_name: Option<String>,
    pub first_copied_at: DateTime<Utc>,
    pub last_copied_at: DateTime<Utc>,
    pub times_copied: i32,
    pub is_pinned: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ContentType {
    Text,
    Image,
    Color,
    Link,
    File,
}

impl ContentType {
    pub fn from_str(s: &str) -> Result<Self, AppError> {
        match s {
            "text" => Ok(ContentType::Text),
            "image" => Ok(ContentType::Image),
            "color" => Ok(ContentType::Color),
            "link" => Ok(ContentType::Link),
            "file" => Ok(ContentType::File),
            _ => Err(AppError::ClipboardHistory("Invalid content type".into())),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ContentType::Text => "text",
            ContentType::Image => "image",
            ContentType::Color => "color",
            ContentType::Link => "link",
            ContentType::File => "file",
        }
    }
}