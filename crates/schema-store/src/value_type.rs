#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum ValueType {
    #[default]
    Any,
    Null,
    Boolean,
    Integer,
    Float,
    String,
    OffsetDateTime,
    LocalDateTime,
    LocalDate,
    LocalTime,
    Array,
    Table,
    OneOf(Vec<ValueType>),
    AnyOf(Vec<ValueType>),
    AllOf(Vec<ValueType>),
}
