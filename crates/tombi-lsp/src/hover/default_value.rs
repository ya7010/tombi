#[derive(Debug, Clone)]
pub enum DisplayValue {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    OffsetDateTime(String),
    LocalDateTime(String),
    LocalDate(String),
    LocalTime(String),
    Array(Vec<DisplayValue>),
    Table(Vec<(String, DisplayValue)>),
}

impl TryFrom<&tombi_json::Value> for DisplayValue {
    type Error = ();

    fn try_from(value: &tombi_json::Value) -> Result<Self, Self::Error> {
        match value {
            tombi_json::Value::Bool(boolean) => Ok(DisplayValue::Boolean(*boolean)),
            tombi_json::Value::Number(number) => match number {
                tombi_json::Number::Integer(integer) => Ok(DisplayValue::Integer(*integer)),
                tombi_json::Number::Float(float) => Ok(DisplayValue::Float(*float)),
            },
            tombi_json::Value::String(string) => Ok(DisplayValue::String(string.clone())),
            tombi_json::Value::Array(array) => Ok(DisplayValue::Array(
                array.iter().map(|item| item.try_into().unwrap()).collect(),
            )),
            tombi_json::Value::Object(object) => Ok(DisplayValue::Table(
                object
                    .iter()
                    .map(|(key, value)| (key.clone(), value.try_into().unwrap()))
                    .collect(),
            )),
            tombi_json::Value::Null => Err(()),
        }
    }
}

impl std::fmt::Display for DisplayValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayValue::Boolean(boolean) => write!(f, "{}", boolean),
            DisplayValue::Integer(integer) => write!(f, "{}", integer),
            DisplayValue::Float(float) => write!(f, "{}", float),
            DisplayValue::String(string) => write!(f, "\"{}\"", string.replace("\"", "\\\"")),
            DisplayValue::OffsetDateTime(offset_date_time) => write!(f, "{}", offset_date_time),
            DisplayValue::LocalDateTime(local_date_time) => write!(f, "{}", local_date_time),
            DisplayValue::LocalDate(local_date) => write!(f, "{}", local_date),
            DisplayValue::LocalTime(local_time) => write!(f, "{}", local_time),
            DisplayValue::Array(array) => {
                write!(f, "[")?;
                for (i, value) in array.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, "]")
            }
            DisplayValue::Table(table) => {
                write!(f, "{{ ")?;
                for (i, (key, value)) in table.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, value)?;
                }
                write!(f, " }}")
            }
        }
    }
}
