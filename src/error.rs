use thiserror::Error;

/// Extension error types
#[derive(Error, Debug)]
pub enum DatenoError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("URL parsing error: {0}")]
    Url(#[from] url::ParseError),

    #[error("Invalid filter format: {0}")]
    InvalidFilter(String),

    #[error("Invalid limit: {0}. Must be between 1 and 100")]
    InvalidLimit(i64),

    #[error("Missing required parameter: {0}")]
    MissingParameter(String),

    #[error("API error: {0}")]
    Api(String),

    #[error("DuckDB error: {0}")]
    DuckDB(String),

    #[error("Async runtime error: {0}")]
    Runtime(String),
}

pub type Result<T> = std::result::Result<T, DatenoError>;
