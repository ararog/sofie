use thiserror::Error;

#[derive(Error, Debug)]
pub enum SophiaError {
    #[error("Failed to start server: {0}")]
    ServerStart(String),
}