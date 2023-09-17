use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use uuid::Uuid;

use crate::api::Lifecycle;
use crate::model::FlowInstance;
use crate::model::FlowTypeId;
use crate::plugins::FlowInstanceProvider;
use crate::reactive::ReactiveFlow;
use crate::rt_api::ReactiveFlowCreationError;

#[async_trait]
pub trait ReactiveFlowManager: Send + Sync + Lifecycle {
    /// Returns true, if an flow instance exists with the given UUID.
    fn has(&self, id: Uuid) -> bool;

    /// Returns the flow instance with the given UUID or None.
    fn get(&self, id: Uuid) -> Option<ReactiveFlow>;

    /// Returns the flow instance that matches the given label or None.
    fn get_by_label(&self, label: &str) -> Option<ReactiveFlow>;

    /// Returns all reactive flow instances.
    fn get_all(&self) -> Vec<ReactiveFlow>;

    /// Returns the count of registered reactive flow instances.
    fn count_flow_instances(&self) -> usize;

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
    /// The wrapper entity instance will be created as well as entity and
    /// relation instances.
    ///
    /// The properties are assigned to the wrapper entity instance.
    ///
    /// The variables will replace the property value.
    ///
    /// All reactive instances will be registered in the ReactiveEntityManager
    /// and the ReactiveRelationManager.
    fn create_from_type(
        &self,
        ty: &FlowTypeId,
        variables: HashMap<String, Value>,
        properties: HashMap<String, Value>,
    ) -> Result<ReactiveFlow, ReactiveFlowCreationError>;

    /// Registers the given reactive flow instance and registers all of the reactive instances
    /// contained in the given reactive flow instance.
    fn register_flow_instance_and_reactive_instances(&self, reactive_flow_instance: ReactiveFlow);

    /// Registers the given reactive flow instance. Does not register it's reactive instances except
    /// the wrapper entity.
    fn register_flow_instance(&self, reactive_flow_instance: ReactiveFlow);

    // /// The changes of the reactive flow instance with the given id will be written to graph database.
    // // TODO: return result
    // fn commit(&self, id: Uuid);

    /// Deletes the flow instance with the given id.
    fn delete(&self, id: Uuid) -> bool;

    // fn import(&self, path: &str) -> Result<ReactiveFlow, ReactiveFlowImportError>;
    //
    // // TODO: return result
    // fn export(&self, id: Uuid, path: &str);

    /// Registers a flow instance provider.
    fn register_provider(&self, id: Uuid, flow_instance_provider: Arc<dyn FlowInstanceProvider>);

    /// Unregisters a flow instance provider.
    fn unregister_provider(&self, id: &Uuid);
}
