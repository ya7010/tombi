use crate::{
    json_schema::{Referable, SchemaComposition},
    Value,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ObjectSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub schema: Option<Referable<SchemaComposition>>,
    pub default: Option<Value>,
    pub enumerated_values: Vec<Value>,
}

#[cfg(test)]
mod test {
    use schemars::{json_schema, Schema};

    #[allow(unused)]
    fn ref_schema() -> Schema {
        json_schema!({
            "type": "object",
            "properties": {
                "toml-version": {
                    "$ref": "#/$defs/TomlVersion"
                }
            },
            "$defs": {
                "TomlVersion": {
                    "title": "TOML version.",
                    "type": "string",
                    "enum": ["v1.0.0", "v1.1.0-preview"]
                }
            }
        })
    }
}
