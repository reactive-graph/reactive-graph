use std::sync::Arc;
use uuid::Uuid;

use crate::reactive::ComponentBehaviourTypeId;
use crate::reactive::ReactiveEntity;
use crate::behaviour::BehaviourFactory;

pub trait EntityComponentBehaviourRegistry: Send + Sync {
    /// Registers a factory for creating entity component behaviours.
    /// If an entity instance has the given component then the given behaviour is applied.
    /// The behaviour will be created using the given EntityBehaviourCreator.
    #[allow(unused_variables)]
    fn register(&self, component_behaviour_ty: ComponentBehaviourTypeId, factory: Arc<dyn BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync>) {
        panic!("EntityComponentBehaviourRegistry::register");
    }

    /// Unregisters a factory for creating entity component behaviours.
    #[allow(unused_variables)]
    fn unregister(&self, component_behaviour_ty: &ComponentBehaviourTypeId) {
        panic!("EntityComponentBehaviourRegistry::unregister");
    }
}
