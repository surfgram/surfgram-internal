use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TelegramError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Telegram API error: {0}")]
    ApiError(String),

    #[error("Invalid JSON data: {0}")]
    InvalidJson(#[from] serde_json::Error),

    #[error("HTTP error {status}: {body}")]
    HttpError { status: StatusCode, body: String },
}

impl TelegramError {
    pub fn to_py_err(&self) -> pyo3::PyErr {
        match self {
            TelegramError::RequestFailed(err) => {
                pyo3::exceptions::PyException::new_err(format!("Network error: {}", err))
            }
            TelegramError::ApiError(msg) => {
                pyo3::exceptions::PyException::new_err(format!("Telegram API error: {}", msg))
            }
            TelegramError::InvalidJson(err) => {
                pyo3::exceptions::PyException::new_err(format!("Serialization error: {}", err))
            }
            TelegramError::HttpError { status, body } => {
                pyo3::exceptions::PyException::new_err(format!("HTTP {}: {}", status, body))
            }
        }
    }
}
