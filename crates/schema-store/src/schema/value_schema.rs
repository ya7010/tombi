use super::FindSchemaCandidates;
use super::{
    AllOfSchema, AnyOfSchema, ArraySchema, BooleanSchema, FloatSchema, IntegerSchema,
    LocalDateSchema, LocalDateTimeSchema, LocalTimeSchema, OffsetDateTimeSchema, OneOfSchema,
    StringSchema, TableSchema,
};
use crate::Referable;
use crate::{Accessor, SchemaDefinitions};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub enum ValueSchema {
    Null,
    Boolean(BooleanSchema),
    Integer(IntegerSchema),
    Float(FloatSchema),
    String(StringSchema),
    LocalDate(LocalDateSchema),
    LocalDateTime(LocalDateTimeSchema),
    LocalTime(LocalTimeSchema),
    OffsetDateTime(OffsetDateTimeSchema),
    Array(ArraySchema),
    Table(TableSchema),
    OneOf(OneOfSchema),
    AnyOf(AnyOfSchema),
    AllOf(AllOfSchema),
}

impl ValueSchema {
    pub fn new(object: &serde_json::Map<String, serde_json::Value>) -> Option<Self> {
        match object.get("type") {
            Some(serde_json::Value::String(type_str)) => return Self::new_single(type_str, object),
            Some(serde_json::Value::Array(types)) => {
                return Some(Self::OneOf(OneOfSchema {
                    schemas: Arc::new(RwLock::new(
                        types
                            .iter()
                            .filter_map(|type_value| {
                                if let serde_json::Value::String(type_str) = type_value {
                                    Self::new_single(&type_str, object)
                                } else {
                                    None
                                }
                            })
                            .map(Referable::Resolved)
                            .collect(),
                    )),
                    ..Default::default()
                }));
            }
            _ => {}
        }

        if object.get("oneOf").is_some() {
            return Some(ValueSchema::OneOf(OneOfSchema::new(object)));
        }
        if object.get("anyOf").is_some() {
            return Some(ValueSchema::AnyOf(AnyOfSchema::new(object)));
        }
        if object.get("allOf").is_some() {
            return Some(ValueSchema::AllOf(AllOfSchema::new(object)));
        }

        None
    }

    fn new_single(
        type_str: &str,
        object: &serde_json::Map<String, serde_json::Value>,
    ) -> Option<Self> {
        match type_str {
            "null" => Some(ValueSchema::Null),
            "boolean" => Some(ValueSchema::Boolean(BooleanSchema::new(object))),
            "integer" => Some(ValueSchema::Integer(IntegerSchema::new(object))),
            "number" => Some(ValueSchema::Float(FloatSchema::new(object))),
            "string" => {
                if let Some(serde_json::Value::String(format_str)) = object.get("format") {
                    // See: https://json-schema.org/understanding-json-schema/reference/type#built-in-formats
                    match format_str.as_str() {
                        "date" => {
                            Some(ValueSchema::LocalDate(LocalDateSchema::new(object)));
                        }
                        "date-time" => {
                            Some(ValueSchema::OneOf(OneOfSchema {
                                schemas: Arc::new(RwLock::new(
                                    [
                                        ValueSchema::LocalDateTime(LocalDateTimeSchema::new(
                                            object,
                                        )),
                                        ValueSchema::OffsetDateTime(OffsetDateTimeSchema::new(
                                            object,
                                        )),
                                    ]
                                    .map(Referable::Resolved)
                                    .to_vec(),
                                )),
                                ..Default::default()
                            }));
                        }
                        "time" => {
                            Some(ValueSchema::LocalTime(LocalTimeSchema::new(object)));
                        }
                        _ => {}
                    }
                }
                Some(ValueSchema::String(StringSchema::new(object)))
            }
            "array" => Some(ValueSchema::Array(ArraySchema::new(object))),
            "object" => Some(ValueSchema::Table(TableSchema::new(object))),
            _ => None,
        }
    }

    pub fn value_type(&self) -> crate::ValueType {
        match self {
            Self::Null => crate::ValueType::Null,
            Self::Boolean(boolean) => boolean.value_type(),
            Self::Integer(integer) => integer.value_type(),
            Self::Float(float) => float.value_type(),
            Self::String(string) => string.value_type(),
            Self::LocalDate(local_date) => local_date.value_type(),
            Self::LocalDateTime(local_date_time) => local_date_time.value_type(),
            Self::LocalTime(local_time) => local_time.value_type(),
            Self::OffsetDateTime(offset_date_time) => offset_date_time.value_type(),
            Self::Array(array) => array.value_type(),
            Self::Table(table) => table.value_type(),
            Self::OneOf(one_of) => one_of.value_type(),
            Self::AnyOf(any_of) => any_of.value_type(),
            Self::AllOf(all_of) => all_of.value_type(),
        }
    }

