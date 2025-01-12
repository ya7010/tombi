#[derive(Debug, Clone, PartialEq)]
pub enum Referable<T> {
    Resolved(T),
    Ref(String),
}

impl<T> Referable<T> {
    pub fn resolved(&self) -> Option<&T> {
        match self {
            Self::Resolved(t) => Some(t),
            Self::Ref(_) => None,
        }
    }
}
