use pick_action_from_image::models::ChatGpt4v;
use pick_action_from_image::{pick_action_from_image, Action, ImagePath};

use dotenv::dotenv;
use std::env;
// use std::path::PathBuf; // if you want to load the image from a local file
use url::Url;

#[tokio::main]
async fn main() {
    // Provide a URL or file path to an image
    let url = Url::parse("https://www.petlandflorida.com/wp-content/uploads/2022/04/shutterstock_1290320698-1-scaled.jpg").unwrap();
    let image_path = ImagePath::Url(&url);

    // You can also use a file path
    // let path = PathBuf::from("assets/my_image.jpeg");
    // let image_path = ImagePath::File(&path);

    // Add context for how the choice should be made
    let context = "What would make them happiest? They haven't eaten."; // Your question

    // Add actions to choose from. Their description helps provide additional context.
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

    // Create an AI client
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("API key not found");
    let chat_gpt4v = ChatGpt4v { api_key: &api_key };

    //  Ask the AI about the image
    let picked_action = pick_action_from_image(&chat_gpt4v, &context, &actions, &image_path)
        .await
        .unwrap();
    println!("Picked action: {}", picked_action);
}
