use futures::FutureExt;

impl crate::Edit for ast::Root {
    fn edit<'a: 'b, 'b>(
        &'a self,
        _accessors: &'a [schema_store::Accessor],
        _schema_url: Option<&'a schema_store::SchemaUrl>,
        _value_schema: Option<&'a schema_store::ValueSchema>,
        _definitions: Option<&'a schema_store::SchemaDefinitions>,
        _sub_schema_url_map: Option<&'a schema_store::SubSchemaUrlMap>,
        _schema_store: &'a schema_store::SchemaStore,
    ) -> futures::future::BoxFuture<'b, ()> {
        async move {}.boxed()
    }
}
