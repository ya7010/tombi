// arena/string.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StrId(usize);

#[derive(Debug, Clone, Default)]
pub struct StrArena {
    pub(crate) strings: Vec<String>,
}

impl StrArena {
    pub fn new() -> Self {
        Self {
            strings: Vec::new(),
        }
    }
    pub fn alloc(&mut self, s: &str) -> StrId {
        let id = StrId(self.strings.len());
        self.strings.push(s.to_owned());
        id
    }
    pub fn get(&self, id: StrId) -> Option<&str> {
        self.strings.get(id.0).map(|s| s.as_str())
    }
}
