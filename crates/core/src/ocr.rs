use anyhow::{Context, Result};
use image::DynamicImage;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tracing::{debug, info};
use xtract_config::Config;

use crate::{encode_image_to_base64, load_image};

/// OCR client for communicating with DeepSeek-OCR API
pub struct OcrClient {
    client: reqwest::blocking::Client,
    config: Config,
}

#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: usize,
    temperature: f32,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: Vec<ContentPart>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
enum ContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrlData },
}

#[derive(Debug, Serialize)]
struct ImageUrlData {
    url: String,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

impl OcrClient {
    pub fn new(config: Config) -> Self {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .expect("Failed to create HTTP client - check network configuration");
        
        Self { client, config }
    }

    /// Extract text from an image file
    pub fn extract_from_image(&self, image_path: &Path, prompt: Option<&str>) -> Result<String> {
        info!("Extracting text from image: {:?}", image_path);
        
        // Load and encode image
        let img = load_image(image_path)?;
        let base64_img = encode_image_to_base64(&img)?;
        
        self.extract_from_base64(&base64_img, prompt)
    }

    /// Extract text from a base64-encoded image
    pub fn extract_from_base64(&self, base64_img: &str, prompt: Option<&str>) -> Result<String> {
        // Note: Images are encoded as PNG in encode_image_to_base64, regardless of original format
        let data_url = format!("data:image/png;base64,{}", base64_img);
        
        let default_prompt = if self.config.extraction.output_format == "markdown" {
            "<|grounding|>Convert this document to markdown, preserving layout and structure."
        } else {
            "<|grounding|>Extract all text from this document."
        };
        
        let user_prompt = prompt.unwrap_or(default_prompt);
        
        let request = ChatCompletionRequest {
            model: self.config.ocr.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: vec![
                    ContentPart::ImageUrl {
                        image_url: ImageUrlData { url: data_url },
                    },
                    ContentPart::Text {
                        text: user_prompt.to_string(),
                    },
                ],
            }],
            max_tokens: self.config.ocr.max_tokens,
            temperature: self.config.ocr.temperature,
        };

        debug!("Sending OCR request to: {}", self.config.ocr.api_endpoint);
        
        let endpoint = format!("{}/chat/completions", self.config.ocr.api_endpoint);
        let response = self
            .client
            .post(&endpoint)
            .json(&request)
            .send()
            .context("Failed to send request to OCR API")?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().unwrap_or_else(|_| "Failed to read error response body".to_string());
            anyhow::bail!("OCR API request failed with status {}: {}", status, error_text);
        }

        let response_data: ChatCompletionResponse = response
            .json()
            .context("Failed to parse OCR API response")?;

        let extracted_text = response_data
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .context("No response from OCR API")?;

        info!("Successfully extracted {} characters", extracted_text.len());
        Ok(extracted_text)
    }

    /// Extract text from dynamic image
    pub fn extract_from_dynamic_image(&self, img: &DynamicImage, prompt: Option<&str>) -> Result<String> {
        let base64_img = encode_image_to_base64(img)?;
        self.extract_from_base64(&base64_img, prompt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ocr_client_creation() {
        let config = Config::default();
        let client = OcrClient::new(config);
        assert_eq!(client.config.ocr.model, "deepseek-ocr");
    }
}
