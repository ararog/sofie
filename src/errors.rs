use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum SofieError {
    #[error("Failed to start server: {0}")]
    ServerStart(String),
}
