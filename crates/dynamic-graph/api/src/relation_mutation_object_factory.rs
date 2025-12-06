use async_graphql::dynamic::Field;
use async_graphql::dynamic::Object;
use async_trait::async_trait;
use reactive_graph_graph::RelationType;
use reactive_graph_lifecycle::Lifecycle;
use springtime_di::injectable;

#[injectable]
#[async_trait]
pub trait RelationMutationObjectFactory: Send + Sync + Lifecycle {
    /// Creates the mutation objects for all relation types.
    fn create_mutation_objects(&self) -> Vec<Object>;

    /// Creates the mutation object for the given relation type.
    fn create_mutation_object(&self, relation_type: RelationType) -> Object;

    /// Creates the update field in the mutation object for the given relation type.
    fn create_update_field(&self, relation_type: &RelationType) -> Option<Field>;

    /// Creates the trigger field in the mutation object for the given relation type.
    fn create_trigger_field(&self, relation_type: &RelationType) -> Option<Field>;

    /// Creates the export field in the mutation object for the given relation type.
    fn create_export_field(&self) -> Field;

    /// Creates the delete field in the mutation object for the given relation type.
    fn create_delete_field(&self) -> Field;
}
