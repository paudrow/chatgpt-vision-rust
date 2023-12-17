mod action_picker;
mod ai;
mod chatgpt_4v;

use crate::action_picker::{pick_action_from_image, Action};
use crate::ai::ImagePath;
use crate::chatgpt_4v::ChatGpt4v;

use dotenv::dotenv;
use std::env;
use url::Url;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("API key not found");

    // let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // path.push("assets/audrow.jpeg");
    // let image_path = ImagePath::File(&path);
    let url = Url::parse("https://www.petlandflorida.com/wp-content/uploads/2022/04/shutterstock_1290320698-1-scaled.jpg").unwrap();
    let image_path = ImagePath::Url(&url);

    let context = "What would be best? The dog loves touch."; // Your question

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
        // Action {
        //     id: "give haircut",
        //     description: "Cut off all that head hair",
        // },
        // Action {
        //     id: "shave beard",
        //     description: "Shave off that beard down to the baby face (his wife hates this)",
        // },
        // Action {
        //     id: "give mohawk",
        //     description: "Give a Blink 182 style mohawk",
        // },
        // Action {
        //     id: "give mullet",
        //     description: "Give a country style mullet",
        // },
        // Action {
        //     id: "buy a suit",
        //     description: "Buy a suit and tie, like 007",
        // },
        // Action {
        //     id: "none",
        //     description: "None of the above",
        // }
    ];

    let chat_gpt4v = ChatGpt4v { api_key: &api_key };
    let response = pick_action_from_image(&chat_gpt4v, &context, &actions, &image_path)
        .await
        .unwrap();
    println!("{}", response);
}
