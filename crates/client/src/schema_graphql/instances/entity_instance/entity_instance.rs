use crate::PropertyInstances;
use crate::schema_graphql::instances::property_instance::PropertyInstance;
use crate::schema_graphql::scalar::UUID;
use crate::schema_graphql::types::component::Component;
use crate::schema_graphql::types::component::Components;
use crate::schema_graphql::types::entity_type::EntityType;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::InvalidEntityInstanceError;
use reactive_graph_graph::NamespacedTypeContainer;
use std::ops::Deref;
use std::str::FromStr;

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-schema.graphql",
    schema_module = "crate::schema_graphql::schema"
)]
pub struct EntityInstance {
    /// The fully qualified name of the entity type.
    #[cynic(rename = "type")]
    pub entity_type: EntityType,
    pub id: UUID,
    pub name: String,
    pub description: String,
    pub properties: Vec<PropertyInstance>,
    pub components: Vec<Component>,
}

impl EntityInstance {
    pub fn ty(&self) -> Result<EntityTypeId, InvalidEntityInstanceError> {
        EntityTypeId::from_str(&self.entity_type._type).map_err(InvalidEntityInstanceError::InvalidEntityType)
    }
}

impl TryFrom<EntityInstance> for reactive_graph_graph::EntityInstance {
    type Error = InvalidEntityInstanceError;

    fn try_from(entity_instance: EntityInstance) -> Result<Self, Self::Error> {
        Ok(reactive_graph_graph::EntityInstance {
            ty: EntityTypeId::from_str(&entity_instance.entity_type._type)?,
            id: entity_instance.id.into(),
            name: entity_instance.name.clone(),
            description: entity_instance.description.clone(),
            properties: PropertyInstances(entity_instance.properties).into(),
            components: reactive_graph_graph::Components::try_from(Components(entity_instance.components))
                .map_err(InvalidEntityInstanceError::InvalidComponent)?
                .type_ids(),
            extensions: Default::default(),
        })
    }
}

pub struct EntityInstances(pub Vec<EntityInstance>);

impl Deref for EntityInstances {
    type Target = Vec<EntityInstance>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<EntityInstances> for reactive_graph_graph::EntityInstances {
    type Error = InvalidEntityInstanceError;

    fn try_from(entities: EntityInstances) -> Result<Self, Self::Error> {
        let entity_instances = reactive_graph_graph::EntityInstances::new();
        for entity_instance in entities.0 {
            entity_instances.push(reactive_graph_graph::EntityInstance::try_from(entity_instance)?);
        }
        Ok(entity_instances)
    }
}
