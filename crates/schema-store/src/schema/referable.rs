#[derive(Debug, Clone, PartialEq)]
pub enum Referable<T> {
    Resolved(T),
    Ref(String),
}
