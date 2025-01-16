use super::Schema;
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
                                    Self::new_single(type_str, object)
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
            if let serde_json::Value::String(ref_str) = ref_value {
                return Some(Referable::Ref(ref_str.clone()));
            }
        }

        ValueSchema::new(object).map(Referable::Resolved)
    }

    pub fn value_type(&self) -> crate::ValueType {
        match self {
            Referable::Ref(_) => crate::ValueType::Any,
            Referable::Resolved(schema) => schema.value_type(),
        }
    }

    pub fn resolve<'a>(
        &'a mut self,
        definitions: &SchemaDefinitions,
    ) -> Result<&'a ValueSchema, crate::Error> {
        match self {
            Referable::Ref(ref_str) => {
                {
                    if let Some(definition_schema) = definitions.get(ref_str) {
                        *self = definition_schema.clone();
                    } else {
                        Err(crate::Error::DefinitionNotFound {
                            definition_ref: ref_str.clone(),
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

impl Schema for ValueSchema {
    fn find_schema_candidates(
        &self,
        accessors: &[Accessor],
        definitions: &SchemaDefinitions,
    ) -> (Vec<ValueSchema>, Vec<crate::Error>) {
        if accessors.is_empty() {
            (vec![self.clone()], Vec::new())
        } else {
            match self {
                Self::Table(table) => table.find_schema_candidates(accessors, definitions),
                Self::Array(array) => array.find_schema_candidates(accessors, definitions),
                _ => (vec![self.clone()], Vec::new()),
            }
        }
    }
}
