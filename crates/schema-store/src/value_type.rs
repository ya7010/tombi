use itertools::Itertools;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum ValueType {
    #[default]
    Any,
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

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Any => write!(f, "Any"),
            ValueType::Null => write!(f, "Null"),
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
