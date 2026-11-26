use base64::Engine;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Embedded image/media asset stored as base64 in the project file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssetItem {
    pub id: String,
    pub name: String,
    pub mime_type: String,
    pub data_base64: String,
    pub width: u32,
    pub height: u32,
}

impl AssetItem {
    /// Creates an asset item from raw image bytes and detects its dimensions when possible.
    pub fn from_bytes(name: impl Into<String>, mime_type: impl Into<String>, bytes: &[u8]) -> Self {
        let name = name.into();
        let mime_type = mime_type.into();
        let (width, height) = image::load_from_memory(bytes)
            .map(|img| (img.width(), img.height()))
            .unwrap_or((800, 600));
        let data_base64 = base64::engine::general_purpose::STANDARD.encode(bytes);
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            mime_type,
            data_base64,
            width,
            height,
        }
    }

    /// Returns a `data:<mime>;base64,...` URI for inline HTML/Google Sites embeds.
    pub fn data_uri(&self) -> String {
        format!("data:{};base64,{}", self.mime_type, self.data_base64)
    }

    /// Decodes the base64 payload back to raw bytes.
    pub fn decoded_bytes(&self) -> Result<Vec<u8>, base64::DecodeError> {
        base64::engine::general_purpose::STANDARD.decode(&self.data_base64)
    }

    /// Returns a filesystem-safe export filename under `assets/`.
    pub fn sanitized_filename(&self) -> String {
        let short_id: String = self.id.chars().filter(|c| c.is_ascii_alphanumeric()).take(8).collect();
        let clean_name: String = self
            .name
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        if clean_name.is_empty() {
            format!("{}.png", short_id)
        } else {
            format!("{}-{}", short_id, clean_name)
        }
    }
}

/// Infers an image MIME type from a filename extension.
pub fn mime_from_extension(path: &str) -> &'static str {
    let lower = path.to_ascii_lowercase();
    if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        "image/jpeg"
    } else if lower.ends_with(".webp") {
        "image/webp"
    } else if lower.ends_with(".gif") {
        "image/gif"
    } else if lower.ends_with(".svg") {
        "image/svg+xml"
    } else if lower.ends_with(".ico") {
        "image/x-icon"
    } else {
        "image/png"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asset_round_trip_and_sanitization() {
        let bytes = vec![137, 80, 78, 71, 1, 2, 3];
        let asset = AssetItem::from_bytes("My Hero Shot (1).png", "image/png", &bytes);
        assert!(asset.data_uri().starts_with("data:image/png;base64,"));
        assert_eq!(asset.decoded_bytes().unwrap(), bytes);
        let safe = asset.sanitized_filename();
        assert!(!safe.contains(' '));
        assert!(safe.ends_with("My_Hero_Shot__1_.png"));
    }
}
