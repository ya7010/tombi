use config::TomlVersion;

pub fn try_from_offset_date_time(
    value: &str,
    toml_version: TomlVersion,
) -> Result<chrono::DateTime<chrono::FixedOffset>, chrono::ParseError> {
    chrono::DateTime::parse_from_rfc3339(&make_datetime_str(value, toml_version))
}

pub fn try_from_local_date_time(
    value: &str,
    toml_version: TomlVersion,
) -> Result<chrono::NaiveDateTime, chrono::ParseError> {
    chrono::NaiveDateTime::parse_from_str(
        &make_datetime_str(value, toml_version),
        "%Y-%m-%d %H:%M:%S%.f",
    )
}

pub fn try_from_local_date(value: &str) -> Result<chrono::NaiveDate, chrono::ParseError> {
    chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d")
}

pub fn try_from_local_time(value: &str) -> Result<chrono::NaiveTime, chrono::ParseError> {
    // NOTE: Support optional seconds.
    //       See more infomation: https://github.com/toml-lang/toml/issues/671
    if value.chars().nth(5) == Some(':') {
        chrono::NaiveTime::parse_from_str(value, "%H:%M:%S%.f")
    } else {
        chrono::NaiveTime::parse_from_str(value, "%H:%M%.f")
    }
}

#[inline]
fn make_datetime_str(value: &str, toml_version: TomlVersion) -> String {
    let mut datetime_str = String::with_capacity(value.len() + 3);
    value.char_indices().for_each(|(i, c)| {
        if i == 10 && c == 'T' {
            datetime_str.push(' ');
        } else if i == 16 && c != ':' {
            // NOTE: Support optional seconds.
            //       See more infomation: https://github.com/toml-lang/toml/issues/671
            if toml_version >= TomlVersion::V1_1_0_Preview {
                datetime_str.push_str(":00");
            }
            datetime_str.push(c);
        } else {
            datetime_str.push(c);
        }
    });

    if datetime_str.len() == 16 {
        datetime_str.push_str(":00");
    }

    datetime_str
}
