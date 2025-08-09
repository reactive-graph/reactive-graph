use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Deref;
use std::sync::Arc;

use dashmap::DashMap;
use serde::Serialize;
use serde::Serializer;
use serde_json::Map;
use serde_json::Value;
use typed_builder::TypedBuilder;

use crate::ReactiveEntity;
use crate::ReactiveProperties;
use crate::ReactiveProperty;
use reactive_graph_behaviour_model_api::BehaviourTypeId;
use reactive_graph_behaviour_model_api::BehaviourTypeIds;
use reactive_graph_behaviour_model_api::BehaviourTypesContainer;
use reactive_graph_graph::Component;
use reactive_graph_graph::ComponentContainer;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::ComponentTypeIds;
use reactive_graph_graph::Extensions;
use reactive_graph_graph::InstanceId;
use reactive_graph_graph::JsonSchemaId;
use reactive_graph_graph::Mutability;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::NamespaceSegment;
use reactive_graph_graph::NamespacedType;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::PropertyInstanceGetter;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_graph::PropertyInstances;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypes;
use reactive_graph_graph::RelationInstance;
use reactive_graph_graph::RelationInstanceId;
use reactive_graph_graph::RelationInstanceTypeId;
use reactive_graph_graph::RelationType;
use reactive_graph_graph::RelationTypeId;
use reactive_graph_graph::TypeDefinition;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_graph::TypeIdType;
use reactive_graph_graph::instances::named::NamedInstanceContainer;
use reactive_graph_reactive_model_api::ReactiveInstance;
use reactive_graph_reactive_model_api::ReactivePropertyContainer;

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
#[derive(TypedBuilder)]
#[builder(
    build_method(vis="pub", into=ReactiveRelation),
    builder_method(vis = ""),
    builder_type(vis="pub", name=ReactiveRelationInstanceBuilder),
)]
pub struct ReactiveRelationInstance {
    // Possible optimization: Final field id would speed up getting the id
    // pub id: RelationInstanceId,
    /// The outbound entity instance.
    pub outbound: ReactiveEntity,

    /// The type definition of the relation type.
    #[builder(setter(into))]
    pub ty: RelationInstanceTypeId,

    /// The outbound entity instance.
    pub inbound: ReactiveEntity,

    /// The name of the relation instance.
    #[builder(default, setter(into))]
    pub name: String,

    /// Textual description of the relation instance.
    #[builder(default, setter(into))]
    pub description: String,

    /// The reactive properties.
    #[builder(default, setter(into))]
    pub properties: ReactiveProperties<RelationInstanceId>,

    /// The names of the components which are applied on this relation instance.
    #[builder(default, setter(into))]
    pub components: ComponentTypeIds,

    /// The names of the behaviours which are applied on this relation instance.
    #[builder(default, setter(into))]
    pub behaviours: BehaviourTypeIds,
}

#[allow(clippy::result_unit_err)]
impl ReactiveRelationInstance {
    pub fn new_from_properties<T: Into<RelationInstanceTypeId>, P: Into<PropertyInstances>>(
        outbound: ReactiveEntity,
        ty: T,
        inbound: ReactiveEntity,
        properties: P,
    ) -> ReactiveRelationInstance {
        let ty = ty.into();
        let id = RelationInstanceId::new(outbound.id, ty.clone(), inbound.id);
        let properties = ReactiveProperties::new_with_id_from_properties(id.clone(), properties.into());
        ReactiveRelationInstance {
            outbound,
            ty,
            inbound,
            name: String::new(),
            description: String::new(),
            properties,
            components: ComponentTypeIds::new(),
            behaviours: BehaviourTypeIds::new(),
        }
    }

    pub fn new_from_instance(outbound: ReactiveEntity, inbound: ReactiveEntity, instance: RelationInstance) -> ReactiveRelationInstance {
        let id = instance.id();
        let properties = ReactiveProperties::new_with_id_from_properties(id.clone(), instance.properties);
        // let properties = instance.properties;
        // ::<RelationInstanceId>
        // let properties = instance
        //     .properties
        //     .iter()
        //     // TODO: mutability
        //     .map(|(name, value)| (name.clone(), ReactiveProperty::new(instance.ty.clone(), name.clone(), Mutable, value.clone())))
        //     .collect();
        ReactiveRelationInstance {
            outbound,
            ty: instance.ty,
            inbound,
            name: instance.name,
            description: instance.description,
            properties,
            components: ComponentTypeIds::new(),
            behaviours: BehaviourTypeIds::new(),
        }
    }

