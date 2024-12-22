#[derive(Debug, Clone, PartialEq)]
pub enum DefaultValue {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}
