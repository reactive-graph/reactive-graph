use async_graphql::dynamic::Field;
use async_trait::async_trait;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::Namespace;
use reactive_graph_lifecycle::Lifecycle;
use springtime_di::injectable;

#[injectable]
#[async_trait]
pub trait EntityMutationFieldFactory: Send + Sync + Lifecycle {
    /// Creates the mutation fields for entity types in the given namespace.
    fn create_mutation_fields(&self, namespace: &Namespace) -> Vec<Field>;

    /// Creates the creation field for the given entity type.
    fn create_creation_field(&self, entity_type: &EntityType) -> Field;
}