    /// Returns the inner relation type id.
    pub fn relation_type_id(&self) -> RelationTypeId {
        self.ty.relation_type_id()
    }

    /// Returns the relation instance type id.
    pub fn instance_id(&self) -> InstanceId {
        self.ty.instance_id()
    }
}

#[derive(Clone)]
pub struct ReactiveRelation(Arc<ReactiveRelationInstance>);

impl ReactiveRelation {
    pub fn builder() -> ReactiveRelationInstanceBuilder {
        ReactiveRelationInstance::builder()
    }

    #[allow(clippy::type_complexity)]
    pub fn builder_with_entities(
        outbound: ReactiveEntity,
        ty: &RelationInstanceTypeId,
        inbound: ReactiveEntity,
    ) -> ReactiveRelationInstanceBuilder<((ReactiveEntity,), (RelationInstanceTypeId,), (ReactiveEntity,), (), (), (), (), ())> {
        ReactiveRelation::builder().outbound(outbound).ty(ty).inbound(inbound)
    }

    /// Creates a builder for the given relation instance type id.
    /// Generates an id for the reactive relation.
    /// Converts property types into reactive properties and initializes the properties with default values.
    #[allow(clippy::type_complexity)]
    pub fn builder_with_entities_and_properties(
        outbound: ReactiveEntity,
        ty: &RelationInstanceTypeId,
        inbound: ReactiveEntity,
        properties: &PropertyTypes,
    ) -> ReactiveRelationInstanceBuilder<(
        (ReactiveEntity,),
        (RelationInstanceTypeId,),
        (ReactiveEntity,),
        (),
        (),
        (ReactiveProperties<RelationInstanceId>,),
        (),
        (),
    )> {
        let id = RelationInstanceId::new(outbound.id, ty, inbound.id);
        let properties = PropertyInstances::new_from_property_types_with_defaults(properties);
        let reactive_properties: ReactiveProperties<RelationInstanceId> = ReactiveProperties::new_with_id_from_properties(id, properties);
        ReactiveRelation::builder_with_entities(outbound, ty, inbound).properties(reactive_properties)
    }

    #[allow(clippy::type_complexity)]
    pub fn builder_from_type_with_unique_id(
        outbound: ReactiveEntity,
        relation_type: &RelationType,
        inbound: ReactiveEntity,
    ) -> ReactiveRelationInstanceBuilder<(
        (ReactiveEntity,),
        (RelationInstanceTypeId,),
        (ReactiveEntity,),
        (),
        (),
        (ReactiveProperties<RelationInstanceId>,),
        (),
        (),
    )> {
        let ty = RelationInstanceTypeId::new_singleton(&relation_type.ty);
        ReactiveRelation::builder_with_entities_and_properties(outbound, &ty, inbound, &relation_type.properties)
    }

    #[allow(clippy::type_complexity)]
    pub fn builder_from_type_with_unique_instance_id(
        outbound: ReactiveEntity,
        relation_type: &RelationType,
        instance_id: InstanceId,
        inbound: ReactiveEntity,
    ) -> ReactiveRelationInstanceBuilder<(
        (ReactiveEntity,),
        (RelationInstanceTypeId,),
        (ReactiveEntity,),
        (),
        (),
        (ReactiveProperties<RelationInstanceId>,),
        (),
        (),
    )> {
        let ty = RelationInstanceTypeId::new(&relation_type.ty, instance_id);
        ReactiveRelation::builder_with_entities_and_properties(outbound, &ty, inbound, &relation_type.properties)
    }

    #[allow(clippy::type_complexity)]
    pub fn builder_from_type_with_random_instance_id(
        outbound: ReactiveEntity,
        relation_type: &RelationType,
        inbound: ReactiveEntity,
    ) -> ReactiveRelationInstanceBuilder<(
        (ReactiveEntity,),
        (RelationInstanceTypeId,),
        (ReactiveEntity,),
        (),
        (),
        (ReactiveProperties<RelationInstanceId>,),
        (),
        (),
    )> {
        let ty = RelationInstanceTypeId::new_with_random_instance_id(&relation_type.ty);
        ReactiveRelation::builder_with_entities_and_properties(outbound, &ty, inbound, &relation_type.properties)
    }
    // }
    //
    // impl ReactiveRelation {
    pub fn new_from_properties(
        outbound: ReactiveEntity,
        ty: RelationInstanceTypeId,
        inbound: ReactiveEntity,
        properties: DashMap<String, Value>,
    ) -> ReactiveRelation {
        ReactiveRelationInstance::new_from_properties(outbound, ty, inbound, properties).into()
    }

