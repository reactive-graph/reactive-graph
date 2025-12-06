use async_graphql::dynamic::Field;
use async_graphql::dynamic::Object;
use async_trait::async_trait;
use reactive_graph_graph::EntityType;
use reactive_graph_lifecycle::Lifecycle;
use springtime_di::injectable;

#[injectable]
#[async_trait]
pub trait EntityMutationObjectFactory: Send + Sync + Lifecycle {
    /// Creates the mutation objects for all entity types.
    fn create_mutation_objects(&self) -> Vec<Object>;

    /// Create the mutation object for the given entity type.
    fn create_mutation_object(&self, entity_type: EntityType) -> Object;

    /// Creates the update field of the mutation object for the given flow type.
    fn create_update_field(&self, entity_type: &EntityType) -> Option<Field>;

    /// Creates the trigger field of the mutation object for the given flow type.
    fn create_trigger_field(&self, entity_type: &EntityType) -> Option<Field>;

    /// Creates the export field of the mutation object for the given flow type.
    fn create_export_field(&self) -> Field;

    /// Creates the delete field of the mutation object for the given flow type.
    fn create_delete_field(&self) -> Field;
}
