mod error;
pub mod format;
mod lint;
mod schema;
mod server;
mod types;

pub use error::Error;
pub use format::FormatOptions;
pub use lint::LintOptions;
pub use schema::SchemaCatalogItem;
pub use schema::SchemaOptions;
use std::path::PathBuf;
pub use toml_version::TomlVersion;
pub use types::*;

const CONFIG_FILENAME: &str = "tombi.toml";
const PYPROJECT_FILENAME: &str = "pyproject.toml";
pub const SUPPORTED_CONFIG_FILENAMES: [&str; 2] = [CONFIG_FILENAME, PYPROJECT_FILENAME];

/// # Tombi
///
/// **Tombi** (鳶) is a toolkit for TOML; providing a formatter/linter and language server.
/// See the [GitHub repository](https://github.com/tombi-toml/tombi) for more information.
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "jsonschema", schemars(extend("x-tombi-toml-version" = TomlVersion::V1_1_0_Preview)))]
pub struct Config {
    /// # TOML version.
    ///
    /// TOML version to use if not specified in the schema.
    #[cfg_attr(feature = "jsonschema", schemars(default = "TomlVersion::default"))]
    pub toml_version: Option<TomlVersion>,

    /// # Formatter options.
    pub format: Option<FormatOptions>,

    /// # Linter options.
    pub lint: Option<LintOptions>,

    /// # Language server options.
    #[cfg_attr(feature = "jsonschema", schemars(skip))]
    pub server: Option<server::ServerOptions>,

    /// # Schema options.
    pub schema: Option<SchemaOptions>,

    /// # Schema catalog items.
    pub schemas: Option<Vec<SchemaCatalogItem>>,
}

#[doc(hidden)]
#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
struct PyProjectToml {
    tool: Option<Tool>,
}

#[doc(hidden)]
#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
struct Tool {
    tombi: Option<Config>,
}

#[cfg(feature = "serde")]
impl Config {
    fn try_from_path<P: AsRef<std::path::Path>>(
        config_path: P,
    ) -> Result<Option<Self>, crate::Error> {
        let config_path = config_path.as_ref();

        if !config_path.exists() {
            return Err(crate::Error::ConfigFileNotFound {
                config_path: config_path.to_owned(),
            });
        }

        let Ok(config_str) = std::fs::read_to_string(config_path) else {
            return Err(crate::Error::ConfigFileReadFailed {
                config_path: config_path.to_owned(),
            });
        };

        match config_path.file_name().and_then(|name| name.to_str()) {
            Some(CONFIG_FILENAME) => toml::from_str::<Config>(&config_str)
                .map_err(|_| crate::Error::ConfigFileParseFailed {
                    config_path: config_path.to_owned(),
                })
                .map(Some),
            Some(PYPROJECT_FILENAME) => {
                let Ok(pyproject_toml) = toml::from_str::<PyProjectToml>(&config_str) else {
                    return Err(crate::Error::ConfigFileParseFailed {
                        config_path: config_path.to_owned(),
                    });
                };
                if let Some(Tool { tombi: Some(tombi) }) = pyproject_toml.tool {
                    Ok(Some(tombi))
                } else {
                    Ok(None)
                }
            }
            _ => Err(crate::Error::ConfigFileUnsupported {
                config_path: config_path.to_owned(),
            }),
        }
    }

    pub fn try_from_url(config_url: url::Url) -> Result<Option<Self>, crate::Error> {
        match config_url.scheme() {
            "file" => {
                let config_path = config_url
                    .to_file_path()
                    .map_err(|_| crate::Error::ConfigUrlParseFailed { config_url })?;
                Self::try_from_path(config_path)
            }
            _ => Err(crate::Error::ConfigUrlUnsupported { config_url }),
        }
    }
}

/// Load the config from the current directory.
#[cfg(feature = "serde")]
pub fn load_with_path() -> Result<(Config, Option<PathBuf>), crate::Error> {
    let mut current_dir = std::env::current_dir().unwrap();
    loop {
        let config_path = current_dir.join(CONFIG_FILENAME);
        if config_path.exists() {
            tracing::debug!("\"{}\" found at {:?}", CONFIG_FILENAME, &config_path);

            let Some(config) = Config::try_from_path(&config_path)? else {
                unreachable!("tombi.toml must be Some(config)")
            };

            let config_dirpath = match config_path.parent() {
                Some(dir) => dir.to_owned(),
                None => current_dir,
            };

            return Ok((config, Some(config_dirpath)));
        }

        let pyproject_toml_path = current_dir.join(PYPROJECT_FILENAME);
        if pyproject_toml_path.exists() {
            tracing::debug!(
                "\"{}\" found at {:?}",
                PYPROJECT_FILENAME,
                pyproject_toml_path
            );

            let Some(config) = Config::try_from_path(&pyproject_toml_path)? else {
                tracing::debug!("No [tool.tombi] found in {:?}", &config_path);
                continue;
            };
            return Ok((config, Some(pyproject_toml_path)));
        }

        if !current_dir.pop() {
            break;
        }
    }

    tracing::debug!("config file not found, use default config");

    Ok((Config::default(), None))
}

#[cfg(feature = "serde")]
pub fn load() -> Result<Config, crate::Error> {
    let (config, _) = load_with_path()?;
    Ok(config)
}
