use indexmap::map::Entry;
use indexmap::IndexMap;

use crate::Key;
use crate::Value;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Table {
    value: IndexMap<Key, Value>,
}

impl Table {
    pub fn entry(&mut self, key: Key) -> Entry<'_, Key, Value> {
        self.value.entry(key)
    }

    pub fn merge(mut self, other: Self) -> Self {
        for (key, value) in other.value {
            self.value.insert(key, value);
        }
        self
    }
}
