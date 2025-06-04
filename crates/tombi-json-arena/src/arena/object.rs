use super::{StrId, ValueId};
use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectArena {
    data: Vec<IndexMap<StrId, ValueId>>,
}

impl ObjectArena {
    pub fn new() -> Self {
        ObjectArena { data: Vec::new() }
    }

    pub fn insert(&mut self, object: IndexMap<StrId, ValueId>) -> ObjectId {
        let id = ObjectId(self.data.len());
        self.data.push(object);
        id
    }

    pub fn get(&self, id: ObjectId) -> Option<&IndexMap<StrId, ValueId>> {
        self.data.get(id.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectId(pub usize);
