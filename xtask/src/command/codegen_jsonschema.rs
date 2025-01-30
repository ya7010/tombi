use schemars::{generate::SchemaSettings, SchemaGenerator};
use config::TomlVersion;
use crate::utils::project_root;

pub fn run() -> Result<(), anyhow::Error> {
    let settings = SchemaSettings::draft2020_12();
    let generator = SchemaGenerator::new(settings);
    
    std::fs::write(
        project_root().join("schemas/type-test.schema.json"),
        serde_json::to_string_pretty(&generator.clone().into_root_schema_for::<TypeTest>())? + "\n",
    )?;
    std::fs::write(
        project_root().join("tombi.schema.json"),
        serde_json::to_string_pretty(&generator.into_root_schema_for::<config::Config>())? + "\n",
    )?;
    Ok(())
}


#[derive(Debug, Default, Clone)]
#[derive(serde::Serialize)]
#[derive(schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
#[schemars(extend("x-tombi-toml-version" = TomlVersion::V1_1_0_Preview))]
struct TypeTest {
    boolean: Option<bool>,
    integer: Option<i64>,
    float: Option<f64>,
    array: Option<Vec<u64>>,
    offset_date_time: Option<chrono::DateTime<chrono::FixedOffset>>,
    local_date_time: Option<chrono::NaiveDateTime>,
    local_date: Option<chrono::NaiveTime>,
    local_time: Option<chrono::NaiveTime>
}