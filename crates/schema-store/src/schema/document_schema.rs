use crate::Accessor;
use config::TomlVersion;
use indexmap::IndexMap;

use super::ValueSchema;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct DocumentSchema {
    pub(crate) toml_version: Option<TomlVersion>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub schema_url: Option<url::Url>,
    pub properties: IndexMap<Accessor, ValueSchema>,
    pub definitions: ahash::HashMap<String, ValueSchema>,
}

impl DocumentSchema {
    pub fn toml_version(&self) -> Option<TomlVersion> {
        self.toml_version.inspect(|version| {
            tracing::debug!("use schema TOML version: {version}");
        })
    }
}
