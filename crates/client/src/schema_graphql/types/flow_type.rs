use crate::schema_graphql::instances::entity_instance::EntityInstance;
use crate::schema_graphql::instances::relation_instance::RelationInstance;
use crate::schema_graphql::types::extension::Extension;
use crate::schema_graphql::types::extension::Extensions;
use crate::schema_graphql::types::property_type::PropertyType;
use crate::schema_graphql::types::property_type::PropertyTypes;
use reactive_graph_graph::NamespacedTypeGetter;
use std::ops::Deref;

#[derive(cynic::InputObject, Clone, Debug)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-schema.graphql",
    schema_module = "crate::schema_graphql::schema"
)]
pub struct FlowTypeId {
    pub name: String,
    pub namespace: String,
}

impl From<reactive_graph_graph::FlowTypeId> for FlowTypeId {
    fn from(ty: reactive_graph_graph::FlowTypeId) -> Self {
        FlowTypeId {
            name: ty.type_name(),
            namespace: ty.namespace(),
        }
    }
}

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-schema.graphql",
    schema_module = "crate::schema_graphql::schema"
)]
pub struct FlowType {
    /// The namespace of the flow type.
    pub namespace: String,

    /// The type name of the flow type.
    pub name: String,

    /// Textual description of the flow type.
    pub description: String,

    /// The wrapper entity instance.
    pub wrapper_entity_instance: EntityInstance,

    /// The contained entity instances.
    pub entity_instances: Vec<EntityInstance>,

    /// The contained relation instances.
    pub relation_instances: Vec<RelationInstance>,

    /// The variables of the flow type.
    pub variables: Vec<PropertyType>,

    /// The extensions.
    pub extensions: Vec<Extension>,
}

impl From<FlowType> for reactive_graph_graph::FlowType {
    fn from(flow_type: FlowType) -> Self {
        let ty = reactive_graph_graph::FlowTypeId::new_from_type(flow_type.namespace, flow_type.name);
        reactive_graph_graph::FlowType {
            ty,
            description: flow_type.description,
            wrapper_entity_instance: flow_type.wrapper_entity_instance.into(),
            entity_instances: flow_type
                .entity_instances
                .into_iter()
                .map(|entity_instance: EntityInstance| entity_instance.into())
                .collect(),
            relation_instances: flow_type
                .relation_instances
                .into_iter()
                .map(|relation_instance| relation_instance.into())
                .collect(),
            variables: PropertyTypes(flow_type.variables).into(),
            extensions: Extensions(flow_type.extensions).into(),
        }
    }
}

pub struct FlowTypes(pub Vec<FlowType>);

impl Deref for FlowTypes {
    type Target = Vec<FlowType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<FlowTypes> for Vec<reactive_graph_graph::FlowType> {
    fn from(flow_types: FlowTypes) -> Self {
        flow_types.0.into_iter().map(From::from).collect()
    }
}
