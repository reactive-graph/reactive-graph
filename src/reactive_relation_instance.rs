use std::collections::HashMap;
use std::sync::Arc;

use dashmap::DashMap;
use dashmap::DashSet;
use indradb::EdgeKey;
use indradb::EdgeProperties;
use serde_json::Map;
use serde_json::Value;
use uuid::Uuid;

use crate::Component;
use crate::ComponentContainer;
use crate::ComponentTypeId;
use crate::NamespacedTypeGetter;
use crate::PropertyInstanceGetter;
use crate::PropertyInstanceSetter;
use crate::PropertyType;
use crate::ReactiveBehaviourContainer;
use crate::ReactiveEntityInstance;
use crate::ReactivePropertyContainer;
use crate::ReactivePropertyInstance;
use crate::RelationInstance;
use crate::RelationInstanceTypeId;
use crate::RelationTypeId;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;

/// Reactive instance of a relation in the directed property graph.
///
/// Property Graph: The relation instance can store properties.
///
/// Directed Graph: The direction of the relation point from the outbound
/// entity instance to the inbound entity instance.
///
/// Reactive Instance: The properties are streams with a local copies of
/// the last result. The streams can be connected, combined, folded or zipped.
///
/// One example for a directed reactive relation instance is a connector which
/// propagates changes on a property of the outbound entity to a property of
/// the inbound entity.
///
/// Another example would be the velocity transformation which are also using
/// the streams of the properties of the outbound entity, the inbound entity
/// and/or the relation itself.
///
/// Last but not least relation instances can be used for semantic
/// representations like the current camera of a player:
/// Player--(CurrentCamera)-->Camera
///
pub struct ReactiveRelationInstance {
    /// The outbound entity instance.
    pub outbound: Arc<ReactiveEntityInstance>,

    /// The type definition of the relation type.
    pub ty: RelationInstanceTypeId,

    /// The outbound entity instance.
    pub inbound: Arc<ReactiveEntityInstance>,

    /// An optional description of the relation instance.
    pub description: String,

    /// The reactive properties.
    pub properties: DashMap<String, ReactivePropertyInstance>,

    /// The names of the components which are applied on this relation instance.
    pub components: DashSet<ComponentTypeId>,

    /// The names of the behaviours which are applied on this relation instance.
    pub behaviours: DashSet<String>,
}

#[allow(clippy::result_unit_err)]
impl ReactiveRelationInstance {
    pub fn new_from_properties(
        outbound: Arc<ReactiveEntityInstance>,
        inbound: Arc<ReactiveEntityInstance>,
        properties: EdgeProperties,
    ) -> Result<ReactiveRelationInstance, ()> {
        let ty = RelationInstanceTypeId::try_from(&properties.edge.key.t)?;
        let properties = properties
            .props
            .iter()
            .map(|named_property| {
                (
                    named_property.name.to_string(),
                    ReactivePropertyInstance::new(
                        Uuid::new_v4(), // or generate a combined uuid from "outbound_id + type + inbound_id"
                        named_property.name.to_string(),
                        named_property.value.clone(),
                    ),
                )
            })
            .collect();
        Ok(ReactiveRelationInstance {
            outbound,
            ty,
            inbound,
            description: String::new(),
            properties,
            components: DashSet::new(),
            behaviours: DashSet::new(),
        })
    }

    pub fn new_from_instance(
        outbound: Arc<ReactiveEntityInstance>,
        inbound: Arc<ReactiveEntityInstance>,
        instance: RelationInstance,
    ) -> ReactiveRelationInstance {
        let properties = instance
            .properties
            .iter()
            .map(|(name, value)| (name.clone(), ReactivePropertyInstance::new(Uuid::new_v4(), name.clone(), value.clone())))
            .collect();
        ReactiveRelationInstance {
            outbound,
            ty: instance.ty,
            inbound,
            description: instance.description,
            properties,
            components: DashSet::new(),
            behaviours: DashSet::new(),
        }
    }

    pub fn new_from_type_with_properties<S: Into<String>>(
        namespace: S,
        outbound: Arc<ReactiveEntityInstance>,
        type_name: S,
        inbound: Arc<ReactiveEntityInstance>,
        properties: HashMap<String, Value>,
    ) -> ReactiveRelationInstance {
        let ty = RelationInstanceTypeId::new_from_type_unique_id(namespace, type_name);
        let properties = properties
            .iter()
            .map(|(name, value)| {
                (
                    name.clone(),
                    ReactivePropertyInstance::new(
                        Uuid::new_v4(), // or generate a combined uuid from "outbound_id + type + inbound_id"
                        name.clone(),
                        value.clone(),
                    ),
                )
            })
            .collect();
        ReactiveRelationInstance {
            outbound,
            ty,
            inbound,
            description: String::new(),
            properties,
            components: DashSet::new(),
            behaviours: DashSet::new(),
        }
    }

