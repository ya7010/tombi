pub struct SchemaContext<'a> {
    pub toml_version: config::TomlVersion,
    pub root_schema: Option<&'a crate::DocumentSchema>,
    pub sub_schema_url_map: Option<&'a crate::SubSchemaUrlMap>,
    pub store: &'a crate::SchemaStore,
}
