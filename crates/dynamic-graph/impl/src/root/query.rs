use crate::field::namespace_query_field;
use crate::object::namespace::metrics::metrics_field;
use crate::object::namespace::metrics::metrics_object;
use crate::object::namespace::namespace_query;
use async_graphql::dynamic::*;
use itertools::Itertools;
use reactive_graph_dynamic_graph_api::SchemaBuilderContext;

pub fn get_query(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    let mut query = Object::new("Query").description("Queries");
    schema = schema.register(metrics_object());
    query = query.field(metrics_field(None));
    for namespace in context.namespace_manager.get_all().iter().sorted() {
        if let Some(object_namespace) = namespace_query(context.clone(), namespace) {
            query = query.field(namespace_query_field(namespace));
            schema = schema.register(object_namespace);
        }
    }
    schema.register(query)
}
