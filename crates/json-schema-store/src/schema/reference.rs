use crate::Store;

use super::DocumentSchema;

#[derive(Debug, Clone, PartialEq)]
pub enum Referable<T> {
    Ref(String),
    Schema(Result<T, ()>),
}

impl<T> Referable<T> {
    pub fn take_schema(&mut self, _document: &DocumentSchema, _store: &Store) -> Result<&T, &()> {
        match self {
            Referable::Schema(s) => s.as_ref(),
            Referable::Ref(_) => Err(&()),
        }
    }
}

impl<T> From<T> for Referable<T> {
    fn from(schema: T) -> Self {
        Referable::Schema(Ok(schema))
    }
}
