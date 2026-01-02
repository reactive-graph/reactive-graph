use crate::schema_graphql::types::component::Component;
use crate::schema_graphql::types::component::Components;
use crate::schema_graphql::types::entity_type::EntityType;
use crate::schema_graphql::types::extension::Extension;
use crate::schema_graphql::types::extension::Extensions;
use crate::schema_graphql::types::property_type::PropertyType;
use crate::schema_graphql::types::property_type::PropertyTypes;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::InboundOutboundType;
use reactive_graph_graph::InvalidRelationTypeError;
use reactive_graph_graph::MatchingInboundOutboundType;
use reactive_graph_graph::NamespacedTypeContainer;
use reactive_graph_graph::NamespacedTypeParseError;
use reactive_graph_graph::RelationTypeId;
use serde_json::Value;
use std::ops::Deref;
use std::str::FromStr;

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-schema.graphql",
    schema_module = "crate::schema_graphql::schema"
)]
pub struct RelationType {
    /// The outbound type(s).
    pub outbound_types: Vec<EntityType>,

    /// The fully qualified namespace of the relation type.
    #[cynic(rename = "type")]
    pub _type: String,

    /// The inbound type(s).
    pub inbound_types: Vec<EntityType>,

    /// Textual description of the extension.
    pub description: String,

    /// The property types.
    pub components: Vec<Component>,

    /// The property types.
    pub properties: Vec<PropertyType>,

    /// The extensions.
    pub extensions: Vec<Extension>,

    /// The JSON schema.
    pub json_schema: Value,
}

impl RelationType {
    pub fn ty(&self) -> Result<RelationTypeId, NamespacedTypeParseError> {
        RelationTypeId::from_str(&self._type)
    }

    // TODO: Return all possible types as Vec<InboundOutboundType>
    fn get_outbound_type(&self) -> Result<InboundOutboundType, NamespacedTypeParseError> {
        // TODO: fix this: first()
        let Some(outbound_type) = self.outbound_types.first() else {
            return Ok(InboundOutboundType::EntityType(MatchingInboundOutboundType::Any));
        };
        let ty = EntityTypeId::from_str(&outbound_type._type)?;
        Ok(InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(ty)))
    }

    fn get_inbound_type(&self) -> Result<InboundOutboundType, NamespacedTypeParseError> {
        // TODO: fix this: first()
        let Some(inbound_type) = self.inbound_types.first() else {
            return Ok(InboundOutboundType::EntityType(MatchingInboundOutboundType::Any));
        };
        let ty = EntityTypeId::from_str(&inbound_type._type)?;
        Ok(InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(ty)))
    }
}

impl TryFrom<RelationType> for reactive_graph_graph::RelationType {
    type Error = InvalidRelationTypeError;

    fn try_from(relation_type: RelationType) -> Result<Self, Self::Error> {
        let components = Components(relation_type.components.clone());
        Ok(reactive_graph_graph::RelationType {
            outbound_type: relation_type.get_outbound_type()?,
            ty: RelationTypeId::from_str(&relation_type._type).map_err(InvalidRelationTypeError::InvalidRelationType)?,
            inbound_type: relation_type.get_inbound_type()?,
            description: relation_type.description,
            components: reactive_graph_graph::Components::try_from(components)
                .map_err(InvalidRelationTypeError::InvalidComponent)?
                .type_ids(),
            properties: reactive_graph_graph::PropertyTypes::try_from(PropertyTypes(relation_type.properties))
                .map_err(InvalidRelationTypeError::InvalidPropertyType)?,
            extensions: reactive_graph_graph::Extensions::try_from(Extensions(relation_type.extensions)).map_err(InvalidRelationTypeError::InvalidExtension)?,
        })
    }
}

pub struct RelationTypes(pub Vec<RelationType>);

impl Deref for RelationTypes {
    type Target = Vec<RelationType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<RelationTypes> for reactive_graph_graph::RelationTypes {
    type Error = InvalidRelationTypeError;

    fn try_from(relation_types: RelationTypes) -> Result<Self, Self::Error> {
        let relation_types_2 = reactive_graph_graph::RelationTypes::new();
        for relation_type in relation_types.0 {
            relation_types_2.push(reactive_graph_graph::RelationType::try_from(relation_type)?);
        }
        Ok(relation_types_2)
    }
}
