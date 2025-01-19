use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValueType {
    Null,
    Boolean,
    Integer,
    Float,
    String,
    OffsetDateTime,
    LocalDateTime,
    LocalDate,
    LocalTime,
    Array,
    Table,
    OneOf(Vec<ValueType>),
    AnyOf(Vec<ValueType>),
    AllOf(Vec<ValueType>),
}

impl ValueType {
    pub fn into_nullable(self) -> Self {
        match self {
            ValueType::Null => ValueType::Null,
            ValueType::Boolean
            | ValueType::Integer
            | ValueType::Float
            | ValueType::String
            | ValueType::OffsetDateTime
            | ValueType::LocalDateTime
            | ValueType::LocalDate
            | ValueType::LocalTime
            | ValueType::Array
            | ValueType::Table => ValueType::AnyOf(vec![self, ValueType::Null]),
            ValueType::OneOf(mut types) => {
                if !types.iter().any(|t| t.is_nullable()) {
                    types.push(ValueType::Null);
                }
                ValueType::OneOf(types)
            }
            ValueType::AnyOf(mut types) => {
                if !types.iter().all(|t| t.is_nullable()) {
                    types.push(ValueType::Null);
                }
                ValueType::AnyOf(types)
            }
            ValueType::AllOf(types) => {
                if types.iter().all(|t| !t.is_nullable()) {
                    ValueType::AnyOf(vec![ValueType::AllOf(types), ValueType::Null])
                } else {
                    ValueType::AllOf(types)
                }
            }
        }
    }

    pub fn is_nullable(&self) -> bool {
        match self {
            ValueType::Null => true,
            ValueType::Boolean
            | ValueType::Integer
            | ValueType::Float
            | ValueType::String
            | ValueType::OffsetDateTime
            | ValueType::LocalDateTime
            | ValueType::LocalDate
            | ValueType::LocalTime
            | ValueType::Array
            | ValueType::Table => false,
            ValueType::OneOf(types) | ValueType::AnyOf(types) => {
                types.iter().any(|t| t.is_nullable())
            }
            ValueType::AllOf(types) => types.iter().all(|t| t.is_nullable()),
        }
    }
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Null => {
                // NOTE: If this representation appears in the Hover of the Language Server, it is a bug.
                write!(f, "Null")
            }
            ValueType::Boolean => write!(f, "Boolean"),
            ValueType::Integer => write!(f, "Integer"),
            ValueType::Float => write!(f, "Float"),
            ValueType::String => write!(f, "String"),
            ValueType::OffsetDateTime => write!(f, "OffsetDateTime"),
            ValueType::LocalDateTime => write!(f, "LocalDateTime"),
            ValueType::LocalDate => write!(f, "LocalDate"),
            ValueType::LocalTime => write!(f, "LocalTime"),
            ValueType::Array => write!(f, "Array"),
            ValueType::Table => write!(f, "Table"),
            ValueType::OneOf(ref types) => fmt_composit_types(f, types, '^'),
            ValueType::AnyOf(types) => fmt_composit_types(f, types, '|'),
            ValueType::AllOf(types) => fmt_composit_types(f, types, '&'),
        }
    }
}

impl From<document_tree::ValueType> for ValueType {
    fn from(value_type: document_tree::ValueType) -> Self {
        match value_type {
            document_tree::ValueType::Boolean => ValueType::Boolean,
            document_tree::ValueType::Integer => ValueType::Integer,
            document_tree::ValueType::Float => ValueType::Float,
            document_tree::ValueType::String => ValueType::String,
            document_tree::ValueType::OffsetDateTime => ValueType::OffsetDateTime,
            document_tree::ValueType::LocalDateTime => ValueType::LocalDateTime,
            document_tree::ValueType::LocalDate => ValueType::LocalDate,
            document_tree::ValueType::LocalTime => ValueType::LocalTime,
            document_tree::ValueType::Array => ValueType::Array,
            document_tree::ValueType::Table => ValueType::Table,
        }
    }
}

fn fmt_composit_types(
    f: &mut std::fmt::Formatter<'_>,
    types: &[ValueType],
    separator: char,
) -> std::fmt::Result {
    let mut nullable = false;
    let non_null_types = types
        .into_iter()
        .filter(|t| {
            if let ValueType::Null = t {
                nullable = true;
                false
            } else {
                true
            }
        })
        .collect::<Vec<_>>();

    if nullable {
        if non_null_types.len() == 1 {
            write!(f, "{}?", non_null_types[0])
        } else {
            write!(
                f,
                "({})?",
                non_null_types
                    .iter()
                    .map(ToString::to_string)
                    .join(&format!(" {} ", separator)),
            )
        }
    } else {
        write!(
            f,
            "{}",
            types
                .iter()
                .map(ToString::to_string)
                .join(&format!(" {} ", separator))
        )
    }
}
