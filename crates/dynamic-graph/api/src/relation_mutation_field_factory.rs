use async_graphql::dynamic::Field;
use async_trait::async_trait;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::RelationType;
use reactive_graph_lifecycle::Lifecycle;
use springtime_di::injectable;

#[injectable]
#[async_trait]
pub trait RelationMutationFieldFactory: Send + Sync + Lifecycle {
    /// Creates the mutation fields for relation types in the given namespace.
    fn get_mutation_fields(&self, namespace: &Namespace) -> Vec<Field>;

    /// Creates the creation field for the given relation type.
    fn create_creation_field(&self, relation_type: &RelationType) -> Field;
}
