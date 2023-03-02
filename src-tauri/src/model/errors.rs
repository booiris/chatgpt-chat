use serde_with::SerializeDisplay;
use thiserror::Error;

pub type ResultWarp<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, SerializeDisplay)]
pub enum Error {
    #[error("Any Error: {0}")]
    Any(String),
    #[error("Env Error: {0}")]
    EnvError(#[from] std::env::VarError),
    #[error("OpenApi Error: {0}")]
    OpenApiError(#[from] openai_api::Error),
    #[error("CompletionArgsBuilder Error: {0}")]
    CompletionArgsBuilderError(#[from] openai_api::api::CompletionArgsBuilderError),
    #[error("ChatArgsBuilderError Error: {0}")]
    ChatArgsBuilderError(#[from] openai_api::api::ChatArgsBuilderError),
}
