use anyhow::{Context, Result};
use std::path::Path;
use tracing::info;
use xtract_config::Config;

use crate::{ExtractedDocument, DocumentMetadata, OcrClient};

/// Main document extractor that orchestrates OCR and text extraction
pub struct DocumentExtractor {
    ocr_client: OcrClient,
    config: Config,
}

impl DocumentExtractor {
    pub fn new(config: Config) -> Self {
        let ocr_client = OcrClient::new(config.clone());
        Self { ocr_client, config }
    }

    /// Extract text and metadata from a document image
    pub fn extract(&self, image_path: &Path, prompt: Option<&str>) -> Result<ExtractedDocument> {
        info!("Starting document extraction for: {:?}", image_path);
        
        let source = image_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Perform OCR extraction
        let text = self.ocr_client.extract_from_image(image_path, prompt)
            .context("Failed to extract text from image")?;

        let document = ExtractedDocument {
            text,
            format: self.config.extraction.output_format.clone(),
            metadata: DocumentMetadata {
                source,
                page_count: Some(1),
                confidence: None,
            },
        };

        info!("Document extraction completed successfully");
        Ok(document)
    }

    /// Extract from multiple images (batch processing)
    pub fn extract_batch(&self, image_paths: &[&Path], prompt: Option<&str>) -> Result<Vec<ExtractedDocument>> {
        info!("Starting batch extraction for {} images", image_paths.len());
        
        let mut documents = Vec::new();
        
        for (i, path) in image_paths.iter().enumerate() {
            info!("Processing image {}/{}: {:?}", i + 1, image_paths.len(), path);
            match self.extract(path, prompt) {
                Ok(doc) => documents.push(doc),
                Err(e) => {
                    tracing::warn!("Failed to extract from {:?}: {}", path, e);
                    // Continue with other images
                }
            }
        }
        
        info!("Batch extraction completed: {} successful", documents.len());
        Ok(documents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extractor_creation() {
        let config = Config::default();
        let extractor = DocumentExtractor::new(config);
        assert_eq!(extractor.config.extraction.output_format, "markdown");
    }
}
