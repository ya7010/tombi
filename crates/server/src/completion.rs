mod document;
mod hint;
mod table;
mod value;

use config::TomlVersion;
pub use hint::CompletionHint;
use schema_store::{Accessor, SchemaDefinitions, Schemas, ValueSchema};
use tower_lsp::lsp_types::{MarkupContent, MarkupKind, Url};

pub trait FindCompletionItems {
    fn find_completion_items(
        &self,
        accessors: &[Accessor],
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> (
        Vec<tower_lsp::lsp_types::CompletionItem>,
        Vec<schema_store::Error>,
    );
}

pub trait FindCompletionItems2 {
    fn find_completion_items2(
        &self,
        accessors: &Vec<Accessor>,
        value_schema: &ValueSchema,
        toml_version: TomlVersion,
        position: text::Position,
        keys: &[document_tree::Key],
        schema_url: Option<&Url>,
        definitions: &SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> (
        Vec<tower_lsp::lsp_types::CompletionItem>,
        Vec<schema_store::Error>,
    );
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

impl<T: CompositeSchema> FindCompletionItems for T {
    fn find_completion_items(
        &self,
        accessors: &[schema_store::Accessor],
        definitions: &schema_store::SchemaDefinitions,
        completion_hint: Option<CompletionHint>,
    ) -> (
        Vec<tower_lsp::lsp_types::CompletionItem>,
        Vec<schema_store::Error>,
    ) {
        let mut completion_items = Vec::new();
        let mut errors = Vec::new();

        if let Ok(mut schemas) = self.schemas().write() {
            for value_schema in schemas.iter_mut() {
                if let Ok(schema) = value_schema.resolve(definitions) {
                    let (schema_completion_items, schema_errors) =
                        schema.find_completion_items(accessors, definitions, completion_hint);

                    completion_items.extend(schema_completion_items);
                    errors.extend(schema_errors);
                } else {
                    errors.push(schema_store::Error::SchemaLockError);
                }
            }
        }

        for completion_item in completion_items.iter_mut() {
            if completion_item.detail.is_none() {
                completion_item.detail = self.detail(definitions, completion_hint);
            }
            if completion_item.documentation.is_none() {
                completion_item.documentation = self.documentation(definitions, completion_hint);
            }
        }

        (completion_items, errors)
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
) -> (
    Vec<tower_lsp::lsp_types::CompletionItem>,
    Vec<schema_store::Error>,
)
where
    T: FindCompletionItems2,
{
    let mut completion_items = Vec::new();
    let mut errors = Vec::new();

    if let Ok(mut schemas) = one_of_schema.schemas.write() {
        for schema in schemas.iter_mut() {
            if let Ok(schema) = schema.resolve(definitions) {
                let (schema_completions, schema_errors) = value.find_completion_items2(
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
                errors.extend(schema_errors);
            } else {
                errors.push(schema_store::Error::SchemaLockError);
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

    (completion_items, errors)
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
) -> (
    Vec<tower_lsp::lsp_types::CompletionItem>,
    Vec<schema_store::Error>,
)
where
    T: FindCompletionItems2,
{
    let mut completion_items = Vec::new();
    let mut errors = Vec::new();

    if let Ok(mut schemas) = any_of_schema.schemas.write() {
        for schema in schemas.iter_mut() {
            if let Ok(schema) = schema.resolve(definitions) {
                let (schema_completions, schema_errors) = value.find_completion_items2(
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
                errors.extend(schema_errors);
            } else {
                errors.push(schema_store::Error::SchemaLockError);
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

    (completion_items, errors)
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
) -> (
    Vec<tower_lsp::lsp_types::CompletionItem>,
    Vec<schema_store::Error>,
)
where
    T: FindCompletionItems2,
{
    let mut completion_items = Vec::new();
    let mut errors = Vec::new();

    if let Ok(mut schemas) = all_of_schema.schemas.write() {
        for schema in schemas.iter_mut() {
            if let Ok(schema) = schema.resolve(definitions) {
                let (schema_completions, schema_errors) = value.find_completion_items2(
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
                errors.extend(schema_errors);
            } else {
                errors.push(schema_store::Error::SchemaLockError);
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

    (completion_items, errors)
}
