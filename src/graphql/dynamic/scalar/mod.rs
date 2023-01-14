use async_graphql::dynamic::Scalar;
use async_graphql::dynamic::SchemaBuilder;

pub fn get_scalars(schema: SchemaBuilder) -> SchemaBuilder {
    schema.register(Scalar::new("JSON"))
}
