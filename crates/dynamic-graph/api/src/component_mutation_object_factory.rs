use async_graphql::dynamic::Field;
use async_graphql::dynamic::Object;
use async_trait::async_trait;
use reactive_graph_graph::Component;
use reactive_graph_lifecycle::Lifecycle;
use springtime_di::injectable;

#[injectable]
#[async_trait]
pub trait ComponentMutationObjectFactory: Send + Sync + Lifecycle {
    /// Creates the mutation objects for all components.
    fn create_mutation_objects(&self) -> Vec<Object>;

    /// Creates the mutation object for the given component.
    fn create_mutation_object(&self, component: Component) -> Object;

    /// Creates the update field for the given component.
    fn create_update_field(&self, component: &Component) -> Option<Field>;
}
