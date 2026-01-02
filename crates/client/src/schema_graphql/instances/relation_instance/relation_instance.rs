use crate::schema_graphql::instances::entity_instance::EntityInstance;
use crate::schema_graphql::instances::property_instance::PropertyInstance;
use crate::schema_graphql::types::component::Component;
use crate::schema_graphql::types::relation_type::RelationType;

use crate::Components;
use crate::PropertyInstances;
use reactive_graph_graph::InstanceId;
use reactive_graph_graph::InvalidRelationInstanceError;
use reactive_graph_graph::NamespacedTypeContainer;
use reactive_graph_graph::RelationInstanceTypeId;
use reactive_graph_graph::RelationInstanceTypeIdError;
use reactive_graph_graph::RelationTypeId;
use std::ops::Deref;
use std::str::FromStr;

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-schema.graphql",
    schema_module = "crate::schema_graphql::schema"
)]
pub struct RelationInstance {
    inbound: EntityInstance,
    #[cynic(rename = "type")]
    relation_type: RelationType,
    instance_id: String,
    outbound: EntityInstance,
    name: String,
    description: String,
    properties: Vec<PropertyInstance>,
    components: Vec<Component>,
}

impl RelationInstance {
    pub fn ty(&self) -> Result<RelationTypeId, RelationInstanceTypeIdError> {
        RelationTypeId::from_str(&self.relation_type._type).map_err(RelationInstanceTypeIdError::NamespacedTypeParseError)
    }

    pub fn instance_ty(&self) -> Result<RelationInstanceTypeId, RelationInstanceTypeIdError> {
        Ok(RelationInstanceTypeId::new(
            self.ty()?,
            InstanceId::parse_named(&self.instance_id).map_err(RelationInstanceTypeIdError::InstanceIdError)?,
        ))
    }
}

impl TryFrom<RelationInstance> for reactive_graph_graph::RelationInstance {
    type Error = InvalidRelationInstanceError;

    fn try_from(relation_instance: RelationInstance) -> Result<Self, Self::Error> {
        let ty = relation_instance
            .instance_ty()
            .map_err(InvalidRelationInstanceError::InvalidRelationInstanceTypeId)?;
        Ok(reactive_graph_graph::RelationInstance {
            outbound_id: relation_instance.outbound.id.into(),
            ty,
            inbound_id: relation_instance.inbound.id.into(),
            name: relation_instance.name.clone(),
            description: relation_instance.description.clone(),
            properties: PropertyInstances(relation_instance.properties).into(),
            components: reactive_graph_graph::Components::try_from(Components(relation_instance.components))
                .map_err(InvalidRelationInstanceError::InvalidComponent)?
                .type_ids(),
            // TODO: implement!
            extensions: Default::default(),
        })
    }
}

pub struct RelationInstances(pub Vec<RelationInstance>);

impl Deref for RelationInstances {
    type Target = Vec<RelationInstance>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<RelationInstances> for reactive_graph_graph::RelationInstances {
    type Error = InvalidRelationInstanceError;

    fn try_from(relations: RelationInstances) -> Result<Self, Self::Error> {
        let relation_instances = reactive_graph_graph::RelationInstances::new();
        for relation_instance in relations.0 {
            relation_instances.push(reactive_graph_graph::RelationInstance::try_from(relation_instance)?);
        }
        Ok(relation_instances)
    }
}
