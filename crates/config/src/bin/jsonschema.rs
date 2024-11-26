fn main() {
    #[cfg(feature = "jsonschema")]
    {
        use schemars::schema_for;
        let schema = schema_for!(config::Config);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }

    #[cfg(not(feature = "jsonschema"))]
    {
        eprintln!("Feature `jsonschema` is required to generate JSON schema.");
        std::process::exit(1);
    }
}
