use crate::ai::{AiImageChat, AiImageChatError, ImagePath};

pub struct Action<'a> {
    pub id: &'a str,
    pub description: &'a str,
}

pub async fn pick_action<T: AiImageChat>(
    ai: &T,
    context: &str,
    actions: &Vec<Action<'_>>,
    image_path: &ImagePath<'_>,
) -> Result<String, AiImageChatError> {
    let instructions = "Given the context and instructions below, respond with only the single action id that best matches the image. Don't reply with anything else.";

    let mut full_prompt = String::new();
    full_prompt.push_str(instructions);
    full_prompt.push_str("\n\n");
    full_prompt.push_str(format!("Context: {}\n\n", context).as_str());

    full_prompt.push_str("Possible Actions:\n\n");
    actions.iter().for_each(|action| {
        full_prompt.push_str(format!("{}: {}\n\n", action.id, action.description).as_str());
    });

    let response = ai.ask_about_image(&full_prompt, image_path).await?;
    Ok(response)
}