    pub fn new_from_instance(outbound: ReactiveEntity, inbound: ReactiveEntity, instance: RelationInstance) -> ReactiveRelation {
        ReactiveRelationInstance::new_from_instance(outbound, inbound, instance).into()
    }
}

impl NamedInstanceContainer for ReactiveRelation {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }
}

impl Deref for ReactiveRelation {
    type Target = Arc<ReactiveRelationInstance>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ReactiveInstance<RelationInstanceId> for ReactiveRelation {
    /// Returns the relation instance id.
    fn id(&self) -> RelationInstanceId {
        RelationInstanceId::new(self.outbound.id, self.ty.clone(), self.inbound.id)
    }
}

impl ReactivePropertyContainer for ReactiveRelation {
    fn tick_checked(&self) {
        for property_instance in self.properties.iter() {
            property_instance.tick_checked();
        }
    }

    fn tick(&self) {
        for property_instance in self.properties.iter() {
            property_instance.tick();
        }
    }

    fn has_property(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }

    fn add_property<S: Into<String>>(&self, name: S, mutability: Mutability, value: Value) {
        let name = name.into();
        let id = self.id();
        if !self.properties.contains_key(name.as_str()) {
            let property_instance = ReactiveProperty::new(id.clone(), name.clone(), mutability, value);
            self.properties.insert(name, property_instance);
        }
    }

    fn add_property_by_type(&self, property: &PropertyType) {
        let property_instance = ReactiveProperty::new(self.id().clone(), &property.name, property.mutability, property.data_type.default_value());
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
        for property_instance in self.properties.iter() {
            property_instance.stream.read().unwrap().clear();
        }
    }
}

impl ComponentContainer for ReactiveRelation {
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

impl BehaviourTypesContainer for ReactiveRelation {
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

impl PropertyInstanceGetter for ReactiveRelation {
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

impl PropertyInstanceSetter for ReactiveRelation {
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

impl NamespacedTypeGetter for ReactiveRelation {
    fn namespaced_type(&self) -> NamespacedType {
        self.ty.namespaced_type()
    }

    fn namespace(&self) -> Namespace {
        self.ty.namespace()
    }

    fn path(&self) -> Namespace {
        self.ty.path()
    }

    fn type_name(&self) -> NamespaceSegment {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for ReactiveRelation {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
    }

    fn type_id_type() -> TypeIdType {
        TypeIdType::RelationType
    }
}

impl TypeDefinitionGetter for &ReactiveRelation {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
    }

    fn type_id_type() -> TypeIdType {
        TypeIdType::RelationType
    }
}

impl Display for ReactiveRelation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}--[{}]-->{}", self.outbound.id, &self.ty, self.inbound.id)
    }
}

impl From<ReactiveRelation> for RelationInstance {
    fn from(relation: ReactiveRelation) -> Self {
        RelationInstance {
            outbound_id: relation.outbound.id,
            ty: relation.ty.clone(),
            inbound_id: relation.inbound.id,
            name: relation.name.clone(),
            description: relation.description.clone(),
            properties: PropertyInstances::from(&relation.properties),
            components: relation.components.clone(),
            extensions: Extensions::new(),
        }
    }
}

impl From<&ReactiveRelation> for RelationInstance {
    fn from(relation: &ReactiveRelation) -> Self {
        RelationInstance {
            outbound_id: relation.outbound.id,
            ty: relation.ty.clone(),
            inbound_id: relation.inbound.id,
            name: relation.name.clone(),
            description: relation.description.clone(),
            properties: PropertyInstances::from(&relation.properties),
            components: relation.components.clone(),
            extensions: Extensions::new(),
        }
    }
}

impl From<ReactiveRelationInstance> for ReactiveRelation {
    fn from(relation_instance: ReactiveRelationInstance) -> Self {
        ReactiveRelation(Arc::new(relation_instance))
    }
}

impl Serialize for ReactiveRelation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let property_instances = PropertyInstances::from(&self.properties);
        property_instances.insert("$id".to_string(), JsonSchemaId::from(&self).into());
        property_instances.insert("outbound_id".to_string(), Value::String(self.outbound.id.to_string()));
        property_instances.insert("instance_id".to_string(), Value::String(self.instance_id().to_string()));
        property_instances.insert("inbound_id".to_string(), Value::String(self.inbound.id.to_string()));
        serializer.collect_map(property_instances)
    }
}
