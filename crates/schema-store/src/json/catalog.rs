use super::JsonSchema;

pub const DEFAULT_CATALOG_URL: &str = "https://www.schemastore.org/api/json/catalog.json";

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Catalog {
    pub schemas: Vec<JsonSchema>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CatalogUrl(url::Url);

impl CatalogUrl {
    #[inline]
    pub fn new(url: url::Url) -> Self {
        Self(url)
    }

    #[inline]
    pub fn parse(s: &str) -> Result<Self, url::ParseError> {
        url::Url::parse(s).map(Self)
    }

    #[inline]
    pub fn from_file_path<P: AsRef<std::path::Path>>(path: P) -> Result<Self, ()> {
        url::Url::from_file_path(&path).map(Self)
    }
}

impl std::ops::Deref for CatalogUrl {
    type Target = url::Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for CatalogUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
