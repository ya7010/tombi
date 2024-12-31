use config::TomlVersion;

use crate::Accessor;

use super::object_schema::ObjectSchema;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct DocumentSchema {
    pub(crate) toml_version: Option<TomlVersion>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub schema_url: Option<url::Url>,
    pub properties: ahash::HashMap<Accessor, ObjectSchema>,
    pub definitions: ahash::HashMap<String, ObjectSchema>,
}

impl DocumentSchema {
    pub fn toml_version(&self) -> Option<TomlVersion> {
        self.toml_version.inspect(|version| {
            tracing::debug!("use schema TOML version: {version}");
        })
    }
}
