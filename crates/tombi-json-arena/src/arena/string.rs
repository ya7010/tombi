// arena/string.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StrId(usize);

#[derive(Default)]
pub struct StrArena<'a> {
    pub(crate) strings: Vec<&'a str>,
}

impl<'a> StrArena<'a> {
    pub fn new() -> Self {
        Self {
            strings: Vec::new(),
        }
    }
    pub fn alloc(&mut self, s: &'a str) -> StrId {
        let id = StrId(self.strings.len());
        self.strings.push(s);
        id
    }
    pub fn get(&self, id: StrId) -> Option<&'a str> {
        self.strings.get(id.0).copied()
    }
}
