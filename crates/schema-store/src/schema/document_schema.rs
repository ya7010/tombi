use config::TomlVersion;

use crate::Accessor;

use super::object_schema::ObjectSchema;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct DocumentSchema {
    pub toml_version: Option<TomlVersion>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub schema_url: Option<url::Url>,
    pub properties: ahash::HashMap<Accessor, ObjectSchema>,
    pub definitions: ahash::HashMap<String, ObjectSchema>,
}
