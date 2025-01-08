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

use all_of::AllOfSchema;
use any_of::AnyOfSchema;
pub use array::ArraySchema;
pub use boolean::BooleanSchema;
pub use float::FloatSchema;
pub use integer::IntegerSchema;
pub use local_date::LocalDateSchema;
pub use local_date_time::LocalDateTimeSchema;
pub use local_time::LocalTimeSchema;
pub use offset_date_time::OffsetDateTimeSchema;
use one_of::OneOfSchema;
pub use string::StringSchema;
pub use table::TableSchema;

use super::{referable::Referable, DocumentSchema};

#[derive(Debug, Clone, PartialEq)]
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
        if let Some(serde_json::Value::String(type_str)) = object.get("type") {
            match type_str.as_str() {
                "null" => return Some(ValueSchema::Null),
                "boolean" => return Some(ValueSchema::Boolean(BooleanSchema::new(object))),
                "integer" => return Some(ValueSchema::Integer(IntegerSchema::new(object))),
                "number" => return Some(ValueSchema::Float(FloatSchema::new(object))),
                "string" => {
                    if let Some(serde_json::Value::String(format_str)) = object.get("format") {
                        match format_str.as_str() {
                            "date" => {
                                return Some(ValueSchema::LocalDate(LocalDateSchema::new(object)));
                            }
                            "date-time" => {
                                return Some(ValueSchema::OneOf(OneOfSchema {
                                    schemas: [
                                        ValueSchema::LocalDateTime(LocalDateTimeSchema::new(
                                            object,
                                        )),
                                        ValueSchema::OffsetDateTime(OffsetDateTimeSchema::new(
                                            object,
                                        )),
                                    ]
                                    .map(Referable::Resolved)
                                    .to_vec(),
                                    ..Default::default()
                                }));
                            }
                            "time" => {
                                return Some(ValueSchema::LocalTime(LocalTimeSchema::new(object)));
                            }
                            _ => {}
                        }
                    }
                    return Some(ValueSchema::String(StringSchema::new(object)));
                }
                "array" => return Some(ValueSchema::Array(ArraySchema::new(object))),
                "object" => return Some(ValueSchema::Table(TableSchema::new(object))),
                _ => {}
            };
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

    pub fn resolve<'a>(
        &'a mut self,
        document_schema: &'a DocumentSchema,
    ) -> Result<&'a ValueSchema, crate::Error> {
        match self {
            Referable::Ref(ref_str) => {
                let definitions = match document_schema.definitions.read() {
                    Ok(guard) => guard,
                    Err(_) => {
                        return Err(crate::Error::SchemaLockError {
                            ref_string: ref_str.clone(),
                        });
                    }
                };

                if let Some(definition_schema) = definitions.get(ref_str) {
                    *self = definition_schema.clone();
                    self.resolve(document_schema)
                } else {
                    Err(crate::Error::DefinitionNotFound {
                        definition_ref: ref_str.clone(),
                    })
                }
            }
            Referable::Resolved(resolved) => {
                match resolved {
                    ValueSchema::OneOf(OneOfSchema { schemas, .. })
                    | ValueSchema::AnyOf(AnyOfSchema { schemas, .. })
                    | ValueSchema::AllOf(AllOfSchema { schemas, .. }) => {
                        for schema in schemas {
                            schema.resolve(document_schema)?;
                        }
                    }
                    _ => {}
                }
                Ok(resolved)
            }
        }
    }
}
