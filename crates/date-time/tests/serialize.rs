use date_time::{LocalDate, LocalDateTime, LocalTime, TimeZoneOffset, OffsetDateTime};
use rstest::rstest;
use serde_json::json;

#[test]
fn test_local_date_serialization() {
    let date = LocalDate::from_ymd(2021, 1, 1);

    pretty_assertions::assert_eq!(
        serde_json::to_value(&date).unwrap(),
        json!({"$__tombi_private_datetime":"2021-01-01"})
    );
}

#[test]
fn test_local_time_serialization() {
    let time = LocalTime::from_hms(12, 0, 0);

    pretty_assertions::assert_eq!(
        serde_json::to_value(&time).unwrap(),
        json!({"$__tombi_private_datetime":"12:00:00"})
    );
}

#[test]
fn test_local_date_time_serialization() {
    let date_time = LocalDateTime::from_ymd_hms(2021, 1, 1, 12, 0, 0);

    pretty_assertions::assert_eq!(
        serde_json::to_value(&date_time).unwrap(),
        json!({"$__tombi_private_datetime":"2021-01-01T12:00:00"})
    );
}

#[rstest]
#[case(TimeZoneOffset::Z, "2021-01-01T12:00:00Z")]
#[case(TimeZoneOffset::Custom { minutes: 0 }, "2021-01-01T12:00:00+00:00")]
#[case(TimeZoneOffset::Custom { minutes: 30 }, "2021-01-01T12:00:00+00:30")]
#[case(TimeZoneOffset::Custom { minutes: -30 }, "2021-01-01T12:00:00-00:30")]
fn test_offset_date_time_serialization(#[case] offset: TimeZoneOffset, #[case] expected: &str) {
    let date_time = OffsetDateTime::from_ymd_hms(2021, 1, 1, 12, 0, 0, offset);

    pretty_assertions::assert_eq!(
        serde_json::to_value(&date_time).unwrap(),
        json!({ "$__tombi_private_datetime": expected })
    );
}
