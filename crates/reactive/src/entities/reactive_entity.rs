use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Deref;
use std::sync::Arc;

use serde_json::Map;
use serde_json::Value;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use inexor_rgf_reactive_api::prelude::*;

use crate::behaviour_api::BehaviourTypeId;
use crate::behaviour_api::BehaviourTypeIds;
use crate::model::Component;
use crate::model::ComponentTypeId;
use crate::model::ComponentTypeIds;
use crate::model::EntityInstance;
use crate::model::EntityType;
use crate::model::EntityTypeId;
use crate::model::Extensions;
use crate::model::Mutability;
use crate::model::Mutability::Mutable;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::PropertyInstances;
use crate::model::PropertyType;
use crate::model::TypeDefinition;
use crate::model::TypeDefinitionGetter;
use crate::BehaviourTypesContainer;
use crate::ReactiveProperties;
use crate::ReactiveProperty;
use crate::ReactivePropertyContainer;

#[derive(TypedBuilder)]
#[builder(
    build_method(vis="pub", into=ReactiveEntity),
    builder_method(vis=""),
    builder_type(vis="pub", name=ReactiveEntityInstanceBuilder),
)]
pub struct ReactiveEntityInstance {
    /// The type definition of the entity type.
    #[builder(setter(into))]
    pub ty: EntityTypeId,

    /// The unique identifier of the entity instance.
    #[builder(default=Uuid::new_v4())]
    pub id: Uuid,

    /// An optional description of the entity instance.
    #[builder(default, setter(into))]
    pub description: String,

    /// The reactive properties.
    #[builder(default, setter(into))]
    pub properties: ReactiveProperties<Uuid>,

    /// The names of the components which are applied on this entity instance.
    #[builder(default, setter(into))]
    pub components: ComponentTypeIds,

    /// The names of the behaviours which are applied on this entity instance.
    #[builder(default, setter(into))]
    pub behaviours: BehaviourTypeIds,
}

#[derive(Clone)]
pub struct ReactiveEntity(Arc<ReactiveEntityInstance>);

impl ReactiveEntity {
    pub fn builder() -> ReactiveEntityInstanceBuilder {
        ReactiveEntityInstance::builder()
    }

    /// Creates a builder from the given entity type.
    /// Generates an id for the reactive entity.
    /// Converts property types into reactive properties and initializes the properties with default values.
    pub fn builder_from_entity_type(
        entity_type: &EntityType,
    ) -> ReactiveEntityInstanceBuilder<((EntityTypeId,), (Uuid,), (), (ReactiveProperties<Uuid>,), (), ())> {
        let id = Uuid::new_v4();
        let properties = PropertyInstances::new_from_property_types_with_defaults(&entity_type.properties);
        let reactive_properties = ReactiveProperties::new_with_id_from_properties(id, properties);
        ReactiveEntity::builder().ty(&entity_type.ty).id(id).properties(reactive_properties)
    }
}

impl Deref for ReactiveEntity {
    type Target = Arc<ReactiveEntityInstance>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ReactiveInstance<Uuid> for ReactiveEntity {
    fn id(&self) -> Uuid {
        self.id
    }
}

impl ReactivePropertyContainer for ReactiveEntity {
    fn tick_checked(&self) {
        for property in self.properties.iter() {
            property.tick_checked();
        }
    }

    fn tick(&self) {
        for property in self.properties.iter() {
            property.tick();
        }
    }

    fn has_property(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }

    fn add_property<S: Into<String>>(&self, name: S, mutability: Mutability, value: Value) {
        let name = name.into();
        if !self.properties.contains_key(&name) {
            let property_instance = ReactiveProperty::new(self.id, name.clone(), mutability, value);
            self.properties.insert(name, property_instance);
        }
    }

    fn add_property_by_type(&self, property: &PropertyType) {
        let property_instance = ReactiveProperty::new(self.id, &property.name, property.mutability, property.data_type.default_value());
        self.properties.insert(property.name.clone(), property_instance);
    }

    fn remove_property<S: Into<String>>(&self, name: S) {
        let name = name.into();
        self.properties.retain(|property_name, _| property_name != &name);
    }

    fn observe_with_handle<F>(&self, name: &str, subscriber: F, handle_id: u128)
    where
        F: FnMut(&Value) + 'static + Send,
    {
        if let Some(property_instance) = self.properties.get(name) {
            property_instance.stream.read().unwrap().observe_with_handle(subscriber, handle_id);
        }
    }

    fn remove_observer(&self, name: &str, handle_id: u128) {
        if let Some(property_instance) = self.properties.get(name) {
            property_instance.stream.read().unwrap().remove(handle_id);
        }
    }

    fn remove_observers(&self, name: &str) {
        if let Some(property_instance) = self.properties.get(name) {
            property_instance.stream.read().unwrap().clear();
        }
    }

    fn remove_all_observers(&self) {
        for property_instance in self.properties.iter() {
            property_instance.stream.read().unwrap().clear();
        }
    }
}

impl ComponentContainer for ReactiveEntity {
    fn get_components(&self) -> ComponentTypeIds {
        self.components.clone()
        // self.components.iter().map(|c| c.key().clone()).collect()
    }

    fn add_component(&self, ty: ComponentTypeId) {
        self.components.insert(ty);
    }

