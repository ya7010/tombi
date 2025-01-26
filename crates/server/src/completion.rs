mod document;
mod hint;
mod value;

use config::TomlVersion;
pub use hint::CompletionHint;
use schema_store::{Accessor, SchemaDefinitions, Schemas, ValueSchema};
use tower_lsp::lsp_types::{MarkupContent, MarkupKind, Url};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CompletionPriority {
    DefaultValue = 0,
    #[default]
    Normal = 1,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CompletionContent {
    pub label: String,
    pub kind: Option<tower_lsp::lsp_types::CompletionItemKind>,
    pub priority: CompletionPriority,
    pub detail: Option<String>,
    pub documentation: Option<tower_lsp::lsp_types::Documentation>,
}

impl CompletionContent {
    pub fn new_default_value(label: String) -> Self {
        Self {
            label,
            kind: Some(tower_lsp::lsp_types::CompletionItemKind::VALUE),
            priority: CompletionPriority::DefaultValue,
            detail: Some("default".to_string()),
            documentation: None,
        }
    }
}

impl Into<tower_lsp::lsp_types::CompletionItem> for CompletionContent {
    fn into(self) -> tower_lsp::lsp_types::CompletionItem {
        let sorted_text = format!("{}_{}", (self.priority as usize), &self.label);
        tower_lsp::lsp_types::CompletionItem {
            label: self.label,
            kind: Some(
                self.kind
                    .unwrap_or(tower_lsp::lsp_types::CompletionItemKind::VALUE),
            ),
            detail: self.detail,
            documentation: self.documentation,
            sort_text: Some(sorted_text),
            ..Default::default()
        }
    }
}

pub trait FindCompletionContents {
    fn find_completion_contents(
        &self,
        accessors: &Vec<Accessor>,
        value_schema: &ValueSchema,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> Vec<CompletionContent>;
}

pub trait CompletionCandidate {
    fn title(
        &self,
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> Option<String>;

    fn description(
        &self,
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> Option<String>;

    fn detail(
        &self,
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> Option<String> {
        self.title(definitions, completion_hint).map(|cow| cow)
    }

    fn documentation(
        &self,
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> Option<tower_lsp::lsp_types::Documentation> {
        self.description(definitions, completion_hint)
            .map(|description| {
                tower_lsp::lsp_types::Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: description,
                })
            })
    }
}

impl<T: CompositeSchema> CompletionCandidate for T {
    fn title(
        &self,
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> Option<String> {
        match self.title().as_deref() {
            Some(title) => Some(title.into()),
            None => {
                let mut candidates = ahash::AHashSet::new();
                {
                    if let Ok(mut schemas) = self.schemas().write() {
                        for schema in schemas.iter_mut() {
                            if let Ok(schema) = schema.resolve(definitions) {
                                if matches!(schema, ValueSchema::Null) {
                                    continue;
                                }
                                if let Some(candidate) =
                                    CompletionCandidate::title(schema, definitions, completion_hint)
                                {
                                    candidates.insert(candidate.to_string());
                                }
                            }
                        }
                    }
                }
                if candidates.len() == 1 {
                    return candidates.into_iter().next();
                }
                None
            }
        }
    }

    fn description(
        &self,
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> Option<String> {
        match self.description().as_deref() {
            Some(description) => Some(description.into()),
            None => {
                let mut candidates = ahash::AHashSet::new();
                {
                    if let Ok(mut schemas) = self.schemas().write() {
                        for schema in schemas.iter_mut() {
                            if let Ok(schema) = schema.resolve(definitions) {
                                if matches!(schema, ValueSchema::Null) {
                                    continue;
                                }
                                if let Some(candidate) = CompletionCandidate::description(
                                    schema,
                                    definitions,
                                    completion_hint,
                                ) {
                                    candidates.insert(candidate.to_string());
                                }
                            }
                        }
                    }
                }
                if candidates.len() == 1 {
                    return candidates.into_iter().next();
                }
                None
            }
        }
    }
}

pub trait CompositeSchema {
    fn title(&self) -> Option<String>;
    fn description(&self) -> Option<String>;
    fn schemas(&self) -> &Schemas;
}

fn find_one_of_completion_items<T>(
    value: &T,
    accessors: &Vec<Accessor>,
    one_of_schema: &schema_store::OneOfSchema,
    toml_version: TomlVersion,
    position: text::Position,
    keys: &[document_tree::Key],
    schema_url: Option<&Url>,
    definitions: &SchemaDefinitions,
    completion_hint: Option<CompletionHint>,
) -> Vec<CompletionContent>
where
    T: FindCompletionContents,
{
    let mut completion_items = Vec::new();

    if let Ok(mut schemas) = one_of_schema.schemas.write() {
        for schema in schemas.iter_mut() {
            if let Ok(schema) = schema.resolve(definitions) {
                let schema_completions = value.find_completion_contents(
                    accessors,
                    schema,
                    toml_version,
                    position,
                    keys,
                    schema_url,
                    definitions,
                    completion_hint,
                );

                completion_items.extend(schema_completions);
            }
        }
    }

    for completion_item in completion_items.iter_mut() {
        if completion_item.detail.is_none() {
            completion_item.detail = one_of_schema.detail(definitions, completion_hint);
        }
        if completion_item.documentation.is_none() {
            completion_item.documentation =
                one_of_schema.documentation(definitions, completion_hint);
        }
    }

    if let Some(default) = &one_of_schema.default {
        completion_items.push(CompletionContent::new_default_value(default.to_string()));
    }

    completion_items
}

fn find_any_of_completion_items<T>(
    value: &T,
    accessors: &Vec<Accessor>,
    any_of_schema: &schema_store::AnyOfSchema,
    toml_version: TomlVersion,
    position: text::Position,
    keys: &[document_tree::Key],
    schema_url: Option<&Url>,
    definitions: &SchemaDefinitions,
    completion_hint: Option<CompletionHint>,
) -> Vec<CompletionContent>
where
    T: FindCompletionContents,
{
    let mut completion_items = Vec::new();

    if let Ok(mut schemas) = any_of_schema.schemas.write() {
        for schema in schemas.iter_mut() {
            if let Ok(schema) = schema.resolve(definitions) {
                let schema_completions = value.find_completion_contents(
                    accessors,
                    schema,
                    toml_version,
                    position,
                    keys,
                    schema_url,
                    definitions,
                    completion_hint,
                );

                completion_items.extend(schema_completions);
            }
        }
    }

    for completion_item in completion_items.iter_mut() {
        if completion_item.detail.is_none() {
            completion_item.detail = any_of_schema.detail(definitions, completion_hint);
        }
        if completion_item.documentation.is_none() {
            completion_item.documentation =
                any_of_schema.documentation(definitions, completion_hint);
        }
    }

    if let Some(default) = &any_of_schema.default {
        completion_items.push(CompletionContent::new_default_value(default.to_string()));
    }

    completion_items
}

fn find_all_if_completion_items<T>(
    value: &T,
    accessors: &Vec<Accessor>,
    all_of_schema: &schema_store::AllOfSchema,
    toml_version: TomlVersion,
    position: text::Position,
    keys: &[document_tree::Key],
    schema_url: Option<&Url>,
    definitions: &SchemaDefinitions,
    completion_hint: Option<CompletionHint>,
) -> Vec<CompletionContent>
where
    T: FindCompletionContents,
{
    let mut completion_items = Vec::new();

    if let Ok(mut schemas) = all_of_schema.schemas.write() {
        for schema in schemas.iter_mut() {
            if let Ok(schema) = schema.resolve(definitions) {
                let schema_completions = value.find_completion_contents(
                    accessors,
                    schema,
                    toml_version,
                    position,
                    keys,
                    schema_url,
                    definitions,
                    completion_hint,
                );

                completion_items.extend(schema_completions);
            }
        }
    }

    for completion_item in completion_items.iter_mut() {
        if completion_item.detail.is_none() {
            completion_item.detail = all_of_schema.detail(definitions, completion_hint);
        }
        if completion_item.documentation.is_none() {
            completion_item.documentation =
                all_of_schema.documentation(definitions, completion_hint);
        }
    }

    if let Some(default) = &all_of_schema.default {
        completion_items.push(CompletionContent::new_default_value(default.to_string()));
    }

    completion_items
}
