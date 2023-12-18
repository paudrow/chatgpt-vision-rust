use crate::ai::{AiImageChat, AiImageChatError, ImagePath};

use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

/// The response from the API call.
#[derive(Deserialize, Serialize, Debug)]
struct ApiResponse {
    /// The ID of the API call.
    id: String,
    /// The object of the API call.
    object: String,
    /// The created time of the API call.
    created: u64,
    /// The model of the API call.
    model: String,
    /// The choices of the API call.
    usage: Usage,
    /// The choices of the API call.
    choices: Vec<Choice>,
}

/// The choice from the API call.
#[derive(Deserialize, Serialize, Debug)]
struct Choice {
    /// The finish reason of the message.
    finish_reason: String,
    /// The index of the message.
    index: u32,
    /// The message from the API call.
    message: Message,
}

/// The message from the API call.
#[derive(Deserialize, Serialize, Debug)]
struct Message {
    /// The content of the message.
    content: String,
    /// The role of the message.
    role: String,
}

/// Usage statistics for the API call.
#[derive(Deserialize, Serialize, Debug)]
struct Usage {
    /// The prompt tokens of the API call.
    prompt_tokens: u32,
    /// The completion tokens of the API call.
    completion_tokens: u32,
    /// The total tokens of the API call.
    total_tokens: u32,
}

// Function to encode the image
fn encode_image(image_path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(image_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(general_purpose::STANDARD.encode(buffer))
}

/// ChatGpt4v is a struct that implements the AiImageChat trait.
///
/// This uses the GPT-4 Vision model.
/// You can learn more about the GPT-4 Vision model here: https://platform.openai.com/docs/guides/vision
///
/// To use this, you need an API key.
/// You can learn more about the API key here: https://platform.openai.com/docs/api-reference/authentication
pub struct ChatGpt4v<'a> {
    pub api_key: &'a str,
}

/// Implementation of the AI chat using GPT-4.
#[async_trait]
impl AiImageChat for ChatGpt4v<'_> {
    /// Ask the AI about the image.
    ///
    /// This will return a string with the AI's response.
    async fn ask_about_image(
        &self,
        question: &str,
        image_path: &ImagePath,
    ) -> Result<String, AiImageChatError> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_key)).unwrap(),
        );

        let image_paths = vec![image_path];
        let payload = make_payload(question, &image_paths)?;

        let client = Client::new();
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .headers(headers)
            .json(&payload)
            .send()
            .await
            .map_err(|e| AiImageChatError::RequestFailed(Box::new(e)))?;

        let status = response.status();
        if status.is_client_error() || status.is_server_error() {
            return Err(AiImageChatError::RequestFailed(Box::new(
                std::io::Error::new(std::io::ErrorKind::Other, "Request failed"),
            )));
        }

        let parsed_message: ApiResponse = response
            .json()
            .await
            .map_err(|e| AiImageChatError::UnableToParseResponse(Box::new(e)))?;

        if parsed_message.choices.len() != 1 {
            return Err(AiImageChatError::UnableToParseResponse(Box::new(
                std::io::Error::new(std::io::ErrorKind::Other, "Expected one choice"),
            )));
        }
        let message = parsed_message
            .choices
            .first()
            .map(|c| c.message.content.as_str())
            .ok_or(AiImageChatError::UnableToParseResponse(Box::new(
                std::io::Error::new(std::io::ErrorKind::Other, "Expected one choice"),
            )))?
            .to_string();
        Ok(message)
    }
}

/// Build the payload for the ChatGPT API call.
///
/// This supports multiple images.
fn make_payload(
    text: &str,
    image_paths: &Vec<&ImagePath>,
) -> Result<serde_json::Value, AiImageChatError> {
    let mut content = vec![json!({
        "type": "text",
        "text": text,
    })];
    for image_path in image_paths {
        let image_url = match image_path {
            ImagePath::Url(url) => url.to_string(),
            ImagePath::File(path) => {
                let base64_image =
                    encode_image(path).map_err(|e| AiImageChatError::BadImagePath(e))?;
                format!("data:image/jpeg;base64,{}", base64_image)
            }
        };
        content.push(json!({
            "type": "image_url",
            "image_url": {
                "url": image_url,
            }
        }));
    }

    Ok(json!({
        "model": "gpt-4-vision-preview",
        "messages": [
            {
                "role": "user",
                "content": content,
            }
        ],
        "max_tokens": 300
    }))
}
