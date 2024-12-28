use schemars::{generate::SchemaSettings, SchemaGenerator};

use crate::utils::project_root;

pub fn run() -> Result<(), anyhow::Error> {
    let settings = SchemaSettings::draft2020_12();
    let generator = SchemaGenerator::new(settings);
    let schema = generator.into_root_schema_for::<config::Config>();

    std::fs::write(
        project_root().join("tombi.schema.json"),
        serde_json::to_string_pretty(&schema)? + "\n",
    )?;
    Ok(())
}
