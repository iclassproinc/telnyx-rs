use thiserror::Error;

/// Error type for all Tenlyx error types
#[derive(Error, Debug)]
pub enum TelnyxError {
    /// HTTP request failed for timeout or transport issues
    #[error("HTTP requestr failed: {0}")]
    Http(#[from] reqwest::Error),
    /// API error response was received
    #[error("API error (status {status}): {message}")]
    Api {
        status: u16,
        message: String
    },
    /// Failed to parse (deserialize) API response
    #[error("Failed to parse response: {0}")]
    Parse(#[from] serde_json::Error),
    /// Client configuration error
    #[error("Configuration error: {0}")]
    Config(String)
}