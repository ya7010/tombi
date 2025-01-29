mod all_of;
mod any_of;
mod array;
mod boolean;
mod float;
mod integer;
mod local_date;
mod local_date_time;
mod local_time;
mod offset_date_time;
mod one_of;
mod string;
mod table;
mod value;

use config::TomlVersion;
use dashmap::DashMap;
use schema_store::{get_schema_name, Accessor, Accessors, DocumentSchema, ValueSchema, ValueType};
use std::{fmt::Debug, ops::Deref};
use tower_lsp::lsp_types::Url;

pub fn get_hover_content(
    tree: &document_tree::DocumentTree,
    toml_version: TomlVersion,
    position: text::Position,
    keys: &[document_tree::Key],
    document_schema: Option<&DocumentSchema>,
) -> Option<HoverContent> {
    let table = tree.deref();
    match document_schema {
        Some(document_schema) => table.get_hover_content(
            &Vec::with_capacity(0),
            Some(document_schema.value_schema()),
            toml_version,
            position,
            keys,
            Some(&document_schema.schema_url),
            &document_schema.definitions,
        ),
        None => table.get_hover_content(
            &Vec::with_capacity(0),
            None,
            toml_version,
            position,
            keys,
            None,
            &DashMap::new(),
        ),
    }
}

trait GetHoverContent {
    fn get_hover_content(
        &self,
        accessors: &Vec<Accessor>,
        value_schema: Option<&ValueSchema>,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        definitions: &schema_store::SchemaDefinitions,
    ) -> Option<HoverContent>;
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct HoverContent {
    pub title: Option<String>,
    pub description: Option<String>,
    pub accessors: Accessors,
    pub value_type: ValueType,
    pub enumerated_values: Vec<String>,
    pub schema_url: Option<tower_lsp::lsp_types::Url>,
    pub range: Option<text::Range>,
}

impl HoverContent {
    pub fn into_nullable(mut self) -> HoverContent {
        self.value_type = self.value_type.into_nullable();
        self
    }
}

impl std::fmt::Display for HoverContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const SECTION_SEPARATOR: &str = "-----";

        if let Some(title) = &self.title {
            writeln!(f, "#### {}\n", title)?;
        }

        if let Some(description) = &self.description {
            writeln!(f, "{}\n", description)?;
        }

        if self.title.is_some() || self.description.is_some() {
            writeln!(f, "{}\n", SECTION_SEPARATOR)?;
        }

        writeln!(f, "Keys: `{}`\n", self.accessors)?;
        writeln!(f, "Value: `{}`\n", self.value_type)?;

        if !self.enumerated_values.is_empty() {
            writeln!(f, "Allowed Values:\n")?;
            for value in &self.enumerated_values {
                writeln!(f, "- `{}`", value)?;
            }
            writeln!(f)?;
        }

        if let Some(schema_url) = &self.schema_url {
            if let Some(schema_filename) = get_schema_name(schema_url) {
                writeln!(f, "Schema: [{schema_filename}]({schema_url})\n",)?;
            }
        }

        Ok(())
    }
}

impl From<HoverContent> for tower_lsp::lsp_types::Hover {
    fn from(value: HoverContent) -> Self {
        tower_lsp::lsp_types::Hover {
            contents: tower_lsp::lsp_types::HoverContents::Markup(
                tower_lsp::lsp_types::MarkupContent {
                    kind: tower_lsp::lsp_types::MarkupKind::Markdown,
                    value: value.to_string(),
                },
            ),
            range: value.range.map(Into::into),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;
    use tower_lsp::lsp_types::Url;

    #[rstest]
    #[case("https://json.schemastore.org/tombi.schema.json")]
    #[case("file://./folder/tombi.schema.json")]
    #[case("file://./tombi.schema.json")]
    #[case("file://tombi.schema.json")]
    fn url_content(#[case] url: &str) {
        let url = Url::parse(url).unwrap();
        assert_eq!(get_schema_name(&url).unwrap(), "tombi.schema.json");
    }
}
