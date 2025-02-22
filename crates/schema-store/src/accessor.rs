/// Represents an accessor to a value in a TOML-like structure.
/// It can either be a key (for objects) or an index (for arrays).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Accessor {
    Key(String),
    Index(usize),
}

impl Accessor {
    #[inline]
    pub fn is_key(&self) -> bool {
        matches!(self, Accessor::Key(_))
    }

    #[inline]
    pub fn is_index(&self) -> bool {
        matches!(self, Accessor::Index(_))
    }
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
#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub struct Accessors(Vec<Accessor>);

impl Accessors {
    #[inline]
    pub fn new(accessors: Vec<Accessor>) -> Self {
        Self(accessors)
    }

    #[inline]
    pub fn first(&self) -> Option<&Accessor> {
        self.0.first()
    }

    #[inline]
    pub fn last(&self) -> Option<&Accessor> {
        self.0.last()
    }
}

impl AsRef<[Accessor]> for Accessors {
    fn as_ref(&self) -> &[Accessor] {
        &self.0
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

/// Represents an accessor to a value in a TOML-like structure.
/// It can either be a key (for objects) or an index (for arrays).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SchemaAccessor {
    Key(String),
    Index,
}

impl SchemaAccessor {
    /// Parse a schema access path into a sequence of accessors.
    ///
    /// # Examples
    ///
    /// ```
    /// use schema_store::{SchemaAccessor, Accessor};
    ///
    /// let accessors = SchemaAccessor::parse("key1[*].key2").unwrap();
    /// assert_eq!(accessors.len(), 3);
    /// assert_eq!(accessors[0], SchemaAccessor::Key("key1".to_string()));
    /// assert_eq!(accessors[1], SchemaAccessor::Index);
    /// assert_eq!(accessors[2], SchemaAccessor::Key("key2".to_string()));
    /// ```
    pub fn parse(path: &str) -> Option<Vec<SchemaAccessor>> {
        let mut accessors = Vec::new();
        let mut current_key = String::new();

        if path.is_empty() {
            return None;
        }

        let chars: Vec<char> = path.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            match chars[i] {
                '[' => {
                    if !current_key.is_empty() {
                        accessors.push(SchemaAccessor::Key(current_key));
                        current_key = String::new();
                    }
                    i += 1;
                    let mut index_str = String::new();
                    while i < chars.len() && chars[i] != ']' {
                        index_str.push(chars[i]);
                        i += 1;
                    }
                    if index_str == "*" {
                        accessors.push(SchemaAccessor::Index); // Use 0 as a placeholder for [*]
                    } else if let Ok(_) = index_str.parse::<usize>() {
                        accessors.push(SchemaAccessor::Index);
                    } else {
                        tracing::error!("Invalid schema accessor: {path}");
                        return None;
                    }
                }
                '.' => {
                    if !current_key.is_empty() {
                        accessors.push(SchemaAccessor::Key(current_key));
                        current_key = String::new();
                    }
                }
                c => {
                    current_key.push(c);
                }
            }
            i += 1;
        }

        if !current_key.is_empty() {
            accessors.push(SchemaAccessor::Key(current_key));
        }

        Some(accessors)
    }
}

impl PartialEq<Accessor> for SchemaAccessor {
    fn eq(&self, other: &Accessor) -> bool {
        match (self, other) {
            (SchemaAccessor::Key(key1), Accessor::Key(key2)) => key1 == key2,
            (SchemaAccessor::Index, Accessor::Index(_)) => true,
            _ => false,
        }
    }
}

impl From<Accessor> for SchemaAccessor {
    fn from(accessor: Accessor) -> Self {
        match accessor {
            Accessor::Key(key) => SchemaAccessor::Key(key),
            Accessor::Index(_) => SchemaAccessor::Index,
        }
    }
}

impl From<&Accessor> for SchemaAccessor {
    fn from(value: &Accessor) -> Self {
        match value {
            Accessor::Key(key) => SchemaAccessor::Key(key.clone()),
            Accessor::Index(_) => SchemaAccessor::Index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_accessor() {
        let cases = vec![
            (
                "key1[*].key2",
                vec![
                    Accessor::Key("key1".to_string()),
                    Accessor::Index(0),
                    Accessor::Key("key2".to_string()),
                ],
            ),
            (
                "key1[0].key2",
                vec![
                    Accessor::Key("key1".to_string()),
                    Accessor::Index(0),
                    Accessor::Key("key2".to_string()),
                ],
            ),
            (
                "simple.key",
                vec![
                    Accessor::Key("simple".to_string()),
                    Accessor::Key("key".to_string()),
                ],
            ),
            (
                "array[5]",
                vec![Accessor::Key("array".to_string()), Accessor::Index(5)],
            ),
        ];

        for (input, expected) in cases {
            let result = SchemaAccessor::parse(input).unwrap();
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }
}
