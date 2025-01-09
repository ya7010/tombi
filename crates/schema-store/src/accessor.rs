#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Key(String);

impl Key {
    pub fn new(key: String) -> Self {
        Self(key)
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> From<T> for Key
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

/// Represents an accessor to a value in a TOML-like structure.
/// It can either be a key (for objects) or an index (for arrays).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Accessor {
    Key(Key),
    Index(usize),
}

impl std::fmt::Display for Accessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Accessor::Key(key) => write!(f, "{}", key),
            Accessor::Index(index) => write!(f, "[{}]", index),
        }
    }
}

/// A collection of `Accessor`.
#[derive(Debug, Default)]
pub struct Accessors(Vec<Accessor>);

impl Accessors {
    pub fn new(accessors: Vec<Accessor>) -> Self {
        Self(accessors)
    }
}

impl std::fmt::Display for Accessors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.0.iter();
        if let Some(accessor) = iter.next() {
            write!(f, "{}", accessor)?;
            for accessor in iter {
                match accessor {
                    Accessor::Key(_) => write!(f, ".{}", accessor)?,
                    Accessor::Index(_) => write!(f, "{}", accessor)?,
                }
            }
        }
        Ok(())
    }
}
