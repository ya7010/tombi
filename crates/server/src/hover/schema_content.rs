use schema_store::ValueSchema;

mod array;
mod boolean;
mod float;
mod integer;
mod local_date;
mod local_date_time;
mod local_time;
mod offset_date_time;
mod string;
mod table;

pub trait SchemaHoverContent {
    fn schema_content(&self) -> Option<String>;
}

impl SchemaHoverContent for ValueSchema {
    fn schema_content(&self) -> Option<String> {
        match self {
            ValueSchema::Boolean(schema) => schema.schema_content(),
            ValueSchema::Integer(schema) => schema.schema_content(),
            ValueSchema::Float(schema) => schema.schema_content(),
            ValueSchema::String(schema) => schema.schema_content(),
            ValueSchema::OffsetDateTime(schema) => schema.schema_content(),
            ValueSchema::LocalDateTime(schema) => schema.schema_content(),
            ValueSchema::LocalDate(schema) => schema.schema_content(),
            ValueSchema::LocalTime(schema) => schema.schema_content(),
            ValueSchema::Array(schema) => schema.schema_content(),
            ValueSchema::Table(schema) => schema.schema_content(),
            ValueSchema::Null
            | ValueSchema::OneOf(_)
            | ValueSchema::AnyOf(_)
            | ValueSchema::AllOf(_) => None,
        }
    }
}
