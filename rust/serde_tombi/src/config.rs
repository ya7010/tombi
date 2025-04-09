use config::{Config, CONFIG_FILENAME, PYPROJECT_FILENAME};

#[doc(hidden)]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
struct PyProjectToml {
    tool: Option<Tool>,
}

#[doc(hidden)]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
struct Tool {
    tombi: Option<Config>,
}

pub fn try_from_path<P: AsRef<std::path::Path>>(
    config_path: P,
) -> Result<Option<Config>, config::Error> {
    let config_path = config_path.as_ref();

    if !config_path.exists() {
        return Err(config::Error::ConfigFileNotFound {
            config_path: config_path.to_owned(),
        });
    }

    let Ok(config_str) = std::fs::read_to_string(config_path) else {
        return Err(config::Error::ConfigFileReadFailed {
            config_path: config_path.to_owned(),
        });
    };

    match config_path.file_name().and_then(|name| name.to_str()) {
        Some(CONFIG_FILENAME) => crate::from_str::<Config>(&config_str)
            .map_err(|_| config::Error::ConfigFileParseFailed {
                config_path: config_path.to_owned(),
            })
            .map(Some),
        Some(PYPROJECT_FILENAME) => {
            let Ok(pyproject_toml) = crate::from_str::<PyProjectToml>(&config_str) else {
                return Err(config::Error::ConfigFileParseFailed {
                    config_path: config_path.to_owned(),
                });
            };
            if let Some(Tool { tombi: Some(tombi) }) = pyproject_toml.tool {
                Ok(Some(tombi))
            } else {
                Ok(None)
            }
        }
        _ => Err(config::Error::ConfigFileUnsupported {
            config_path: config_path.to_owned(),
        }),
    }
}

pub fn try_from_url(config_url: url::Url) -> Result<Option<Config>, config::Error> {
    match config_url.scheme() {
        "file" => {
            let config_path = config_url
                .to_file_path()
                .map_err(|_| config::Error::ConfigUrlParseFailed { config_url })?;
            try_from_path(config_path)
        }
        _ => Err(config::Error::ConfigUrlUnsupported { config_url }),
    }
}

/// Load the config from the current directory.
pub fn load_with_path() -> Result<(Config, Option<std::path::PathBuf>), config::Error> {
    let mut current_dir = std::env::current_dir().unwrap();
    loop {
        let config_path = current_dir.join(CONFIG_FILENAME);
        if config_path.exists() {
            tracing::debug!("\"{}\" found at {:?}", CONFIG_FILENAME, &config_path);

            let Some(config) = try_from_path(&config_path)? else {
                unreachable!("tombi.toml should always be parsed successfully.");
            };

            return Ok((config, Some(config_path)));
        }

        let pyproject_toml_path = current_dir.join(PYPROJECT_FILENAME);
        if pyproject_toml_path.exists() {
            tracing::debug!(
                "\"{}\" found at {:?}",
                PYPROJECT_FILENAME,
                pyproject_toml_path
            );

            match try_from_path(&pyproject_toml_path)? {
                Some(config) => return Ok((config, Some(pyproject_toml_path))),
                None => {
                    tracing::debug!("No [tool.tombi] found in {:?}", &config_path);
                }
            };
        }

        if !current_dir.pop() {
            break;
        }
    }

    tracing::debug!("config file not found, use default config");

    Ok((Config::default(), None))
}

pub fn load() -> Result<Config, config::Error> {
    let (config, _) = load_with_path()?;
    Ok(config)
}
