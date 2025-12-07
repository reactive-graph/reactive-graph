use crate::schema_graphql::instances::entity_instance::EntityInstance;
use crate::schema_graphql::instances::entity_instance::EntityInstances;
use crate::schema_graphql::instances::relation_instance::RelationInstance;
use crate::schema_graphql::instances::relation_instance::RelationInstances;
use crate::schema_graphql::types::extension::Extension;
use crate::schema_graphql::types::extension::Extensions;
use crate::schema_graphql::types::property_type::PropertyType;
use crate::schema_graphql::types::property_type::PropertyTypes;
use reactive_graph_graph::FlowTypeId;
use reactive_graph_graph::InvalidFlowTypeError;
use reactive_graph_graph::NamespacedTypeParseError;
use serde_json::Value;
use std::ops::Deref;
use std::str::FromStr;

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-schema.graphql",
    schema_module = "crate::schema_graphql::schema"
)]
pub struct FlowType {
    /// The fully qualified namespace of the flow type.
    #[cynic(rename = "type")]
    pub _type: String,

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

    /// The JSON schema.
    pub json_schema: Value,
}

impl FlowType {
    pub fn ty(&self) -> Result<FlowTypeId, NamespacedTypeParseError> {
        FlowTypeId::from_str(&self._type)
    }
}

impl TryFrom<FlowType> for reactive_graph_graph::FlowType {
    type Error = InvalidFlowTypeError;

    fn try_from(flow_type: FlowType) -> Result<Self, Self::Error> {
        Ok(reactive_graph_graph::FlowType {
            ty: flow_type.ty().map_err(InvalidFlowTypeError::InvalidFlowType)?,
            description: flow_type.description,
            wrapper_entity_instance: reactive_graph_graph::EntityInstance::try_from(flow_type.wrapper_entity_instance)
                .map_err(InvalidFlowTypeError::InvalidEntityInstance)?,
            entity_instances: reactive_graph_graph::EntityInstances::try_from(EntityInstances(flow_type.entity_instances))
                .map_err(InvalidFlowTypeError::InvalidEntityInstance)?,
            relation_instances: reactive_graph_graph::RelationInstances::try_from(RelationInstances(flow_type.relation_instances))
                .map_err(InvalidFlowTypeError::InvalidRelationInstance)?,
            variables: reactive_graph_graph::PropertyTypes::try_from(PropertyTypes(flow_type.variables)).map_err(InvalidFlowTypeError::InvalidVariable)?,
            extensions: reactive_graph_graph::Extensions::try_from(Extensions(flow_type.extensions)).map_err(InvalidFlowTypeError::InvalidExtension)?,
        })
    }
}

pub struct FlowTypes(pub Vec<FlowType>);

impl Deref for FlowTypes {
    type Target = Vec<FlowType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<FlowTypes> for reactive_graph_graph::FlowTypes {
    type Error = InvalidFlowTypeError;

    fn try_from(flow_types: FlowTypes) -> Result<Self, Self::Error> {
        let flow_types_2 = reactive_graph_graph::FlowTypes::new();
        for flow_type in flow_types.0 {
            flow_types_2.push(reactive_graph_graph::FlowType::try_from(flow_type)?);
        }
        Ok(flow_types_2)
    }
}
