use async_trait::async_trait;
use bytes::Bytes;
use std::fmt::Debug;

#[cfg_attr(feature = "wasm", async_trait(?Send))]
#[cfg_attr(not(feature = "wasm"), async_trait)]
pub trait HttpClient: Debug + Send + Sync {
    async fn get_bytes(&self, url: &str) -> Result<Bytes, String>;
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
    async fn get_bytes(&self, url: &str) -> Result<Bytes, String> {
        let response = self
            .0
            .get(url)
            .send()
            .await
            .map_err(|err| err.to_string())?;

        let bytes = response.bytes().await.map_err(|err| err.to_string())?;
        Ok(bytes)
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
    async fn get_bytes(&self, url: &str) -> Result<Bytes, String> {
        let mut response = gloo_net::http::Request::get(url)
            .send()
            .await
            .map_err(|err| err.to_string())?;

        let bytes = response.binary().await.map_err(|err| err.to_string())?;
        Ok(Bytes::from(bytes))
    }
}
