use date_time::{LocalDate, LocalDateTime, LocalTime, OffsetDateTime, TimeZoneOffset};
use rstest::rstest;
use serde_json::json;

#[test]
fn test_local_date_deserialization() {
    let json = json!({"$__tombi_private_datetime":"2021-01-01"});
    let date: LocalDate = serde_json::from_value(json).unwrap();
    assert_eq!(date, LocalDate::from_ymd(2021, 1, 1));
}

#[test]
fn test_local_time_deserialization() {
    let json = json!({"$__tombi_private_datetime":"12:00:00"});
    let time: LocalTime = serde_json::from_value(json).unwrap();
    assert_eq!(time, LocalTime::from_hms(12, 0, 0));
}

#[rstest]
#[case("12:00:00.012", 12)]
#[case("12:00:00.123", 123)]
fn test_local_time_deserialization_with_milliseconds(
    #[case] input: &str,
    #[case] expected_milliseconds: u32,
) {
    let json = json!({"$__tombi_private_datetime": input});
    let time: LocalTime = serde_json::from_value(json).unwrap();
    assert_eq!(
        time,
        LocalTime::from_hms_milli(12, 0, 0, expected_milliseconds)
    );
}

#[rstest]
#[case("12:00:00.000001234", 1234)]
#[case("12:00:00.123456789", 123456789)]
#[case("12:00:00.999999999", 999999999)]
fn test_local_time_deserialization_with_nanoseconds(
    #[case] input: &str,
    #[case] expected_nanoseconds: u32,
) {
    let json = json!({"$__tombi_private_datetime": input});
    let time: LocalTime = serde_json::from_value(json).unwrap();
    assert_eq!(
        time,
        LocalTime::from_hms_nano(12, 0, 0, expected_nanoseconds)
    );
}

#[test]
fn test_local_date_time_deserialization() {
    let json = json!({"$__tombi_private_datetime":"2021-01-01T12:00:00"});
    let date_time: LocalDateTime = serde_json::from_value(json).unwrap();
    assert_eq!(date_time, LocalDateTime::from_ymd_hms(2021, 1, 1, 12, 0, 0));
}

#[rstest]
#[case("2021-01-01T12:00:00.012", 12)]
#[case("2021-01-01T12:00:00.123", 123)]
fn test_local_date_time_deserialization_with_milliseconds(
    #[case] input: &str,
    #[case] expected_milliseconds: u32,
) {
    let json = json!({"$__tombi_private_datetime": input});
    let date_time: LocalDateTime = serde_json::from_value(json).unwrap();
    assert_eq!(
        date_time,
        LocalDateTime::from_ymd_hms_milli(2021, 1, 1, 12, 0, 0, expected_milliseconds)
    );
}

#[rstest]
#[case("2021-01-01T12:00:00.000001234", 1234)]
#[case("2021-01-01T12:00:00.123456789", 123456789)]
#[case("2021-01-01T12:00:00.999999999", 999999999)]
fn test_local_date_time_deserialization_with_nanoseconds(
    #[case] input: &str,
    #[case] expected_nanoseconds: u32,
) {
    let json = json!({"$__tombi_private_datetime": input});
    let date_time: LocalDateTime = serde_json::from_value(json).unwrap();
    assert_eq!(
        date_time,
        LocalDateTime::from_ymd_hms_nano(2021, 1, 1, 12, 0, 0, expected_nanoseconds)
    );
}

#[rstest]
#[case("2021-01-01T12:00:00Z", TimeZoneOffset::Z)]
#[case("2021-01-01T12:00:00+00:00", TimeZoneOffset::Custom { minutes: 0 })]
#[case("2021-01-01T12:00:00+00:30", TimeZoneOffset::Custom { minutes: 30 })]
#[case("2021-01-01T12:00:00-00:30", TimeZoneOffset::Custom { minutes: -30 })]
fn test_offset_date_time_deserialization(
    #[case] input: &str,
    #[case] expected_offset: TimeZoneOffset,
) {
    let json = json!({"$__tombi_private_datetime": input});
    let date_time: OffsetDateTime = serde_json::from_value(json).unwrap();
    assert_eq!(
        date_time,
        OffsetDateTime::from_ymd_hms(2021, 1, 1, 12, 0, 0, expected_offset)
    );
}
