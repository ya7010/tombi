use super::OneOrMany;

/// Generic value that can be either single or multiple
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct SchemaCatalogPath(String);

impl SchemaCatalogPath {
    #[inline]
    pub fn value(&self) -> &str {
        self.0.as_str()
    }
}

impl Default for SchemaCatalogPath {
    fn default() -> Self {
        SchemaCatalogPath("https://www.schemastore.org/api/json/catalog.json".to_string())
    }
}

impl Default for OneOrMany<SchemaCatalogPath> {
    fn default() -> Self {
        Self::One(SchemaCatalogPath::default())
    }
}

impl TryInto<url::Url> for &SchemaCatalogPath {
    type Error = url::ParseError;

    fn try_into(self) -> Result<url::Url, Self::Error> {
        match self.0.parse() {
            Ok(url) => Ok(url),
            Err(_) => format!("file://{}", self.0).parse(),
        }
    }
}