    pub fn title(&self) -> Option<&str> {
        match self {
            ValueSchema::Null => None,
            ValueSchema::Boolean(schema) => schema.title.as_deref(),
            ValueSchema::Integer(schema) => schema.title.as_deref(),
            ValueSchema::Float(schema) => schema.title.as_deref(),
            ValueSchema::String(schema) => schema.title.as_deref(),
            ValueSchema::LocalDate(schema) => schema.title.as_deref(),
            ValueSchema::LocalDateTime(schema) => schema.title.as_deref(),
            ValueSchema::LocalTime(schema) => schema.title.as_deref(),
            ValueSchema::OffsetDateTime(schema) => schema.title.as_deref(),
            ValueSchema::Array(schema) => schema.title.as_deref(),
            ValueSchema::Table(schema) => schema.title.as_deref(),
            ValueSchema::OneOf(schema) => schema.title.as_deref(),
            ValueSchema::AnyOf(schema) => schema.title.as_deref(),
            ValueSchema::AllOf(schema) => schema.title.as_deref(),
        }
    }

    pub fn set_title(&mut self, title: Option<String>) {
        match self {
            ValueSchema::Null => {}
            ValueSchema::Boolean(schema) => schema.title = title,
            ValueSchema::Integer(schema) => schema.title = title,
            ValueSchema::Float(schema) => schema.title = title,
            ValueSchema::String(schema) => schema.title = title,
            ValueSchema::LocalDate(schema) => schema.title = title,
            ValueSchema::LocalDateTime(schema) => schema.title = title,
            ValueSchema::LocalTime(schema) => schema.title = title,
            ValueSchema::OffsetDateTime(schema) => schema.title = title,
            ValueSchema::Array(schema) => schema.title = title,
            ValueSchema::Table(schema) => schema.title = title,
            ValueSchema::OneOf(schema) => schema.title = title,
            ValueSchema::AnyOf(schema) => schema.title = title,
            ValueSchema::AllOf(schema) => schema.title = title,
        }
    }

    pub fn description(&self) -> Option<&str> {
        match self {
            ValueSchema::Null => None,
            ValueSchema::Boolean(schema) => schema.description.as_deref(),
            ValueSchema::Integer(schema) => schema.description.as_deref(),
            ValueSchema::Float(schema) => schema.description.as_deref(),
            ValueSchema::String(schema) => schema.description.as_deref(),
            ValueSchema::LocalDate(schema) => schema.description.as_deref(),
            ValueSchema::LocalDateTime(schema) => schema.description.as_deref(),
            ValueSchema::LocalTime(schema) => schema.description.as_deref(),
            ValueSchema::OffsetDateTime(schema) => schema.description.as_deref(),
            ValueSchema::Array(schema) => schema.description.as_deref(),
            ValueSchema::Table(schema) => schema.description.as_deref(),
            ValueSchema::OneOf(schema) => schema.description.as_deref(),
            ValueSchema::AnyOf(schema) => schema.description.as_deref(),
            ValueSchema::AllOf(schema) => schema.description.as_deref(),
        }
    }

    pub fn set_description(&mut self, description: Option<String>) {
        match self {
            ValueSchema::Null => {}
            ValueSchema::Boolean(schema) => schema.description = description,
            ValueSchema::Integer(schema) => schema.description = description,
            ValueSchema::Float(schema) => schema.description = description,
            ValueSchema::String(schema) => schema.description = description,
            ValueSchema::LocalDate(schema) => schema.description = description,
            ValueSchema::LocalDateTime(schema) => schema.description = description,
            ValueSchema::LocalTime(schema) => schema.description = description,
            ValueSchema::OffsetDateTime(schema) => schema.description = description,
            ValueSchema::Array(schema) => schema.description = description,
            ValueSchema::Table(schema) => schema.description = description,
            ValueSchema::OneOf(schema) => schema.description = description,
            ValueSchema::AnyOf(schema) => schema.description = description,
            ValueSchema::AllOf(schema) => schema.description = description,
        }
    }

    pub fn match_schemas<T: Fn(&ValueSchema) -> bool>(&self, condition: &T) -> Vec<ValueSchema> {
        let mut matched_schemas = Vec::new();
        match self {
            ValueSchema::OneOf(OneOfSchema { schemas, .. })
            | ValueSchema::AnyOf(AnyOfSchema { schemas, .. })
            | ValueSchema::AllOf(AllOfSchema { schemas, .. }) => {
                if let Ok(mut schemas) = schemas.write() {
                    for schema in schemas.iter_mut() {
                        if let Ok(schema) = schema.resolve(&SchemaDefinitions::default()) {
                            matched_schemas.extend(schema.match_schemas(condition))
                        }
                    }
                }
            }
            _ => {
                if condition(self) {
                    matched_schemas.push(self.clone());
                }
            }
        };

        matched_schemas
    }

