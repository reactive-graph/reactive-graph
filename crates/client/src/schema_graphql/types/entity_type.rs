use crate::schema_graphql::types::component::Component;
use crate::schema_graphql::types::component::Components;
use crate::schema_graphql::types::extension::Extension;
use crate::schema_graphql::types::extension::Extensions;
use crate::schema_graphql::types::property_type::PropertyType;
use crate::schema_graphql::types::property_type::PropertyTypes;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::InboundOutboundType;
use reactive_graph_graph::InvalidEntityTypeError;
use reactive_graph_graph::MatchingInboundOutboundType;
use reactive_graph_graph::NamespacedTypeContainer;
use reactive_graph_graph::NamespacedTypeParseError;
use serde_json::Value;
use std::ops::Deref;
use std::str::FromStr;

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-schema.graphql",
    schema_module = "crate::schema_graphql::schema"
)]
pub struct EntityType {
    /// The fully qualified namespace of the entity type.
    #[cynic(rename = "type")]
    pub _type: String,

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
    pub fn ty(&self) -> Result<EntityTypeId, NamespacedTypeParseError> {
        EntityTypeId::from_str(&self._type)
    }
}

impl TryFrom<EntityType> for reactive_graph_graph::EntityType {
    type Error = InvalidEntityTypeError;

    fn try_from(entity_type: EntityType) -> Result<Self, Self::Error> {
        Ok(reactive_graph_graph::EntityType {
            ty: EntityTypeId::from_str(&entity_type._type).map_err(InvalidEntityTypeError::InvalidEntityType)?,
            description: entity_type.description,
            components: reactive_graph_graph::Components::try_from(Components(entity_type.components))
                .map_err(InvalidEntityTypeError::InvalidComponent)?
                .type_ids(),
            properties: reactive_graph_graph::PropertyTypes::try_from(PropertyTypes(entity_type.properties))
                .map_err(InvalidEntityTypeError::InvalidPropertyType)?,
            extensions: reactive_graph_graph::Extensions::try_from(Extensions(entity_type.extensions)).map_err(InvalidEntityTypeError::InvalidExtension)?,
        })
    }
}

pub struct EntityTypes(pub Vec<EntityType>);

impl Deref for EntityTypes {
    type Target = Vec<EntityType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<EntityTypes> for reactive_graph_graph::EntityTypes {
    type Error = InvalidEntityTypeError;

    fn try_from(entity_types: EntityTypes) -> Result<Self, Self::Error> {
        let entity_types_2 = reactive_graph_graph::EntityTypes::new();
        for entity_type in entity_types.0 {
            entity_types_2.push(reactive_graph_graph::EntityType::try_from(entity_type)?);
        }
        Ok(entity_types_2)
    }
}

impl TryFrom<EntityType> for InboundOutboundType {
    type Error = InvalidEntityTypeError;

    fn try_from(entity_type: EntityType) -> Result<Self, Self::Error> {
        Ok(InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(EntityTypeId::from_str(
            &entity_type._type,
        )?)))
    }
}
