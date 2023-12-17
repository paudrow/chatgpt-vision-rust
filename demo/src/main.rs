use pick_action_from_image::models::ChatGpt4v;
use pick_action_from_image::{pick_action_from_image, Action, ImagePath};

use dotenv::dotenv;
use std::env;
use url::Url;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("API key not found");

    let url = Url::parse("https://www.petlandflorida.com/wp-content/uploads/2022/04/shutterstock_1290320698-1-scaled.jpg").unwrap();
    let image_path = ImagePath::Url(&url);
    let context = "What would make them happiest? They haven't eaten."; // Your question
    let actions = vec![
        Action {
            id: "give bone",
            description: "Give a delicious bone to chew on",
        },
        Action {
            id: "give toy",
            description: "Give a fun toy to play with",
        },
        Action {
            id: "give belly rub",
            description: "Give a belly rub",
        },
        Action {
            id: "none",
            description: "Don't do anything",
        },
    ];

    println!("Question: {}", context);
    let chat_gpt4v = ChatGpt4v { api_key: &api_key };
    let response = pick_action_from_image(&chat_gpt4v, &context, &actions, &image_path)
        .await
        .unwrap();
    println!("Picked action: {}", response);
}
