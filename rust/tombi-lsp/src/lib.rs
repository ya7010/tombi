pub mod app;
mod document;
mod server;
mod toml;
mod version;

use serde::de::DeserializeOwned;

pub use version::version;

pub fn from_json<T: DeserializeOwned>(
    what: &'static str,
    json: &serde_json::Value,
) -> anyhow::Result<T> {
    serde_json::from_value(json.clone())
        .map_err(|e| anyhow::format_err!("Failed to deserialize {what}: {e}; {json}"))
}
