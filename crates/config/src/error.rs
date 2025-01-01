use url::Url;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unsupported config url: {config_url}")]
    UrlSchemaUnsupported { config_url: Url },

    #[error("failed to parse config url: {config_url}")]
    UrlSchemaParseFailed { config_url: Url },

    #[error("failed to read {config_path:?}")]
    ConfigFileReadFailed { config_path: std::path::PathBuf },

    #[error("failed to parse {config_path:?}")]
    ConfigFileParseFailed { config_path: std::path::PathBuf },
}
