use crate::ai::{AiImageChat, AiImageChatError, ImagePath};

use async_trait::async_trait;
use base64::encode;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug)]
struct ApiResponse {
    choices: Vec<Choice>,
    created: u64,
    id: String,
    model: String,
    object: String,
    usage: Usage,
}

#[derive(Deserialize, Serialize, Debug)]
struct Choice {
    finish_details: HashMap<String, String>,
    index: u32,
    message: Message,
}

#[derive(Deserialize, Serialize, Debug)]
struct Message {
    content: String,
    role: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Usage {
    completion_tokens: u32,
    prompt_tokens: u32,
    total_tokens: u32,
}

// Function to encode the image
fn encode_image(image_path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(image_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(encode(buffer))
}

pub struct ChatGpt4v<'a> {
    pub api_key: &'a str,
}

#[async_trait]
impl AiImageChat for ChatGpt4v<'_> {
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

        let image_url = match image_path {
            ImagePath::Url(url) => url.to_string(),
            ImagePath::File(path) => {
                let base64_image =
                    encode_image(path).map_err(|e| AiImageChatError::BadImagePath(e))?;
                format!("data:image/jpeg;base64,{}", base64_image)
            }
        };

        let payload = json!({
            "model": "gpt-4-vision-preview",
            "messages": [
                {
                    "role": "user",
                    "content": [
                        {
                            "type": "text",
                            "text": question
                        },
                        {
                            "type": "image_url",
                            "image_url": {
                                "url": image_url,
                            }
                        }
                    ]
                }
            ],
            "max_tokens": 300
        });

        let client = Client::new();
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .headers(headers)
            .json(&payload)
            .send()
            .await
            .map_err(|e| AiImageChatError::RequestFailed(Box::new(e)))?;

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
