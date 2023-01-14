use async_graphql::dynamic::*;

use crate::graphql::dynamic::namespace_mutation;
use crate::graphql::dynamic::namespace_mutation_field;
use crate::graphql::dynamic::SchemaBuilderContext;

pub fn get_mutation(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    let mut mutation = Object::new("Mutation").description("Mutations");
    for namespace in context.namespace_manager.get_all() {
        if let Some(object_namespace) = namespace_mutation(context.clone(), &namespace) {
            mutation = mutation.field(namespace_mutation_field(&namespace));
            schema = schema.register(object_namespace)
        }
    }
    schema.register(mutation)
}
