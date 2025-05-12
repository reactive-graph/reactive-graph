use crate::ExtensionDefinition;
use crate::PropertyInstanceDefinition;
use crate::schema_graphql::scalar::UUID;
use reactive_graph_graph::EntityInstance;
use reactive_graph_graph::EntityInstances;
use reactive_graph_graph::NamespacedTypeGetter;
use typed_builder::TypedBuilder;

#[derive(cynic::InputObject, Debug, TypedBuilder)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-schema.graphql",
    schema_module = "crate::schema_graphql::schema"
)]
pub struct EntityInstanceDefinition {
    pub namespace: String,
    pub type_name: String,
    pub id: UUID,
    pub description: String,
    pub properties: Vec<PropertyInstanceDefinition>,
    pub extensions: Vec<ExtensionDefinition>,
}

impl From<EntityInstance> for EntityInstanceDefinition {
    fn from(entity_instance: EntityInstance) -> Self {
        let ty = entity_instance.ty.clone();
        let properties = entity_instance
            .properties
            .into_iter()
            .map(|(name, value)| PropertyInstanceDefinition { name, value })
            .collect();
        let extensions = entity_instance.extensions.into_iter().map(|(_, extension)| extension.into()).collect();
        EntityInstanceDefinition {
            namespace: ty.namespace(),
            type_name: ty.type_name(),
            id: UUID(entity_instance.id),
            description: entity_instance.description.clone(),
            properties,
            extensions,
        }
    }
}

pub struct EntityInstanceDefinitions(pub Vec<EntityInstanceDefinition>);

impl EntityInstanceDefinitions {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl From<EntityInstanceDefinitions> for Vec<EntityInstanceDefinition> {
    fn from(entity_instances: EntityInstanceDefinitions) -> Self {
        entity_instances.0.into_iter().collect()
    }
}

impl From<EntityInstances> for EntityInstanceDefinitions {
    fn from(entity_instances: EntityInstances) -> Self {
        EntityInstanceDefinitions(
            entity_instances
                .iter()
                .map(|p| {
                    let entity_instance = p.value().clone();
                    entity_instance.into()
                    // let ty = entity_instance.ty.clone();
                    // let properties = entity_instance
                    //     .properties
                    //     .into_iter()
                    //     .map(|(name, value)| PropertyInstanceDefinition { name, value })
                    //     .collect();
                    // let extensions = entity_instance.extensions.into_iter().map(|(_, extension)| extension.into()).collect();
                    // EntityInstanceDefinition {
                    //     namespace: ty.namespace(),
                    //     type_name: ty.type_name(),
                    //     id: UUID(*p.key()),
                    //     description: entity_instance.description.clone(),
                    //     properties,
                    //     extensions,
                    // }
                })
                .collect(),
        )
    }
}

impl Default for EntityInstanceDefinitions {
    fn default() -> Self {
        Self::new()
    }
}
