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
