use crate::schema_graphql::instances::entity_instance::EntityInstance;
use crate::schema_graphql::instances::property_instance::PropertyInstance;
use crate::schema_graphql::types::component::Component;
use crate::schema_graphql::types::relation_type::RelationType;

use crate::PropertyInstances;
use reactive_graph_graph::RelationInstanceTypeId;
use reactive_graph_graph::RelationTypeId;
use std::ops::Deref;

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-schema.graphql",
    schema_module = "crate::schema_graphql::schema"
)]
pub struct RelationInstance {
    inbound: EntityInstance,
    #[cynic(rename = "type")]
    ty: Option<RelationType>,
    instance_id: String,
    outbound: EntityInstance,
    name: String,
    description: String,
    properties: Vec<PropertyInstance>,
    components: Vec<Component>,
}

impl RelationInstance {
    pub fn ty(&self) -> RelationTypeId {
        self.ty
            .clone()
            .map(|relation_type| RelationTypeId::new_from_type(relation_type.namespace, relation_type.name))
            .unwrap_or(RelationTypeId::new_from_type(String::new(), String::new()))
    }

    pub fn instance_ty(&self) -> RelationInstanceTypeId {
        RelationInstanceTypeId::new_unique_for_instance_id(self.ty(), self.instance_id.clone())
    }
}

impl From<RelationInstance> for reactive_graph_graph::RelationInstance {
    fn from(relation_instance: RelationInstance) -> Self {
        let ty = relation_instance.instance_ty();
        let outbound_id = relation_instance.outbound.id.into();
        let inbound_id = relation_instance.inbound.id.into();
        let properties = PropertyInstances(relation_instance.properties).into();
        let components = relation_instance
            .components
            .iter()
            .map(|component| {
                let ty: reactive_graph_graph::ComponentTypeId = component.clone().ty().into();
                ty
            })
            .collect();
        reactive_graph_graph::RelationInstance {
            outbound_id,
            ty,
            inbound_id,
            name: relation_instance.name.clone(),
            description: relation_instance.description.clone(),
            properties,
            components,
            extensions: Default::default(),
        }
    }
}

pub struct RelationInstances(pub Vec<RelationInstance>);

impl Deref for RelationInstances {
    type Target = Vec<RelationInstance>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<RelationInstances> for Vec<reactive_graph_graph::RelationInstance> {
    fn from(relations: RelationInstances) -> Self {
        relations.0.into_iter().map(From::from).collect()
    }
}

impl From<RelationInstances> for reactive_graph_graph::RelationInstances {
    fn from(relations: RelationInstances) -> Self {
        relations.0.into_iter().map(From::from).collect()
    }
}
