mod arena;
pub use arena::{StrArena, StrId, ValueArena, ValueId};

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(StrId),
    Array(Array),
    Object(Object),
}

pub type Array = Vec<ValueId>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Object(Vec<(StrId, ValueId)>);

impl Object {
    pub fn new() -> Self {
        Object(Vec::new())
    }

    pub fn insert(&mut self, key: StrId, value: ValueId) {
        if let Some((_, v)) = self.0.iter_mut().find(|(k, _)| *k == key) {
            *v = value;
        } else {
            self.0.push((key, value));
        }
    }

    pub fn get(&self, key: &StrId) -> Option<&ValueId> {
        self.0.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn remove(&mut self, key: &StrId) -> Option<ValueId> {
        if let Some(pos) = self.0.iter().position(|(k, _)| k == key) {
            Some(self.0.remove(pos).1)
        } else {
            None
        }
    }

    pub fn contains_key(&self, key: &StrId) -> bool {
        self.0.iter().any(|(k, _)| k == key)
    }

    pub fn keys(&self) -> impl Iterator<Item = &StrId> {
        self.0.iter().map(|(k, _)| k)
    }

    pub fn values(&self) -> impl Iterator<Item = &ValueId> {
        self.0.iter().map(|(_, v)| v)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&StrId, &ValueId)> {
        self.0.iter().map(|(k, v)| (k, v))
    }
}
