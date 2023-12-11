use base64::encode;
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::env;
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
fn encode_image(image_path: &str) -> String {
    let mut file = File::open(image_path).expect("File not found");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read the file");
    encode(buffer)
}

async fn ask_openai(
    api_key: &str,
    question: &str,
    image_path: &str,
) -> Result<ApiResponse, reqwest::Error> {
    let base64_image = encode_image(image_path);

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );

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
                            "url": format!("data:image/jpeg;base64,{}", base64_image)
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
        .await?;

    response.json().await
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("API key not found");

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("assets/audrow.jpeg");
    let image_path = path.display().to_string();

    let question = "What is in this image?"; // Your question

    match ask_openai(&api_key, question, &image_path).await {
        Ok(parsed_response) => {
            parsed_response.choices.iter().for_each(|choice| {
                println!("{}", choice.message.content);
            });
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