    fn add_component_with_properties(&self, component: &Component) {
        self.add_component(component.ty.clone());
        for property_type in component.properties.iter() {
            if !self.properties.contains_key(&property_type.name) {
                self.add_property_by_type(&property_type);
            }
        }
    }

    fn remove_component(&self, ty: &ComponentTypeId) {
        self.components.remove(ty);
    }

    fn is_a(&self, ty: &ComponentTypeId) -> bool {
        self.components.contains(ty)
    }
}

impl BehaviourTypesContainer for ReactiveEntity {
    fn get_behaviours(&self) -> Vec<BehaviourTypeId> {
        self.behaviours.iter().map(|b| b.key().clone()).collect()
    }

    fn add_behaviour(&self, ty: BehaviourTypeId) {
        self.behaviours.insert(ty);
    }

    fn remove_behaviour(&self, ty: &BehaviourTypeId) {
        self.behaviours.remove(ty);
    }

    fn behaves_as(&self, ty: &BehaviourTypeId) -> bool {
        self.behaviours.contains(ty)
    }
}

impl PropertyInstanceGetter for ReactiveEntity {
    fn get<S: Into<String>>(&self, property_name: S) -> Option<Value> {
        self.properties.get(&property_name.into()).map(|p| p.get())
    }

    fn as_bool<S: Into<String>>(&self, property_name: S) -> Option<bool> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_bool())
    }

    fn as_u64<S: Into<String>>(&self, property_name: S) -> Option<u64> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_u64())
    }

    fn as_i64<S: Into<String>>(&self, property_name: S) -> Option<i64> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_i64())
    }

    fn as_f64<S: Into<String>>(&self, property_name: S) -> Option<f64> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_f64())
    }

    fn as_string<S: Into<String>>(&self, property_name: S) -> Option<String> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_string())
    }

    fn as_array<S: Into<String>>(&self, property_name: S) -> Option<Vec<Value>> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_array())
    }

    fn as_object<S: Into<String>>(&self, property_name: S) -> Option<Map<String, Value>> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_object())
    }
}

impl PropertyInstanceSetter for ReactiveEntity {
    fn set_checked<S: Into<String>>(&self, property_name: S, value: Value) {
        if let Some(instance) = self.properties.get(&property_name.into()) {
            instance.set_checked(value);
        }
    }

    fn set<S: Into<String>>(&self, property_name: S, value: Value) {
        if let Some(instance) = self.properties.get(&property_name.into()) {
            instance.set(value);
        }
    }

    fn set_no_propagate_checked<S: Into<String>>(&self, property_name: S, value: Value) {
        if let Some(instance) = self.properties.get(&property_name.into()) {
            instance.set_no_propagate_checked(value);
        }
    }

    fn set_no_propagate<S: Into<String>>(&self, property_name: S, value: Value) {
        if let Some(instance) = self.properties.get(&property_name.into()) {
            instance.set_no_propagate(value);
        }
    }

    fn mutability<S: Into<String>>(&self, property_name: S) -> Option<Mutability> {
        self.properties.get(&property_name.into()).map(|p| p.value().mutability)
    }

    fn set_mutability<S: Into<String>>(&self, property_name: S, mutability: Mutability) {
        if let Some(mut property_instance) = self.properties.get_mut(&property_name.into()) {
            property_instance.set_mutability(mutability);
        }
    }

    // TODO: fn set(&self, Map<String, Value>
    // TODO: Set values transactional: first set all values internally, then send all affected streams
}

impl NamespacedTypeGetter for ReactiveEntity {
    fn namespace(&self) -> String {
        self.ty.namespace()
    }

    fn type_name(&self) -> String {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for ReactiveEntity {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
    }
}

impl Display for ReactiveEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}__{}", &self.ty, self.id)
    }
}

impl From<ReactiveEntity> for EntityInstance {
    fn from(entity: ReactiveEntity) -> Self {
        EntityInstance {
            ty: entity.ty.clone(),
            id: entity.id,
            description: entity.description.clone(),
            properties: PropertyInstances::from(&entity.properties),
            extensions: Extensions::new(),
        }
    }
}

impl From<&ReactiveEntity> for EntityInstance {
    fn from(entity: &ReactiveEntity) -> Self {
        EntityInstance {
            ty: entity.ty.clone(),
            id: entity.id,
            description: entity.description.clone(),
            properties: PropertyInstances::from(&entity.properties),
            extensions: Extensions::new(),
        }
    }
}

impl From<ReactiveEntityInstance> for ReactiveEntity {
    fn from(reactive_entity: ReactiveEntityInstance) -> Self {
        ReactiveEntity(Arc::new(reactive_entity))
    }
}

impl From<EntityInstance> for ReactiveEntity {
    fn from(instance: EntityInstance) -> Self {
        let properties: ReactiveProperties<Uuid> = instance
            .properties
            .iter()
            .map(|property| ReactiveProperty::new(instance.id, property.key(), Mutable, property.value().clone()))
            .collect();

        let entity_instance = ReactiveEntityInstance {
            ty: instance.ty.clone(),
            id: instance.id,
            description: instance.description,
            properties,
            components: ComponentTypeIds::new(),
            behaviours: BehaviourTypeIds::new(),
        };
        entity_instance.into()
    }
}
