use std::path::PathBuf;

fn project_root() -> PathBuf {
    let dir = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned());
    PathBuf::from(dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned()
}

pub fn tombi_schema_path() -> PathBuf {
    project_root().join("tombi.schema.json")
}

pub fn cargo_schema_path() -> PathBuf {
    project_root().join("schemas").join("cargo.schema.json")
}

pub fn pyproject_schema_path() -> PathBuf {
    project_root().join("schemas").join("pyproject.schema.json")
}

pub fn type_test_schema_path() -> PathBuf {
    project_root().join("schemas").join("type-test.schema.json")
}

pub fn today_offset_date_time() -> String {
    let mut today = chrono::Local::now();
    if let Some(time) = chrono::NaiveTime::from_hms_opt(0, 0, 0) {
        today = match today.with_time(time) {
            chrono::LocalResult::Single(today) => today,
            _ => today,
        };
    };
    today.format("%Y-%m-%dT%H:%M:%S%.3f%:z").to_string()
}

pub fn today_local_date_time() -> String {
    let mut today = chrono::Local::now();
    if let Some(time) = chrono::NaiveTime::from_hms_opt(0, 0, 0) {
        today = match today.with_time(time) {
            chrono::LocalResult::Single(today) => today,
            _ => today,
        };
    };
    today.format("%Y-%m-%dT%H:%M:%S%.3f").to_string()
}

pub fn today_local_date() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

pub fn today_local_time() -> String {
    let mut today = chrono::Local::now();
    if let Some(time) = chrono::NaiveTime::from_hms_opt(0, 0, 0) {
        today = match today.with_time(time) {
            chrono::LocalResult::Single(today) => today,
            _ => today,
        };
    };
    today.format("%H:%M:%S%.3f").to_string()
}
