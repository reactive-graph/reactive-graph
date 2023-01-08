use async_graphql::dynamic::*;

use crate::graphql::dynamic::get_namespace;
use crate::graphql::dynamic::namespace_field;
use crate::graphql::dynamic::SchemaBuilderContext;

pub fn get_query(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    let mut query = Object::new("Query");
    for namespace in context.namespace_manager.get_all() {
        if let Some(object_namespace) = get_namespace(context.clone(), &namespace) {
            query = query.field(namespace_field(&namespace));
            schema = schema.register(object_namespace)
        }
    }
    schema.register(query)
}
