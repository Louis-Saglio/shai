use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShaiError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("Environment error: {0}")]
    EnvError(String),

    #[error("Execution error: {0}")]
    ExecutionError(String),

    #[error("Input error: {0}")]
    InputError(String),
}

pub type Result<T> = std::result::Result<T, ShaiError>;
