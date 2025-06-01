use uuid::Uuid;

use reactive_graph_graph::FlowInstance;
use reactive_graph_graph::FlowTypeId;
use reactive_graph_graph::PropertyInstances;
use reactive_graph_reactive_model_impl::ReactiveFlow;
use reactive_graph_reactive_service_api::ReactiveFlowCreationError;

pub trait FlowInstanceManager: Send + Sync {
    /// Returns true, if an flow instance exists with the given UUID.
    fn has(&self, id: Uuid) -> bool;

    /// Returns the flow instance with the given UUID or None.
    fn get(&self, id: Uuid) -> Option<ReactiveFlow>;

    /// Returns the flow instance with the given label or None.
    fn get_by_label(&self, label: &str) -> Option<ReactiveFlow>;

    /// Creates a new reactive flow instance from the given flow instance descriptor.
    ///
    /// The wrapper entity instance will be created as well as entity and
    /// relation instances.
    ///
    /// All reactive instances will be registered in the ReactiveEntityManager
    /// and the ReactiveRelationManager.
    fn create(&self, flow_instance: FlowInstance) -> Result<ReactiveFlow, ReactiveFlowCreationError>;

    /// Create a new reactive flow instance from the flow type by the given name.
    ///
    /// It's possible to individualize the flow instance with templating using the given variables.
    ///
    /// The wrapper entity instance will be created as well as entity and
    /// relation instances.
    ///
    /// All reactive instances will be registered in the ReactiveEntityManager
    /// and the ReactiveRelationManager.
    fn create_from_type(
        &self,
        ty: &FlowTypeId,
        id: Option<Uuid>,
        variables: PropertyInstances,
        properties: PropertyInstances,
    ) -> Result<ReactiveFlow, ReactiveFlowCreationError>;

    /// Deletes the flow instance with the given id.
    fn delete(&self, id: Uuid) -> bool;
}
