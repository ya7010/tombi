#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
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
}
