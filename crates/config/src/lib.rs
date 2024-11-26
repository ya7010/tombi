pub mod format;
pub mod lint;
mod toml_version;
pub use toml_version::TomlVersion;

pub struct Config {
    pub toml_version: TomlVersion,
    pub format: format::Options,
}
