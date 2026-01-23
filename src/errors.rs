use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum SophiaError {
    #[error("Failed to start server: {0}")]
    ServerStart(String),
}
