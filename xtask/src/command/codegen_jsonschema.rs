use crate::utils::project_root;

pub fn run() -> Result<(), anyhow::Error> {
    use schemars::schema_for;
    let schema = schema_for!(config::Config);
    std::fs::write(
        &project_root().join("tombi.schema.json"),
        serde_json::to_string_pretty(&schema)? + "\n",
    )?;
    Ok(())
}
