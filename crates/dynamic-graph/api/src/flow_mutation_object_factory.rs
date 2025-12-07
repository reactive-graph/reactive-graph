use async_graphql::dynamic::Field;
use async_graphql::dynamic::Object;
use async_trait::async_trait;
use reactive_graph_graph::FlowType;
use reactive_graph_lifecycle::Lifecycle;
use springtime_di::injectable;

#[injectable]
#[async_trait]
pub trait FlowMutationObjectFactory: Send + Sync + Lifecycle {
    /// Creates the mutation objects for all flow types.
    fn create_mutation_objects(&self) -> Vec<Object>;

    /// Create the mutation object for the given flow type.
    fn create_mutation_object(&self, flow_type: FlowType) -> Object;

    /// Creates the update field of the mutation object for the given flow type.
    fn create_update_field(&self, flow_type: &FlowType) -> Option<Field>;

    /// Creates the trigger field of the mutation object for the given flow type.
    fn create_trigger_field(&self, flow_type: &FlowType) -> Option<Field>;

    /// Creates the delete field of the mutation object for the given flow type.
    fn create_delete_field(&self) -> Field;
}
