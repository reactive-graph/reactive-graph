use crate::PropertyInstances;
use crate::schema_graphql::instances::property_instance::PropertyInstance;
use crate::schema_graphql::scalar::UUID;
use crate::schema_graphql::types::component::Component;
use crate::schema_graphql::types::entity_type::EntityType;
use reactive_graph_graph::EntityTypeId;
use std::ops::Deref;

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-schema.graphql",
    schema_module = "crate::schema_graphql::schema"
)]
pub struct EntityInstance {
    #[cynic(rename = "type")]
    pub ty: Option<EntityType>,
    pub id: UUID,
    pub name: String,
    pub description: String,
    pub properties: Vec<PropertyInstance>,
    pub components: Vec<Component>,
}

impl EntityInstance {
    pub fn ty(&self) -> EntityTypeId {
        self.ty
            .clone()
            .map(|entity_type| EntityTypeId::new_from_type(entity_type.namespace, entity_type.name))
            .unwrap_or(EntityTypeId::new_from_type(String::new(), String::new()))
    }
}

impl From<EntityInstance> for reactive_graph_graph::EntityInstance {
    fn from(entity_instance: EntityInstance) -> Self {
        let ty = entity_instance.ty();
        let id = entity_instance.id.into();
        let properties = PropertyInstances(entity_instance.properties).into();
        let components = entity_instance
            .components
            .iter()
            .map(|component| {
                let ty: reactive_graph_graph::ComponentTypeId = component.clone().ty().into();
                ty
            })
            .collect();
        reactive_graph_graph::EntityInstance {
            ty,
            id,
            name: entity_instance.name.clone(),
            description: entity_instance.description.clone(),
            properties,
            components,
            extensions: Default::default(),
        }
    }
}

pub struct EntityInstances(pub Vec<EntityInstance>);

impl Deref for EntityInstances {
    type Target = Vec<EntityInstance>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<EntityInstances> for Vec<reactive_graph_graph::EntityInstance> {
    fn from(entities: EntityInstances) -> Self {
        entities.0.into_iter().map(From::from).collect()
    }
}

impl From<EntityInstances> for reactive_graph_graph::EntityInstances {
    fn from(entities: EntityInstances) -> Self {
        entities.0.into_iter().map(From::from).collect()
    }
}
