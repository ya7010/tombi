// arena/value.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueId(pub(crate) usize);

#[derive(Default)]
pub struct ValueArena {
    pub(crate) values: Vec<crate::Value>,
}

impl ValueArena {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    pub fn alloc(&mut self, value: crate::Value) -> ValueId {
        let id = ValueId(self.values.len());
        self.values.push(value);
        id
    }
    pub fn get(&self, id: ValueId) -> Option<&crate::Value> {
        self.values.get(id.0)
    }
}
