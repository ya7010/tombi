use ast::AstNode;
use document_tree::TryIntoDocumentTree;
use itertools::Itertools;
use schema_store::ArraySchema;
use syntax::SyntaxElement;
use x_tombi::ArrayValuesOrder;

pub async fn array_values_order(
    node: &syntax::SyntaxNode,
    array_schema: &ArraySchema,
    toml_version: toml_version::TomlVersion,
) -> Vec<crate::Change> {
    let values_order = match array_schema {
        ArraySchema {
            values_order: Some(values_order),
            ..
        } => values_order,
        _ => return Vec::with_capacity(0),
    };

    let values = if let Some(array) = ast::Array::cast(node.clone()) {
        array.values().collect_vec()
    } else {
        return Vec::with_capacity(0);
    };

    if values.is_empty() {
        return Vec::with_capacity(0);
    }

    let old = std::ops::RangeInclusive::new(
        SyntaxElement::Node(values.first().unwrap().syntax().clone()),
        SyntaxElement::Node(values.last().unwrap().syntax().clone()),
    );

    let sortable_values = match SortableValues::new(values, toml_version) {
        Ok(sortable_values) => sortable_values,
        Err(err) => {
            tracing::error!("{err}");
            return Vec::with_capacity(0);
        }
    };

    match values_order {
        ArrayValuesOrder::Ascending => {
            let new = sortable_values
                .sorted()
                .into_iter()
                .map(|kv| SyntaxElement::Node(kv.syntax().clone()))
                .collect_vec();

            vec![crate::Change::ReplaceRange { old, new }]
        }
        ArrayValuesOrder::Descending => {
            let new = sortable_values
                .sorted()
                .into_iter()
                .rev()
                .map(|kv| SyntaxElement::Node(kv.syntax().clone()))
                .collect_vec();

            vec![crate::Change::ReplaceRange { old, new }]
        }
    }
}

enum SortableType {
    Boolean,
    Integer,
    String,
    OffsetDateTime,
    LocalDateTime,
    LocalDate,
    LocalTime,
}

enum SortableValues {
    Boolean(Vec<(bool, ast::Value)>),
    Integer(Vec<(i64, ast::Value)>),
    String(Vec<(String, ast::Value)>),
    OffsetDateTime(Vec<(String, ast::Value)>),
    LocalDateTime(Vec<(String, ast::Value)>),
    LocalDate(Vec<(String, ast::Value)>),
    LocalTime(Vec<(String, ast::Value)>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, thiserror::Error)]
enum Error {
    #[error("Cannot sort array values because the values are empty.")]
    Empty,

    #[error("Cannot sort array values because the values are incomplete.")]
    Incomplete,

    #[error("Cannot sort array values because the values only support the following types: [Boolean, Integer, String, OffsetDateTime, LocalDateTime, LocalDate, LocalTime]")]
    UnsupportedTypes,

    #[error("Cannot sort array values because the values have different types.")]
    DifferentTypes,
}