    /// Returns the inner relation type id.
    pub fn relation_type_id(&self) -> RelationTypeId {
        self.ty.relation_type_id()
    }

    /// Returns the relation instance type id.
    pub fn instance_id(&self) -> String {
        self.ty.instance_id()
    }

    pub fn get_key(&self) -> EdgeKey {
        EdgeKey::new(self.outbound.id, self.type_id(), self.inbound.id)
    }
}

impl ReactivePropertyContainer for ReactiveRelationInstance {
    fn tick(&self) {
        for property_instance in &self.properties {
            property_instance.tick();
        }
    }

    fn has_property(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }

    fn add_property<S: Into<String>>(&self, name: S, value: Value) {
        let name = name.into();
        if !self.properties.contains_key(name.as_str()) {
            let property_instance = ReactivePropertyInstance::new(Uuid::new_v4(), name.clone(), value);
            self.properties.insert(name, property_instance);
        }
    }

    fn add_property_by_type(&self, property: &PropertyType) {
        let property_instance = ReactivePropertyInstance::new(Uuid::new_v4(), &property.name, property.data_type.default_value());
        self.properties.insert(property.name.clone(), property_instance);
    }

    fn remove_property<S: Into<String>>(&self, name: S) {
        let name = name.into();
        self.properties.retain(|property_name, _| property_name != &name);
    }

    fn observe_with_handle<F>(&self, name: &str, subscriber: F, handle_id: u128)
    where
        F: FnMut(&Value) + 'static,
    {
        if let Some(property) = self.properties.get(name) {
            property.stream.read().unwrap().observe_with_handle(subscriber, handle_id);
        }
    }

    fn remove_observer(&self, name: &str, handle_id: u128) {
        if let Some(property) = self.properties.get(name) {
            property.stream.read().unwrap().remove(handle_id);
        }
    }
}

impl ComponentContainer for ReactiveRelationInstance {
    fn get_components(&self) -> Vec<ComponentTypeId> {
        self.components.iter().map(|c| c.key().clone()).collect()
    }

    fn add_component(&self, ty: ComponentTypeId) {
        self.components.insert(ty);
    }

    fn add_component_with_properties(&self, component: &Component) {
        self.add_component(component.ty.clone());
        for property_type in component.properties.iter() {
            if !self.properties.contains_key(&property_type.name) {
                self.add_property_by_type(property_type);
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

impl ReactiveBehaviourContainer for ReactiveRelationInstance {
    fn add_behaviour<S: Into<String>>(&self, behaviour: S) {
        self.behaviours.insert(behaviour.into());
    }

    fn remove_behaviour<S: Into<String>>(&self, behaviour: S) {
        self.behaviours.remove(behaviour.into().as_str());
    }

    fn behaves_as<S: Into<String>>(&self, behaviour: S) -> bool {
        self.behaviours.contains(behaviour.into().as_str())
    }
}

impl From<Arc<ReactiveRelationInstance>> for RelationInstance {
    fn from(instance: Arc<ReactiveRelationInstance>) -> Self {
        let properties = instance
            .properties
            .iter()
            .map(|property_instance| (property_instance.key().clone(), property_instance.get()))
            .collect();
        RelationInstance {
            outbound_id: instance.outbound.id,
            ty: instance.ty.clone(),
            inbound_id: instance.inbound.id,
            description: instance.description.clone(),
            properties,
            extensions: Vec::new(),
        }
    }
}

impl PropertyInstanceGetter for ReactiveRelationInstance {
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

impl PropertyInstanceSetter for ReactiveRelationInstance {
    fn set<S: Into<String>>(&self, property_name: S, value: Value) {
        if let Some(instance) = self.properties.get(&property_name.into()) {
            instance.set(value);
        }
    }

    fn set_no_propagate<S: Into<String>>(&self, property_name: S, value: Value) {
        if let Some(instance) = self.properties.get(&property_name.into()) {
            instance.set_no_propagate(value);
        }
    }
}

impl NamespacedTypeGetter for ReactiveRelationInstance {
    fn namespace(&self) -> String {
        self.ty.namespace()
    }

    fn type_name(&self) -> String {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for ReactiveRelationInstance {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
    }
}
