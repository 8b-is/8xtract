use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine as _};
use image::DynamicImage;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{debug, info};

pub mod ocr;
pub mod extraction;

pub use ocr::OcrClient;
pub use extraction::DocumentExtractor;

/// Represents an extracted document with OCR results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedDocument {
    pub text: String,
    pub format: String,
    pub metadata: DocumentMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub source: String,
    pub page_count: Option<usize>,
    pub confidence: Option<f32>,
}

/// Load an image from a file path
pub fn load_image(path: &Path) -> Result<DynamicImage> {
    debug!("Loading image from: {:?}", path);
    let img = image::open(path)
        .with_context(|| format!("Failed to load image from {:?}", path))?;
    info!("Image loaded successfully: {}x{}", img.width(), img.height());
    Ok(img)
}

/// Encode an image to base64 for API transmission
pub fn encode_image_to_base64(img: &DynamicImage) -> Result<String> {
    let mut buffer = std::io::Cursor::new(Vec::new());
    img.write_to(&mut buffer, image::ImageFormat::Png)
        .context("Failed to encode image to PNG")?;
    let encoded = general_purpose::STANDARD.encode(buffer.into_inner());
    Ok(encoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_metadata() {
        let metadata = DocumentMetadata {
            source: "test.png".to_string(),
            page_count: Some(1),
            confidence: Some(0.95),
        };
        
        assert_eq!(metadata.source, "test.png");
        assert_eq!(metadata.page_count, Some(1));
    }
}
