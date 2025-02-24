use futures::FutureExt;

impl crate::Edit for ast::KeyValue {
    fn edit<'a: 'b, 'b>(
        &'a self,
        _accessors: &'a [schema_store::Accessor],
        _schema_url: Option<&'a schema_store::SchemaUrl>,
        _value_schema: Option<&'a schema_store::ValueSchema>,
        _definitions: Option<&'a schema_store::SchemaDefinitions>,
        _schema_context: &'a schema_store::SchemaContext<'a>,
    ) -> futures::future::BoxFuture<'b, Vec<crate::Change>> {
        async move { vec![] }.boxed()
    }
}
