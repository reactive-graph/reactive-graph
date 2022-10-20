use std::sync::Arc;

use crate::model::Component;
use crate::model::ReactiveEntityInstance;
use crate::model::ReactiveRelationInstance;
use indradb::EdgeKey;
use uuid::Uuid;

#[derive(Debug)]
pub enum ComponentBehaviourProviderError {
    InitializationError,
}

pub trait ComponentBehaviourProvider: Send + Sync {
    /// Possibly adds new behaviour to the given entity instance
    #[allow(unused_variables)]
    fn add_behaviours_to_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {}

    /// Possibly adds new behaviour to the given entity instance's component
    #[allow(unused_variables)]
    fn add_behaviours_to_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: Component) {}

    /// Possibly adds new behaviour to the given relation instance
    #[allow(unused_variables)]
    fn add_behaviours_to_relation(&self, relation_instance: Arc<ReactiveRelationInstance>) {}

    /// Possibly adds new behaviour to the given relation instance's component
    #[allow(unused_variables)]
    fn add_behaviours_to_relation_component(&self, relation_instance: Arc<ReactiveRelationInstance>, component: Component) {}

    /// Removes behaviour from the given entity instance
    #[allow(unused_variables)]
    fn remove_behaviours_from_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {}

    /// Removes behaviour from the given entity instance's component
    #[allow(unused_variables)]
    fn remove_behaviours_from_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: Component) {}

    /// Removes behaviour from the given relation instance
    #[allow(unused_variables)]
    fn remove_behaviours_from_relation(&self, relation_instance: Arc<ReactiveRelationInstance>) {}

    /// Removes behaviour from the given relation instance's component
    #[allow(unused_variables)]
    fn remove_behaviours_from_relation_component(&self, relation_instance: Arc<ReactiveRelationInstance>, component: Component) {}

    /// Removes behaviour from the given entity instance by uuid
    #[allow(unused_variables)]
    fn remove_behaviours_by_id(&self, id: Uuid) {}

    /// Removes behaviour from the given relation instance by edge key
    #[allow(unused_variables)]
    fn remove_behaviours_by_key(&self, edge_key: &EdgeKey) {}
}

#[macro_export]
macro_rules! component_behaviour_provider {
    ($component_behaviour_provider:expr) => {{
        let component_behaviour_provider = $component_behaviour_provider.clone();
        let component_behaviour_provider: Result<Arc<dyn ComponentBehaviourProvider>, _> =
            <dyn query_interface::Object>::query_arc(component_behaviour_provider);
        if component_behaviour_provider.is_err() {
            return Err(ComponentBehaviourProviderError::InitializationError);
        }
        Ok(component_behaviour_provider.ok())
    }};
}
