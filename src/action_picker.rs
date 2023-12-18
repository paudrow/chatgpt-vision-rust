use crate::ai::{AiImageChat, AiImageChatError, ImagePath};
use askama::Template;

/// An action that can be taken.
#[derive(Clone)]
pub struct Action<'a> {
    /// The ID of the action.
    pub id: &'a str,
    /// The description of the action.
    pub description: &'a str,
}

/// Pick an action from an image.
#[derive(Template, Clone)]
#[template(path = "pick_action_from_image_prompt.txt")]
struct PickActionsFromImagePrompt<'a> {
    /// The context provided to the AI.
    context: &'a str,
    /// The actions that can be taken.
    actions: &'a Vec<Action<'a>>,
}

/// Pick an action from an image and provided context.
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

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use url::Url;

    struct TestAI {}

    #[async_trait]
    impl AiImageChat for TestAI {
        async fn ask_about_image(
            &self,
            _question: &str,
            _image_path: &ImagePath,
        ) -> Result<String, AiImageChatError> {
            Ok("answer".to_string())
        }
    }

    #[tokio::test]
    async fn test_pick_action_from_image() {
        let url = Url::parse("https://www.petlandflorida.com/wp-content/uploads/2022/04/shutterstock_1290320698-1-scaled.jpg").unwrap();
        let image_path = ImagePath::Url(&url);
        let context = "What would make them happiest? They haven't eaten."; // Your question
        let actions = vec![
            Action {
                id: "give bone",
                description: "Give a delicious bone to chew on",
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

        let test_ai = TestAI {};
        let response = pick_action_from_image(&test_ai, &context, &actions, &image_path)
            .await
            .unwrap();
        assert_eq!(response, "answer");
    }
}