impl SortableValues {
    pub fn new(
        values: Vec<ast::Value>,
        toml_version: toml_version::TomlVersion,
    ) -> Result<Self, Error> {
        if values.is_empty() {
            return Err(Error::UnsupportedTypes);
        }

        let sortable_type = match values.first().unwrap() {
            ast::Value::Boolean(_) => SortableType::Boolean,
            ast::Value::IntegerBin(_)
            | ast::Value::IntegerOct(_)
            | ast::Value::IntegerDec(_)
            | ast::Value::IntegerHex(_) => SortableType::Integer,
            ast::Value::BasicString(_)
            | ast::Value::LiteralString(_)
            | ast::Value::MultiLineBasicString(_)
            | ast::Value::MultiLineLiteralString(_) => SortableType::String,
            ast::Value::OffsetDateTime(_) => SortableType::OffsetDateTime,
            ast::Value::LocalDateTime(_) => SortableType::LocalDateTime,
            ast::Value::LocalDate(_) => SortableType::LocalDate,
            ast::Value::LocalTime(_) => SortableType::LocalTime,
            _ => return Err(Error::Empty),
        };

        let sortable_values = match sortable_type {
            SortableType::Boolean => {
                let mut sortable_values = Vec::with_capacity(values.len());
                for value in values {
                    if let ast::Value::Boolean(_) = value {
                        match value.syntax().to_string().as_ref() {
                            "true" => sortable_values.push((true, value)),
                            "false" => sortable_values.push((false, value)),
                            _ => return Err(Error::Incomplete),
                        }
                    } else {
                        return Err(Error::DifferentTypes);
                    }
                }
                SortableValues::Boolean(sortable_values)
            }
            SortableType::Integer => {
                let mut sortable_values = Vec::with_capacity(values.len());
                for value in values {
                    match value.clone() {
                        ast::Value::IntegerBin(integer_bin) => {
                            if let Ok(document_tree::Value::Integer(integer)) =
                                integer_bin.try_into_document_tree(toml_version)
                            {
                                sortable_values.push((integer.value(), value));
                            } else {
                                return Err(Error::Incomplete);
                            }
                        }
                        ast::Value::IntegerOct(integer_oct) => {
                            if let Ok(document_tree::Value::Integer(integer)) =
                                integer_oct.try_into_document_tree(toml_version)
                            {
                                sortable_values.push((integer.value(), value));
                            } else {
                                return Err(Error::Incomplete);
                            }
                        }
                        ast::Value::IntegerDec(integer_dec) => {
                            if let Ok(document_tree::Value::Integer(integer)) =
                                integer_dec.try_into_document_tree(toml_version)
                            {
                                sortable_values.push((integer.value(), value));
                            } else {
                                return Err(Error::Incomplete);
                            }
                        }
                        ast::Value::IntegerHex(integer_hex) => {
                            if let Ok(document_tree::Value::Integer(integer)) =
                                integer_hex.try_into_document_tree(toml_version)
                            {
                                sortable_values.push((integer.value(), value));
                            } else {
                                return Err(Error::Incomplete);
                            }
                        }
                        _ => return Err(Error::DifferentTypes),
                    }
                }
                SortableValues::Integer(sortable_values)
            }
            SortableType::String => {
                let mut sortable_values = Vec::with_capacity(values.len());
                for value in values {
                    match value.clone() {
                        ast::Value::BasicString(basic_string) => {
                            if let Ok(document_tree::Value::String(string)) =
                                basic_string.try_into_document_tree(toml_version)
                            {
                                sortable_values.push((string.value().to_owned(), value));
                            } else {
                                return Err(Error::Incomplete);
                            }
                        }
                        ast::Value::LiteralString(literal_string) => {
                            if let Ok(document_tree::Value::String(string)) =
                                literal_string.try_into_document_tree(toml_version)
                            {
                                sortable_values.push((string.value().to_owned(), value));
                            } else {
                                return Err(Error::Incomplete);
                            }
                        }
                        ast::Value::MultiLineBasicString(multi_line_basic_string) => {
                            if let Ok(document_tree::Value::String(string)) =
                                multi_line_basic_string.try_into_document_tree(toml_version)
                            {
                                sortable_values.push((string.value().to_owned(), value));
                            } else {
                                return Err(Error::Incomplete);
                            }
                        }
                        ast::Value::MultiLineLiteralString(multi_line_literal_string) => {
                            if let Ok(document_tree::Value::String(string)) =
                                multi_line_literal_string.try_into_document_tree(toml_version)
                            {
                                sortable_values.push((string.value().to_owned(), value));
                            } else {
                                return Err(Error::Incomplete);
                            }
                        }
                        _ => return Err(Error::UnsupportedTypes),
                    }
                }
                SortableValues::String(sortable_values)
            }
            SortableType::OffsetDateTime => {
                let mut sortable_values = Vec::with_capacity(values.len());
                for value in values {
                    if let ast::Value::OffsetDateTime(_) = value {
                        sortable_values.push((value.syntax().to_string(), value));
                    } else {
                        return Err(Error::DifferentTypes);
                    }
                }
                SortableValues::OffsetDateTime(sortable_values)
            }
            SortableType::LocalDateTime => {
                let mut sortable_values = Vec::with_capacity(values.len());
                for value in values {
                    if let ast::Value::LocalDateTime(_) = value {
                        sortable_values.push((value.syntax().to_string(), value));
                    } else {
                        return Err(Error::DifferentTypes);
                    }
                }
                SortableValues::LocalDateTime(sortable_values)
            }
            SortableType::LocalDate => {
                let mut sortable_values = Vec::with_capacity(values.len());
                for value in values {
                    if let ast::Value::LocalDate(_) = value {
                        sortable_values.push((value.syntax().to_string(), value));
                    } else {
                        return Err(Error::DifferentTypes);
                    }
                }
                SortableValues::LocalDate(sortable_values)
            }
            SortableType::LocalTime => {
                let mut sortable_values = Vec::with_capacity(values.len());
                for value in values {
                    if let ast::Value::LocalTime(_) = value {
                        sortable_values.push((value.syntax().to_string(), value));
                    } else {
                        return Err(Error::DifferentTypes);
                    }
                }
                SortableValues::LocalTime(sortable_values)
            }
        };

        Ok(sortable_values)
    }

    pub fn sorted(self) -> Vec<ast::Value> {
        match self {
            Self::Boolean(mut sortable_values) => {
                sortable_values.sort_by_key(|(key, _)| *key);

                sortable_values
                    .into_iter()
                    .map(|(_, value)| value)
                    .collect_vec()
            }
            Self::Integer(mut sortable_values) => {
                sortable_values.sort_by_key(|(key, _)| *key);

                sortable_values
                    .into_iter()
                    .map(|(_, value)| value)
                    .collect_vec()
            }
            Self::String(mut sortable_values) => {
                sortable_values.sort_by_key(|(key, _)| key.clone());

                sortable_values
                    .into_iter()
                    .map(|(_, value)| value)
                    .collect_vec()
            }
            Self::OffsetDateTime(mut sortable_values) => {
                sortable_values.sort_by_key(|(key, _)| key.clone());

                sortable_values
                    .into_iter()
                    .map(|(_, value)| value)
                    .collect_vec()
            }
            Self::LocalDateTime(mut sortable_values) => {
                sortable_values.sort_by_key(|(key, _)| key.clone());

                sortable_values
                    .into_iter()
                    .map(|(_, value)| value)
                    .collect_vec()
            }
            Self::LocalDate(mut sortable_values) => {
                sortable_values.sort_by_key(|(key, _)| key.clone());

                sortable_values
                    .into_iter()
                    .map(|(_, value)| value)
                    .collect_vec()
            }
            Self::LocalTime(mut sortable_values) => {
                sortable_values.sort_by_key(|(key, _)| key.clone());

                sortable_values
                    .into_iter()
                    .map(|(_, value)| value)
                    .collect_vec()
            }
        }
    }
}
