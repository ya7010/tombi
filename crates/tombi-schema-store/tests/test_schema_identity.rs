use std::{io::Write, str::FromStr};

use tombi_schema_store::{DocumentSchema, SchemaStore, SchemaUri};

#[tokio::test]
async fn distinguishes_schema_document_uri_schema_resource_uri_and_optional_id() {
    let schema_document_uri =
        SchemaUri::from_str("https://example.com/schemas/root.json").expect("valid retrieval URI");
    let schema_store = SchemaStore::new();

    let schema_without_id = DocumentSchema::new(
        tombi_json::ValueNode::from_str(r#"{ "type": "object" }"#).expect("valid schema"),
        schema_document_uri.clone(),
        None,
        &schema_store,
    )
    .await
    .expect("DocumentSchema::new");
    assert_eq!(schema_without_id.id, None);
    assert_eq!(
        schema_without_id.schema_document_uri(),
        &schema_document_uri
    );
    assert_eq!(
        schema_without_id.schema_resource_uri(),
        &schema_document_uri
    );
    assert_eq!(schema_without_id.schema_base_uri(), &schema_document_uri);

    let schema_with_id = DocumentSchema::new(
        tombi_json::ValueNode::from_str(r#"{ "$id": "canonical.json" }"#).expect("valid schema"),
        schema_document_uri.clone(),
        None,
        &schema_store,
    )
    .await
    .expect("DocumentSchema::new");
    let canonical_uri = SchemaUri::from_str("https://example.com/schemas/canonical.json")
        .expect("valid canonical URI");
    assert_eq!(schema_with_id.id.as_ref(), Some(&canonical_uri));
    assert_eq!(schema_with_id.schema_document_uri(), &schema_document_uri);
    assert_eq!(schema_with_id.schema_resource_uri(), &canonical_uri);
    assert_eq!(schema_with_id.schema_base_uri(), &canonical_uri);
}

#[tokio::test]
async fn store_rejects_duplicate_canonical_schema_resource_uri() {
    let mut schema_file =
        tempfile::NamedTempFile::with_suffix(".json").expect("temporary schema file");
    schema_file
        .write_all(
            br#"{
                "$defs": {
                    "first": { "$id": "https://example.com/duplicate" },
                    "second": { "$id": "https://example.com/duplicate" }
                }
            }"#,
        )
        .expect("write schema");
    let schema_uri = SchemaUri::from_file_path(schema_file.path()).expect("valid schema file URI");
    let schema_store = SchemaStore::new();

    let error = schema_store
        .try_get_document_schema(&schema_uri)
        .await
        .expect_err("duplicate canonical resource URI");

    assert!(
        error
            .to_string()
            .contains("duplicate $id `https://example.com/duplicate`"),
        "{error}"
    );
    assert!(error.to_string().contains("#/$defs/first"), "{error}");
    assert!(error.to_string().contains("#/$defs/second"), "{error}");
}

#[tokio::test]
async fn reload_config_drops_embedded_resource_index() {
    let mut schema_file =
        tempfile::NamedTempFile::with_suffix(".json").expect("temporary schema file");
    schema_file
        .write_all(
            br#"{
                "$ref": "shoko://embedded/resource",
                "$defs": {
                    "resource": {
                        "$id": "shoko://embedded/resource",
                        "type": "boolean"
                    }
                }
            }"#,
        )
        .expect("write schema");
    let schema_document_uri =
        SchemaUri::from_file_path(schema_file.path()).expect("valid schema file URI");
    let embedded_uri =
        SchemaUri::from_str("shoko://embedded/resource").expect("valid embedded URI");
    let schema_store = SchemaStore::new();

    assert!(
        schema_store
            .try_get_document_schema(&schema_document_uri)
            .await
            .expect("load compound schema")
            .is_some()
    );
    assert!(
        schema_store
            .try_get_document_schema(&embedded_uri)
            .await
            .expect("lookup embedded resource")
            .is_some()
    );

    schema_store
        .reload_config(&tombi_config::Config::default(), None)
        .await
        .expect("reload");

    let lookup = schema_store.try_get_document_schema(&embedded_uri).await;
    assert!(
        lookup.is_err() || lookup.ok().flatten().is_none(),
        "embedded resource must not survive config reload"
    );
}
