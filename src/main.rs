mod ai;
mod chatgpt_4v;

use crate::ai::{AiImageChat, ImagePath};
use crate::chatgpt_4v::ChatGpt4v;

use dotenv::dotenv;
use std::env;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("API key not found");

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("assets/audrow.jpeg");

    let question = "What is in this image?"; // Your question

    let chat_gpt4v = ChatGpt4v { api_key: &api_key };
    let response = chat_gpt4v
        .ask_about_image(question, &ImagePath::File(&path))
        .await
        .unwrap();
    println!("{:?}", response)
}
