use config::TomlVersion;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseError {
    #[error(transparent)]
    Chrono(#[from] chrono::ParseError),
    #[error("optional seconds are allowed in TOML v1.1.0 or later")]
    OptionalSeconds,
}

pub fn try_from_offset_date_time(
    value: &str,
    toml_version: TomlVersion,
) -> Result<chrono::DateTime<chrono::FixedOffset>, ParseError> {
    Ok(chrono::DateTime::parse_from_rfc3339(&make_datetime_str(
        value,
        toml_version,
    )?)?)
}

pub fn try_from_local_date_time(
    value: &str,
    toml_version: TomlVersion,
) -> Result<chrono::NaiveDateTime, ParseError> {
    Ok(chrono::NaiveDateTime::parse_from_str(
        &make_datetime_str(value, toml_version)?,
        "%Y-%m-%d %H:%M:%S%.f",
    )?)
}

pub fn try_from_local_date(
    value: &str,
    _toml_version: TomlVersion,
) -> Result<chrono::NaiveDate, ParseError> {
    Ok(chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d")?)
}

pub fn try_from_local_time(
    value: &str,
    toml_version: TomlVersion,
) -> Result<chrono::NaiveTime, ParseError> {
    const HOUR_MINUTE_SIZE: usize = "00:00".len();

    // NOTE: Support optional seconds.
    //       See more infomation: https://github.com/toml-lang/toml/issues/671
    if value.chars().nth(HOUR_MINUTE_SIZE) == Some(':') {
        Ok(chrono::NaiveTime::parse_from_str(value, "%H:%M:%S%.f")?)
    } else {
        if toml_version < TomlVersion::V1_1_0_Preview {
            return Err(ParseError::OptionalSeconds);
        }
        Ok(chrono::NaiveTime::parse_from_str(value, "%H:%M%.f")?)
    }
}

#[inline]
fn make_datetime_str(value: &str, toml_version: TomlVersion) -> Result<String, ParseError> {
    const DEFAULT_SECONDS: &str = ":00";
    const SECONDS_SIZE: usize = DEFAULT_SECONDS.len();
    const DATE_SIZE: usize = "2024-12-31".len();
    const DATE_TIME_WITHOUT_SECONDS_SIZE: usize = "2024-01-01T00:00".len();

    let mut datetime_str = String::with_capacity(value.len() + SECONDS_SIZE);

    for (i, c) in value.char_indices() {
        if i == DATE_SIZE && matches!(c, 'T' | 't') {
            datetime_str.push(' ');
        } else if i == DATE_TIME_WITHOUT_SECONDS_SIZE && c != ':' {
            // NOTE: Support optional seconds.
            //       See more infomation: https://github.com/toml-lang/toml/issues/671
            if toml_version >= TomlVersion::V1_1_0_Preview {
                datetime_str.push_str(DEFAULT_SECONDS);
            } else {
                return Err(ParseError::OptionalSeconds);
            }

            datetime_str.push(c);
        } else {
            datetime_str.push(c);
        }
    }

    if datetime_str.len() == DATE_TIME_WITHOUT_SECONDS_SIZE {
        if toml_version >= TomlVersion::V1_1_0_Preview {
            datetime_str.push_str(DEFAULT_SECONDS);
        } else {
            return Err(ParseError::OptionalSeconds);
        }
    }

    Ok(datetime_str)
}
