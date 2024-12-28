pub mod format;
mod lint;
mod schema;
mod types;

pub use format::FormatOptions;
pub use lint::LintOptions;
pub use schema::SchemaOptions;
use std::path::PathBuf;
pub use toml_version::TomlVersion;
pub use types::*;

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
    #[cfg_attr(feature = "jsonschema", schemars(default = "TomlVersion::default"))]
    pub toml_version: Option<TomlVersion>,

    /// # Formatter options.
    pub format: Option<FormatOptions>,

    /// # Linter options.
    pub lint: Option<LintOptions>,

    /// # Schema options array.
    pub schemas: Option<Vec<SchemaOptions>>,
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

/// Load the config from the current directory.
#[cfg(feature = "serde")]
pub fn load_with_path() -> (Config, Option<PathBuf>) {
    const CONFIG_FILENAME: &str = "tombi.toml";
    const PYPROJECT_FILENAME: &str = "pyproject.toml";

    let mut current_dir = std::env::current_dir().unwrap();
    loop {
        let config_path = current_dir.join(CONFIG_FILENAME);
        if config_path.exists() {
            tracing::debug!("\"{}\" found at {:?}", CONFIG_FILENAME, &config_path);

            let Ok(config_str) = std::fs::read_to_string(&config_path) else {
                tracing::error!("Failed to read {:?}", &config_path);
                std::process::exit(1);
            };
            let Ok(config) = toml::from_str::<Config>(&config_str) else {
                tracing::error!("Failed to parse {:?}", &config_path);
                std::process::exit(1);
            };
            return (config, Some(config_path));
        }

        let pyproject_toml_path = current_dir.join(PYPROJECT_FILENAME);
        if pyproject_toml_path.exists() {
            tracing::debug!(
                "\"{}\" found at {:?}",
                PYPROJECT_FILENAME,
                pyproject_toml_path
            );

            let Ok(pyproject_toml_str) = std::fs::read_to_string(&pyproject_toml_path) else {
                tracing::error!("Failed to read {:?}", &pyproject_toml_path);
                std::process::exit(1);
            };
            let Ok(config) = toml::from_str::<PyProjectToml>(&pyproject_toml_str) else {
                tracing::error!("Failed to parse {:?}", &config_path);
                std::process::exit(1);
            };
            if let Some(Tool { tombi: Some(tombi) }) = config.tool {
                return (tombi, Some(pyproject_toml_path));
            } else {
                tracing::debug!("No [tool.tombi] found in {:?}", &config_path);
                continue;
            }
        }

        if !current_dir.pop() {
            break;
        }
    }

    tracing::debug!("No config file found.");
    tracing::debug!("Using default config.");

    (Config::default(), None)
}

#[cfg(feature = "serde")]
pub fn load() -> Config {
    let (config, _) = load_with_path();
    config
}
