use crate::schema_graphql::types::component::Component;
use crate::schema_graphql::types::component::Components;
use crate::schema_graphql::types::extension::Extension;
use crate::schema_graphql::types::extension::Extensions;
use crate::schema_graphql::types::property_type::PropertyType;
use crate::schema_graphql::types::property_type::PropertyTypes;
use reactive_graph_graph::ComponentOrEntityTypeId;
use reactive_graph_graph::NamespacedTypeGetter;
use serde_json::Value;
use std::ops::Deref;

#[derive(cynic::InputObject, Clone, Debug)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-schema.graphql",
    schema_module = "crate::schema_graphql::schema"
)]
pub struct EntityTypeId {
    pub name: String,
    pub namespace: String,
}

impl From<reactive_graph_graph::EntityTypeId> for EntityTypeId {
    fn from(ty: reactive_graph_graph::EntityTypeId) -> Self {
        EntityTypeId {
            name: ty.type_name(),
            namespace: ty.namespace(),
        }
    }
}

impl From<&EntityTypeId> for reactive_graph_graph::EntityTypeId {
    fn from(ty: &EntityTypeId) -> Self {
        reactive_graph_graph::EntityTypeId::new_from_type(ty.namespace.clone(), ty.name.clone())
    }
}

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-schema.graphql",
    schema_module = "crate::schema_graphql::schema"
)]
pub struct EntityType {
    /// The namespace of the extension.
    pub namespace: String,

    /// The name of the extension.
    pub name: String,

    /// Textual description of the extension.
    pub description: String,

    /// The applied components.
    pub components: Vec<Component>,

    /// The property types.
    pub properties: Vec<PropertyType>,

    /// The extensions.
    pub extensions: Vec<Extension>,

    /// The JSON schema.
    pub json_schema: Value,
}

impl EntityType {
    pub fn ty(&self) -> EntityTypeId {
        EntityTypeId {
            namespace: self.namespace.clone(),
            name: self.name.clone(),
        }
    }
}

impl From<EntityType> for reactive_graph_graph::EntityType {
    fn from(entity_type: EntityType) -> Self {
        let ty = (&entity_type.ty()).into();
        let components: reactive_graph_graph::ComponentTypeIds = Components(entity_type.components).into();
        reactive_graph_graph::EntityType {
            ty,
            description: entity_type.description,
            components,
            properties: PropertyTypes(entity_type.properties).into(),
            extensions: Extensions(entity_type.extensions).into(),
        }
    }
}

pub struct EntityTypes(pub Vec<EntityType>);

impl Deref for EntityTypes {
    type Target = Vec<EntityType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<EntityTypes> for Vec<reactive_graph_graph::EntityType> {
    fn from(entity_types: EntityTypes) -> Self {
        entity_types.0.into_iter().map(From::from).collect()
    }
}

impl From<EntityType> for ComponentOrEntityTypeId {
    fn from(entity_type: EntityType) -> Self {
        let entity_type: reactive_graph_graph::EntityType = entity_type.into();
        ComponentOrEntityTypeId::EntityType(entity_type.ty)
    }
}
