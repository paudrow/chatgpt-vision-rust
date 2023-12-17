use async_trait::async_trait;
use std::error::Error;
use std::path::PathBuf;
use url::Url;

#[derive(Debug)]
pub enum AiImageChatError {
    BadImagePath(Box<dyn Error>),
    RequestFailed(Box<dyn Error>),
    UnableToParseResponse(Box<dyn Error>),
    FailedToGeneratePrompt(Box<dyn Error>),
}

pub enum ImagePath<'a> {
    Url(&'a Url),
    File(&'a PathBuf),
}

#[async_trait]
pub trait AiImageChat {
    async fn ask_about_image(
        &self,
        question: &str,
        image_path: &ImagePath,
    ) -> Result<String, AiImageChatError>;
}