    pub fn is_match<T: Fn(&ValueSchema) -> bool>(&self, condition: &T) -> bool {
        match self {
            ValueSchema::OneOf(one_of) => {
                let Ok(mut schemas) = one_of.schemas.write() else {
                    return false;
                };
                schemas.iter_mut().any(|schema| {
                    if let Ok(schema) = schema.resolve(&SchemaDefinitions::default()) {
                        schema.is_match(condition)
                    } else {
                        false
                    }
                })
            }
            ValueSchema::AnyOf(any_of) => {
                let Ok(mut schemas) = any_of.schemas.write() else {
                    return false;
                };
                schemas.iter_mut().any(|schema| {
                    if let Ok(schema) = schema.resolve(&SchemaDefinitions::default()) {
                        schema.is_match(condition)
                    } else {
                        false
                    }
                })
            }
            ValueSchema::AllOf(all_of) => {
                let Ok(mut schemas) = all_of.schemas.write() else {
                    return false;
                };
                schemas.iter_mut().all(|schema| {
                    if let Ok(schema) = schema.resolve(&SchemaDefinitions::default()) {
                        schema.is_match(condition)
                    } else {
                        false
                    }
                })
            }
            _ => condition(self),
        }
    }
}

impl Referable<ValueSchema> {
    pub fn new(object: &serde_json::Map<String, serde_json::Value>) -> Option<Self> {
        if let Some(ref_value) = object.get("$ref") {
            if let serde_json::Value::String(ref_string) = ref_value {
                return Some(Referable::Ref {
                    reference: ref_string.clone(),
                    title: object
                        .get("title")
                        .and_then(|title| title.as_str().map(|s| s.to_string())),
                    description: object
                        .get("description")
                        .and_then(|description| description.as_str().map(|s| s.to_string())),
                });
            }
        }

        ValueSchema::new(object).map(Referable::Resolved)
    }

    pub fn value_type(&self) -> crate::ValueType {
        match self {
            Referable::Resolved(schema) => schema.value_type(),
            Referable::Ref { .. } => unreachable!("unreachable ref value_tyle."),
        }
    }

    pub fn resolve<'a>(
        &'a mut self,
        definitions: &SchemaDefinitions,
    ) -> Result<&'a ValueSchema, crate::Error> {
        match self {
            Referable::Ref {
                reference,
                title,
                description,
            } => {
                {
                    if let Some(definition_schema) = definitions.get(reference) {
                        let mut referable_schema = definition_schema.to_owned();
                        if let Referable::Resolved(ref mut schema) = &mut referable_schema {
                            if title.is_some() || description.is_some() {
                                schema.set_title(title.to_owned());
                                schema.set_description(description.to_owned());
                            }
                        }

                        *self = referable_schema;
                    } else {
                        Err(crate::Error::DefinitionNotFound {
                            definition_ref: reference.clone(),
                        })?;
                    }
                }
                self.resolve(definitions)
            }
            Referable::Resolved(resolved) => {
                match resolved {
                    ValueSchema::OneOf(OneOfSchema { schemas, .. })
                    | ValueSchema::AnyOf(AnyOfSchema { schemas, .. })
                    | ValueSchema::AllOf(AllOfSchema { schemas, .. }) => {
                        if let Ok(mut schemas) = schemas.write() {
                            for schema in schemas.iter_mut() {
                                schema.resolve(definitions)?;
                            }
                        }
                    }
                    _ => {}
                }
                Ok(resolved)
            }
        }
    }
}

impl FindSchemaCandidates for ValueSchema {
    fn find_schema_candidates(
        &self,
        accessors: &[Accessor],
        definitions: &SchemaDefinitions,
    ) -> (Vec<ValueSchema>, Vec<crate::Error>) {
        match self {
            Self::Table(table) => {
                if accessors.is_empty() {
                    (vec![self.clone()], Vec::new())
                } else {
                    table.find_schema_candidates(accessors, definitions)
                }
            }
            Self::Array(array) => {
                if accessors.is_empty() {
                    (vec![self.clone()], Vec::new())
                } else {
                    array.find_schema_candidates(accessors, definitions)
                }
            }
            Self::OneOf(OneOfSchema { schemas, .. })
            | Self::AnyOf(AnyOfSchema { schemas, .. })
            | Self::AllOf(AllOfSchema { schemas, .. }) => {
                let mut candidates = Vec::new();
                let mut errors = Vec::new();

                if let Ok(mut schemas) = schemas.write() {
                    for schema in schemas.iter_mut() {
                        let Ok(schema) = schema.resolve(definitions) else {
                            continue;
                        };
                        let (schema_candidates, schema_errors) =
                            schema.find_schema_candidates(accessors, definitions);
                        candidates.extend(schema_candidates);
                        errors.extend(schema_errors);
                    }
                }
                (candidates, errors)
            }
            _ => (vec![self.clone()], Vec::new()),
        }
    }
}
