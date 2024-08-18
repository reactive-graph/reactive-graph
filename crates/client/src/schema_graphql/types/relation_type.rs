use crate::schema_graphql::types::component::Component;
use crate::schema_graphql::types::component::Components;
use crate::schema_graphql::types::entity_type::EntityType;
use crate::schema_graphql::types::extension::Extension;
use crate::schema_graphql::types::extension::Extensions;
use crate::schema_graphql::types::property_type::PropertyType;
use crate::schema_graphql::types::property_type::PropertyTypes;
use reactive_graph_graph::ComponentOrEntityTypeId;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::NamespacedTypeGetter;
use std::ops::Deref;

#[derive(cynic::InputObject, Clone, Debug)]
#[cynic(schema_path = "schema_graphql.graphql", schema_module = "crate::schema_graphql::schema")]
pub struct RelationTypeId {
    pub name: String,
    pub namespace: String,
}

impl From<reactive_graph_graph::RelationTypeId> for RelationTypeId {
    fn from(ty: reactive_graph_graph::RelationTypeId) -> Self {
        RelationTypeId {
            name: ty.type_name(),
            namespace: ty.namespace(),
        }
    }
}

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(schema_path = "schema_graphql.graphql", schema_module = "crate::schema_graphql::schema")]
pub struct RelationType {
    /// The outbound type(s).
    pub outbound_types: Vec<EntityType>,

    /// The namespace of the extension.
    pub namespace: String,

    /// The name of the extension.
    pub name: String,

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
}

impl RelationType {
    fn get_outbound_type(&self) -> ComponentOrEntityTypeId {
        self.outbound_types
            .first()
            .cloned()
            .map(|entity_type| entity_type.into())
            .unwrap_or(ComponentOrEntityTypeId::EntityType(EntityTypeId::new_from_type("*", "*")))
    }

    fn get_inbound_type(&self) -> ComponentOrEntityTypeId {
        self.inbound_types
            .first()
            .cloned()
            .map(|entity_type| entity_type.into())
            .unwrap_or(ComponentOrEntityTypeId::EntityType(EntityTypeId::new_from_type("*", "*")))
    }
}

impl From<RelationType> for reactive_graph_graph::RelationType {
    fn from(relation_type: RelationType) -> Self {
        let ty = reactive_graph_graph::RelationTypeId::new_from_type(&relation_type.namespace, &relation_type.name);
        let components: reactive_graph_graph::ComponentTypeIds = Components(relation_type.components.clone()).into();
        let outbound_type = relation_type.get_outbound_type();
        let inbound_type = relation_type.get_inbound_type();
        reactive_graph_graph::RelationType {
            outbound_type,
            ty,
            inbound_type,
            description: relation_type.description,
            components,
            properties: PropertyTypes(relation_type.properties).into(),
            extensions: Extensions(relation_type.extensions).into(),
        }
    }
}

pub struct RelationTypes(pub Vec<RelationType>);

impl Deref for RelationTypes {
    type Target = Vec<RelationType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<RelationTypes> for Vec<reactive_graph_graph::RelationType> {
    fn from(relation_types: RelationTypes) -> Self {
        relation_types.0.into_iter().map(From::from).collect()
    }
}
