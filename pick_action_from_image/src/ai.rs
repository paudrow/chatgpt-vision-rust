use async_trait::async_trait;
use std::error::Error;
use std::path::PathBuf;
use url::Url;

/// An error from the AI.
#[derive(Debug)]
pub enum AiImageChatError {
    /// The image path was invalid.
    BadImagePath(Box<dyn Error>),
    /// The request to the AI failed.
    RequestFailed(Box<dyn Error>),
    /// The response from the AI could not be parsed.
    UnableToParseResponse(Box<dyn Error>),
    /// The prompt could not be generated.j
    FailedToGeneratePrompt(Box<dyn Error>),
}

/// A path to an image.
pub enum ImagePath<'a> {
    /// A URL to an image.
    Url(&'a Url),
    /// A path to a local image file.
    File(&'a PathBuf),
}

/// A trait for AI that can answer questions about images.
#[async_trait]
pub trait AiImageChat {
    /// Ask the AI about the image.
    async fn ask_about_image(
        &self,
        question: &str,
        image_paths: &ImagePath,
    ) -> Result<String, AiImageChatError>;
}
