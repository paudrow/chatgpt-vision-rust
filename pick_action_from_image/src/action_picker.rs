use crate::ai::{AiImageChat, AiImageChatError, ImagePath};
use askama::Template;

#[derive(Clone)]
pub struct Action<'a> {
    pub id: &'a str,
    pub description: &'a str,
}

#[derive(Template, Clone)]
#[template(path = "pick_action_from_image_prompt.txt")]
struct PickActionsFromImagePrompt<'a> {
    context: &'a str,
    actions: &'a Vec<Action<'a>>,
}

pub async fn pick_action_from_image<'a, T: AiImageChat>(
    ai: &T,
    context: &str,
    actions: &'a Vec<Action<'a>>,
    image_path: &ImagePath<'a>,
) -> Result<String, AiImageChatError> {
    let prompt = PickActionsFromImagePrompt { context, actions }
        .render()
        .map_err(|e| AiImageChatError::FailedToGeneratePrompt(Box::new(e)))?;
    let response = ai.ask_about_image(&prompt, image_path).await?;
    Ok(response)
}
