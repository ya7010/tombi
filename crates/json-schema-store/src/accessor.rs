#[derive(Debug)]
pub enum Accessor {
    Key(String),
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

pub fn get_accessors(
    root: document_tree::Root,
    keys: &[document_tree::Key],
    position: text::Position,
) -> Vec<Accessor> {
    use document_tree::{Table, Value};
    let mut accessors = Vec::new();
    let table: Table = root.into();
    let mut table_ref = &table;

    for key in keys {
        accessors.push(Accessor::Key(key.to_string()));
        if let Some(value) = table_ref.get(key) {
            match value {
                Value::Table(tbl) => {
                    table_ref = tbl;
                }
                Value::Array(array) => {
                    let mut index = 0;
                    for value in array.values() {
                        if value.range().contains(position) {
                            accessors.push(Accessor::Index(index));
                            if let Some(tbl) = get_item_table(value, &mut accessors) {
                                table_ref = tbl;
                            }
                            break;
                        }
                        index += 1;
                    }
                }
                _ => {}
            }
        }
    }

    accessors
}

fn get_item_table<'a>(
    value: &'a document_tree::Value,
    accessors: &mut Vec<Accessor>,
) -> Option<&'a document_tree::Table> {
    match value {
        document_tree::Value::Table(tbl) => Some(tbl),
        document_tree::Value::Array(ary) => {
            let mut index = 0;
            for value in ary.values() {
                if let Some(tbl) = get_item_table(value, accessors) {
                    accessors.push(Accessor::Index(index));
                    return Some(tbl);
                }
                index += 1;
            }
            None
        }
        _ => None,
    }
}
