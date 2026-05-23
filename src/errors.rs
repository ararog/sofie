use thiserror::Error;

/// Error types for the Sophia server
#[derive(Error, Debug, PartialEq, Clone)]
pub enum SofieError {
    /// Error when the server fails to start
    #[error("Failed to start server: {0}")]
    ServerStart(String),
}
