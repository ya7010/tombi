use std::path::PathBuf;

use schema_store::DocumentSchema;

fn project_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let cargo_manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);

    if cargo_manifest_dir.ends_with("schema-store") {
        Ok(cargo_manifest_dir
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf())
    } else {
        Ok(cargo_manifest_dir)
    }
}

#[test]
fn tombi_schema() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::Read;

    let path = project_root()?;
    let document_path = path.join("tombi.schema.json");
    let file = File::open(&document_path)?;
    let mut reader = BufReader::new(file);

    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let document_schema = DocumentSchema::new(
        serde_json::from_str(&contents)?,
        url::Url::from_file_path(&document_path).unwrap(),
    );

    dbg!(document_schema);
    Ok(())
}
