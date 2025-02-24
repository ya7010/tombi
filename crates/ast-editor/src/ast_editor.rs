use schema_store::DocumentSchema;

use crate::ast_change::AstChange;
use crate::Edit;

pub struct AstEditor<'a> {
    root: ast::Root,
    #[allow(dead_code)]
    changes: Vec<AstChange>,
    document_schema: Option<&'a DocumentSchema>,
    sub_schema_url_map: Option<&'a schema_store::SubSchemaUrlMap>,
    schema_store: &'a schema_store::SchemaStore,
}

impl<'a> AstEditor<'a> {
    pub fn new(
        root: ast::Root,
        document_schema: Option<&'a DocumentSchema>,
        sub_schema_url_map: Option<&'a schema_store::SubSchemaUrlMap>,
        schema_store: &'a schema_store::SchemaStore,
    ) -> Self {
        Self {
            root,
            changes: vec![],
            document_schema,
            sub_schema_url_map,
            schema_store,
        }
    }

    pub async fn edit(&mut self) {
        self.root
            .edit(
                &[],
                self.document_schema
                    .map(|document_schema| &document_schema.schema_url),
                self.document_schema
                    .and_then(|document_schema| document_schema.value_schema.as_ref()),
                self.document_schema
                    .map(|document_schema| &document_schema.definitions),
                self.sub_schema_url_map,
                self.schema_store,
            )
            .await;
    }
}
