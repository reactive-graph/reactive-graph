use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::sync::Arc;

use dashmap::DashMap;
use dashmap::DashSet;
use indradb::EdgeKey;
use indradb::EdgeProperties;
use serde_json::Map;
use serde_json::Value;
use uuid::Uuid;

use crate::BehaviourTypeId;
use crate::Component;
use crate::ComponentContainer;
use crate::ComponentTypeId;
use crate::Mutability;
use crate::Mutability::Mutable;
use crate::NamespacedTypeGetter;
use crate::PropertyInstanceGetter;
use crate::PropertyInstanceSetter;
use crate::PropertyType;
use crate::ReactiveBehaviourContainer;
use crate::ReactiveEntityInstance;
use crate::ReactiveInstance;
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
    pub behaviours: DashSet<BehaviourTypeId>,
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
                        Mutable,
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
            // TODO: mutability
            .map(|(name, value)| (name.clone(), ReactivePropertyInstance::new(Uuid::new_v4(), name.clone(), Mutable, value.clone())))
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
                        // TODO: mutability
                        Mutable,
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
    fn tick_checked(&self) {
        for property_instance in &self.properties {
            property_instance.tick_checked();
        }
    }

    fn tick(&self) {
        for property_instance in &self.properties {
            property_instance.tick();
        }
    }

    fn has_property(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }

    fn add_property<S: Into<String>>(&self, name: S, mutability: Mutability, value: Value) {
        let name = name.into();
        if !self.properties.contains_key(name.as_str()) {
            let property_instance = ReactivePropertyInstance::new(Uuid::new_v4(), name.clone(), mutability, value);
            self.properties.insert(name, property_instance);
        }
    }

    fn add_property_by_type(&self, property: &PropertyType) {
        let property_instance = ReactivePropertyInstance::new(Uuid::new_v4(), &property.name, property.mutability, property.data_type.default_value());
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
        if let Some(property) = self.properties.get(name) {
            property.stream.read().unwrap().observe_with_handle(subscriber, handle_id);
        }
    }

    fn remove_observer(&self, name: &str, handle_id: u128) {
        if let Some(property) = self.properties.get(name) {
            property.stream.read().unwrap().remove(handle_id);
        }
    }

    fn remove_observers(&self, name: &str) {
        if let Some(property_instance) = self.properties.get(name) {
            property_instance.stream.read().unwrap().clear();
        }
    }

    fn remove_all_observers(&self) {
        for property_instance in &self.properties {
            property_instance.stream.read().unwrap().clear();
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

impl Display for ReactiveRelationInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}--[{}]-->{}", self.outbound.id, &self.ty, self.inbound.id)
    }
}

impl ReactiveInstance for ReactiveRelationInstance {}

#[macro_export]
macro_rules! relation_model {
    (
        $ident: ident
        $(,
            $accessor_type: tt
            $(
            $accessor_name: ident
            $accessor_data_type: tt
            )?
        )*
        $(,)?
    ) => {
        // use $crate::PropertyInstanceGetter as RxPropertyInstanceGetter;
        // use $crate::PropertyInstanceSetter as RxPropertyInstanceSetter;
        pub struct $ident {
            i: std::sync::Arc<$crate::ReactiveRelationInstance>,
        }

        impl $ident {
            $(
                $crate::rx_accessor!(pub $accessor_type $($accessor_name $accessor_data_type)?);
            )*
        }

        impl $crate::ReactiveInstanceGetter<$crate::ReactiveRelationInstance> for $ident {
            fn get_reactive_instance(&self) -> &std::sync::Arc<$crate::ReactiveRelationInstance> {
                &self.i
            }
        }

        impl From<std::sync::Arc<$crate::ReactiveRelationInstance>> for $ident {
            fn from(i: std::sync::Arc<$crate::ReactiveRelationInstance>) -> Self {
                $ident { i }
            }
        }

        impl $crate::PropertyInstanceGetter for $ident {
            fn get<S: Into<String>>(&self, property_name: S) -> Option<serde_json::Value> {
                self.i.get(property_name)
            }

            fn as_bool<S: Into<String>>(&self, property_name: S) -> Option<bool> {
                self.i.as_bool(property_name)
            }

            fn as_u64<S: Into<String>>(&self, property_name: S) -> Option<u64> {
                self.i.as_u64(property_name)
            }

            fn as_i64<S: Into<String>>(&self, property_name: S) -> Option<i64> {
                self.i.as_i64(property_name)
            }

            fn as_f64<S: Into<String>>(&self, property_name: S) -> Option<f64> {
                self.i.as_f64(property_name)
            }

            fn as_string<S: Into<String>>(&self, property_name: S) -> Option<String> {
                self.i.as_string(property_name)
            }

            fn as_array<S: Into<String>>(&self, property_name: S) -> Option<Vec<serde_json::Value>> {
                self.i.as_array(property_name)
            }

            fn as_object<S: Into<String>>(&self, property_name: S) -> Option<serde_json::Map<String, serde_json::Value>> {
                self.i.as_object(property_name)
            }
        }

        impl $crate::PropertyInstanceSetter for $ident {
            fn set_checked<S: Into<String>>(&self, property_name: S, value: serde_json::Value) {
                self.i.set_checked(property_name, value);
            }

            fn set<S: Into<String>>(&self, property_name: S, value: serde_json::Value) {
                self.i.set(property_name, value);
            }

            fn set_no_propagate_checked<S: Into<String>>(&self, property_name: S, value: serde_json::Value) {
                self.i.set_no_propagate_checked(property_name, value);
            }

            fn set_no_propagate<S: Into<String>>(&self, property_name: S, value: serde_json::Value) {
                self.i.set_no_propagate(property_name, value);
            }

            fn mutability<S: Into<String>>(&self, property_name: S) -> Option<$crate::Mutability> {
                self.i.mutability(property_name)
            }

            fn set_mutability<S: Into<String>>(&self, property_name: S, mutability: $crate::Mutability) {
                self.i.set_mutability(property_name, mutability);
            }
        }

        impl $crate::NamespacedTypeGetter for $ident {
            fn namespace(&self) -> String {
                self.i.ty.namespace()
            }

            fn type_name(&self) -> String {
                self.i.ty.type_name()
            }
        }

        impl $crate::TypeDefinitionGetter for $ident {
            fn type_definition(&self) -> $crate::TypeDefinition {
                self.i.ty.type_definition()
            }
        }

        impl std::fmt::Display for $ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", &self.i)
            }
        }
    };
}
