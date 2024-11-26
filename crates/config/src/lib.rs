pub mod format;
pub mod lint;
mod toml_version;
pub use toml_version::TomlVersion;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
#[derive(Debug, Default, Clone)]
pub struct Config {
    pub toml_version: TomlVersion,
    pub format: format::Options,
}
