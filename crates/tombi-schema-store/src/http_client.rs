use async_trait::async_trait;
use bytes::Bytes;
use std::fmt::Debug;

#[derive(Debug, Clone, thiserror::Error)]
pub enum FetchError {
    #[error("{reason}")]
    FetchFailed { reason: String },
    #[error("unexpected status: {status}")]
    StatusNotOk { status: u16 },
    #[error("failed to read body: {reason}")]
    BodyReadFailed { reason: String },
}

#[cfg_attr(feature = "wasm", async_trait(?Send))]
#[cfg_attr(not(feature = "wasm"), async_trait)]
pub trait HttpClient: Debug + Send + Sync {
    async fn get_bytes(&self, url: &str) -> Result<Bytes, FetchError>;
}

#[cfg(not(feature = "wasm"))]
#[derive(Debug)]
pub struct DefaultClient(reqwest::Client);

#[cfg(not(feature = "wasm"))]
impl DefaultClient {
    pub fn new() -> Self {
        Self(reqwest::Client::new())
    }
}

#[cfg(not(feature = "wasm"))]
#[async_trait]
impl HttpClient for DefaultClient {
    async fn get_bytes(&self, url: &str) -> Result<Bytes, FetchError> {
        let response = self
            .0
            .get(url)
            .send()
            .await
            .map_err(|err| FetchError::FetchFailed {
                reason: err.to_string(),
            })?;

        if !response.status().is_success() {
            return Err(FetchError::StatusNotOk {
                status: response.status().as_u16(),
            });
        }

        response
            .bytes()
            .await
            .map_err(|err| FetchError::BodyReadFailed {
                reason: err.to_string(),
            })
    }
}

#[cfg(feature = "wasm")]
#[derive(Debug)]
pub struct DefaultClient;

#[cfg(feature = "wasm")]
impl DefaultClient {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(feature = "wasm")]
#[async_trait(?Send)]
impl HttpClient for DefaultClient {
    async fn get_bytes(&self, url: &str) -> Result<Bytes, FetchError> {
        let mut response = gloo_net::http::Request::get(url)
            .send()
            .await
            .map_err(|err| FetchError::FetchFailed {
                reason: err.to_string(),
            })?;

        let is_success = 200 <= response.status() && response.status() < 300;
        if !is_success {
            return Err(FetchError::StatusNotOk {
                status: response.status(),
            });
        }

        let binary = response
            .binary()
            .await
            .map_err(|e| FetchError::BodyReadFailed {
                reason: e.to_string(),
            })?;

        Ok(Bytes::from(binary))
    }
}
