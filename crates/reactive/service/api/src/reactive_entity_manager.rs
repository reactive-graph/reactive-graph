use std::collections::HashMap;

use async_trait::async_trait;
use serde_json::Value;
use springtime_di::injectable;
use uuid::Uuid;

use crate::ReactiveEntityComponentAddError;
use crate::ReactiveEntityCreationError;
use crate::ReactiveEntityPropertyAddError;
use crate::ReactiveEntityPropertyRemoveError;
use crate::ReactiveEntityRegistrationError;
use inexor_rgf_behaviour_model_api::BehaviourTypeId;
use inexor_rgf_behaviour_model_api::ComponentBehaviourTypeId;
use inexor_rgf_behaviour_model_api::EntityBehaviourTypeId;
use inexor_rgf_graph::ComponentTypeId;
use inexor_rgf_graph::EntityInstance;
use inexor_rgf_graph::EntityTypeId;
use inexor_rgf_graph::Mutability;
use inexor_rgf_graph::PropertyInstances;
use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_reactive_model_impl::ReactiveEntity;

#[injectable]
#[async_trait]
pub trait ReactiveEntityManager: Send + Sync + Lifecycle {
    /// Returns true, if an entity instance exists with the given UUID.
    fn has(&self, id: Uuid) -> bool;

    /// Returns the reactive entity instance with the given UUID or None.
    fn get(&self, id: Uuid) -> Option<ReactiveEntity>;

    /// Returns the reactive entity instance that matches the given label or None.
    fn get_by_label(&self, label: &str) -> Option<ReactiveEntity>;

    /// Returns the reactive entity instance and the matched path parameters that matches the given label or None.
    /// /org/inexor/local/users/:user_id
    /// /org/inexor/local/users/PeterPenacka returns: (instance, {"user_id": "PeterPenacka"})
    fn get_by_label_with_params(&self, label: &str) -> Option<(ReactiveEntity, HashMap<String, String>)>;

    /// Returns all registered reactive entity instances.
    fn get_all(&self) -> Vec<ReactiveEntity>;

    /// Returns all reactive entity instances of the given type.
    fn get_by_type(&self, ty: &EntityTypeId) -> Vec<ReactiveEntity>;

    /// Returns all reactive entity instances of the given type which are of the given component..
    fn get_by_component(&self, component_ty: &ComponentTypeId) -> Vec<ReactiveEntity>;

    /// Returns all reactive entity instances of the given type which behaves as the given behaviour.
    fn get_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) -> Vec<ReactiveEntity>;

    /// Returns all reactive entity instances of the given namespace.
    fn get_by_namespace(&self, namespace: &str) -> Vec<ReactiveEntity>;

    /// Returns the ids of all registered reactive entity instances.
    fn get_ids(&self) -> Vec<Uuid>;

    /// Returns the count of registered reactive entity instances.
    fn count(&self) -> usize;

    /// Returns the count of registered reactive entity instances of the given type.
    fn count_by_type(&self, ty: &EntityTypeId) -> usize;

    /// Returns the count of registered reactive entity instances which are of the given component.
    fn count_by_component(&self, component_ty: &ComponentTypeId) -> usize;

    /// Returns the count of registered reactive entity instances which behaves as the given behaviour.
    fn count_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) -> usize;

    /// Creates a new reactive entity instance of the given type. The reactive instance will be
    /// initialized with the given properties and values. A random id will be generated.
    fn create_reactive_entity(&self, ty: &EntityTypeId, properties: PropertyInstances) -> Result<ReactiveEntity, ReactiveEntityCreationError>;

    /// Creates a new reactive entity instance of the given type, with the given id and initialized
    /// with the given properties and values.
    fn create_with_id(&self, ty: &EntityTypeId, id: Uuid, properties: PropertyInstances) -> Result<ReactiveEntity, ReactiveEntityCreationError>;

    /// Creates a reactive entity instance from the given non-reactive entity instance. The
    /// reactive entity instance will be registered.
    fn create_reactive_instance(&self, entity_instance: EntityInstance) -> Result<ReactiveEntity, ReactiveEntityCreationError>;

    /// Registers a reactive entity instance and applies components and behaviours.
    fn register_reactive_instance(&self, reactive_entity: ReactiveEntity) -> Result<ReactiveEntity, ReactiveEntityRegistrationError>;

    /// Registers a reactive entity instance if and only if the given instance doesn't exist.
    ///
    /// No properties are merged if the given entity instance already exists.
    fn register_or_merge_reactive_instance(&self, reactive_entity: ReactiveEntity) -> Result<ReactiveEntity, ReactiveEntityRegistrationError>;

    /// Adds the component with the given name to the entity instance with the given id.
    fn add_component(&self, id: Uuid, component_ty: &ComponentTypeId) -> Result<(), ReactiveEntityComponentAddError>;

    /// Removes the component with the given name from the entity instance with the given id.
    fn remove_component(&self, id: Uuid, component_ty: &ComponentTypeId);

    /// Adds the property with the given name and initial value to the entity instance with the given id.
    fn add_property(&self, id: Uuid, property_name: &str, mutability: Mutability, value: Value) -> Result<(), ReactiveEntityPropertyAddError>;

    /// Removes the property with the given name from the entity instance with the given id.
    fn remove_property(&self, id: Uuid, property_name: &str) -> Result<(), ReactiveEntityPropertyRemoveError>;

    /// Adds the given behaviour to all instances of the given entity type.
    fn add_behaviour_to_all_entity_instances(&self, entity_behaviour_ty: &EntityBehaviourTypeId);

    /// Adds the given behaviour to all instances of the given entity type.
    fn add_behaviour_to_all_entity_components(&self, component_behaviour_ty: &ComponentBehaviourTypeId);

    fn delete(&self, id: Uuid) -> bool;

    // TODO: fn delete_and_delete_relations(&self, id: Uuid);

    /// Unregisters the reactive entity instance. Also removes all behaviours. If there are any
    /// references to the reactive entity instance, their reactive streams still work but the
    /// applied behaviours are gone.
    fn unregister_reactive_instance(&self, id: Uuid) -> bool;

    // TODO: rename import_from_file
    // fn import(&self, path: &str) -> Result<ReactiveEntity, ReactiveEntityImportError>;

    // TODO: import_from_json_string
    // Goal: web-ui: upload entity instance as json file

    // TODO: return result
    // TODO: rename export_as_file
    // fn export(&self, id: Uuid, path: &str);

    // TODO: implement export_as_json_string
    // Goal: web-ui: download entity instance as json file
    // fn export_as_json_string(&self, id: Uuid) -> String;

    fn handle_component_added_events(&self);

    fn handle_component_removed_events(&self);

    fn handle_property_added_events(&self);

    fn handle_property_removed_events(&self);
}
