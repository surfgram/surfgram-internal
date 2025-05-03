use pyo3::{exceptions::PyException, prelude::*};
use pyo3_asyncio::tokio::future_into_py;
use serde_json::{from_str, to_string, Value};
use std::sync::Arc;

mod client;
mod error;

use crate::client::TelegramClient;
use crate::error::TelegramError;

#[pyclass]
pub struct NativeClient {
    inner: Arc<TelegramClient>,
}

#[pymethods]
impl NativeClient {
    #[new]
    fn new(token: String) -> PyResult<Self> {
        TelegramClient::new(token)
            .map(|client| Self {
                inner: Arc::new(client),
            })
            .map_err(|e| e.to_py_err())
    }

    fn send_request<'py>(
        &self,
        py: Python<'py>,
        method: String,
        params: String,
    ) -> PyResult<&'py PyAny> {
        let inner: Arc<TelegramClient> = Arc::clone(&self.inner);

        let params: Value = from_str(&params)
            .map_err(|e| PyException::new_err(format!("Invalid JSON params: {}", e)))?;

        future_into_py(py, async move {
            inner
                .call_api(&method, params)
                .await
                .and_then(|response| to_string(&response).map_err(TelegramError::InvalidJson))
                .map_err(|e: TelegramError| e.to_py_err())
        })
    }
}

#[pymodule]
fn surfgram_internal(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<NativeClient>()?;
    Ok(())
}
