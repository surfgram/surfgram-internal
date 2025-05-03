use crate::error::TelegramError;
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

#[derive(Clone)]
pub struct TelegramClient {
    client: Client,
    base_url: String,
}

impl TelegramClient {
    pub fn new(token: String) -> Result<Self, TelegramError> {
        Ok(Self {
            client: Client::builder()
                .timeout(Duration::from_secs(60))
                .connect_timeout(Duration::from_secs(60))
                .build()?,
            base_url: format!("https://api.telegram.org/bot{}/", token),
        })
    }

    pub async fn call_api(&self, method: &str, params: Value) -> Result<Value, TelegramError> {
        let url = format!("{}{}", self.base_url, method);
        let response = self.client.post(&url).json(&params).send().await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await?;
            return Err(TelegramError::HttpError { status, body });
        }

        let api_response: Value = response.json().await?;

        match api_response.get("ok").and_then(|v| v.as_bool()) {
            Some(true) => Ok(api_response),
            Some(false) => {
                let description = api_response
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown error");
                Err(TelegramError::ApiError(description.to_string()))
            }
            None => Err(TelegramError::ApiError(
                "Invalid API response format".into(),
            )),
        }
    }
}
