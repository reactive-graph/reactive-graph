use std::ops::Deref;
use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;
use reactive_graph_behaviour_service_api::RelationBehaviourRegistry;
use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::InboundOutboundType;
use reactive_graph_graph::JsonSchemaIdGetter;
use reactive_graph_graph::MatchingInboundOutboundType;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::RelationType;
use reactive_graph_graph::RelationTypes;
use reactive_graph_graph::TypeDefinitionJsonSchemaGetter;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::RelationTypeManager;
use serde_json::Value;

use crate::query::GraphQLComponent;
use crate::query::GraphQLComponents;
use crate::query::GraphQLEntityType;
use crate::query::GraphQLEntityTypes;
use crate::query::GraphQLExtension;
use crate::query::GraphQLExtensions;
use crate::query::GraphQLNamespacedType;
use crate::query::GraphQLPropertyType;
use crate::query::GraphQLRelationBehaviour;

pub struct GraphQLRelationType {
    relation_type: RelationType,
}

/// A relation type defines the type of relation instances.
///
/// The relation type defines the entity types of the outbound and inbound entity instances.
/// Also, the relation type defines the properties of the relation instance.
#[Object(name = "RelationType")]
impl GraphQLRelationType {
    /// The outbound entity type(s).
    async fn outbound_types(&self, context: &Context<'_>) -> Result<Vec<GraphQLEntityType>> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let entity_types = match &self.relation_type.outbound_type {
            InboundOutboundType::Component(MatchingInboundOutboundType::NamespacedType(ty)) => entity_type_manager.get_by_having_component(ty),
            InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(ty)) => entity_type_manager.get(ty).into_iter().collect(),
            InboundOutboundType::Component(MatchingInboundOutboundType::Any) | InboundOutboundType::EntityType(MatchingInboundOutboundType::Any) => {
                entity_type_manager.get_all()
            }
        };
        Ok(GraphQLEntityTypes::new(entity_types).into())
    }

    /// The outbound components.
    async fn outbound_components(&self, context: &Context<'_>) -> Result<Vec<GraphQLComponent>> {
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let components = match &self.relation_type.outbound_type {
            InboundOutboundType::Component(MatchingInboundOutboundType::NamespacedType(ty)) => component_manager.get(ty).into_iter().collect(),
            InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(ty)) => entity_type_manager
                .get(ty)
                .map(|entity_type| component_manager.get_by_types(entity_type.components))
                .unwrap_or_default(),
            InboundOutboundType::Component(MatchingInboundOutboundType::Any) | InboundOutboundType::EntityType(MatchingInboundOutboundType::Any) => {
                component_manager.get_all()
            }
        };
        Ok(GraphQLComponents::new(components).into())
    }

    /// The namespace and type name.
    #[graphql(name = "type")]
    async fn ty(&self) -> GraphQLNamespacedType {
        self.relation_type.namespaced_type().into()
    }

    /// The inbound entity type(s).
    async fn inbound_types(&self, context: &Context<'_>) -> Result<Vec<GraphQLEntityType>> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let entity_types = match &self.relation_type.inbound_type {
            InboundOutboundType::Component(MatchingInboundOutboundType::NamespacedType(ty)) => entity_type_manager.get_by_having_component(ty),
            InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(ty)) => entity_type_manager.get(ty).into_iter().collect(),
            InboundOutboundType::Component(MatchingInboundOutboundType::Any) | InboundOutboundType::EntityType(MatchingInboundOutboundType::Any) => {
                entity_type_manager.get_all()
            }
        };
        Ok(GraphQLEntityTypes::new(entity_types).into())
    }

    /// The inbound components.
    async fn inbound_components(&self, context: &Context<'_>) -> Result<Vec<GraphQLComponent>> {
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let components = match &self.relation_type.inbound_type {
            InboundOutboundType::Component(MatchingInboundOutboundType::NamespacedType(ty)) => component_manager.get(ty).into_iter().collect(),
            InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(ty)) => entity_type_manager
                .get(ty)
                .map(|entity_type| component_manager.get_by_types(entity_type.components))
                .unwrap_or_default(),
            InboundOutboundType::Component(MatchingInboundOutboundType::Any) | InboundOutboundType::EntityType(MatchingInboundOutboundType::Any) => {
                component_manager.get_all()
            }
        };
        Ok(GraphQLComponents::new(components).into())
    }

    /// Textual description of the relation type.
    async fn description(&self) -> String {
        self.relation_type.description.clone()
    }

    /// The relation type composes it's properties by these components.
    async fn components(&self, context: &Context<'_>) -> Result<Vec<GraphQLComponent>> {
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;
        let components = self
            .relation_type
            .components
            .iter()
            .filter_map(|ty| component_manager.get(&ty))
            .map(|component| component.into())
            .collect();
        Ok(components)
    }

    /// The count of components.
    async fn count_components(&self) -> usize {
        self.relation_type.components.len()
    }

    /// The properties / property types which are defined by the relation type or
    /// by one of the components.
    async fn properties(
        &self,
        #[graphql(desc = "The name of the property")] name: Option<String>,
        #[graphql(desc = "If true, the properties are sorted by name")] sort: Option<bool>,
    ) -> Vec<GraphQLPropertyType> {
        match name {
            Some(name) => self
                .relation_type
                .properties
                .iter()
                .filter(|property_type| property_type.name == name.clone())
                // .cloned()
                .map(|property_type| property_type.value().into())
                .collect(),
            None => {
                let mut properties: Vec<GraphQLPropertyType> = self.relation_type.properties.iter().map(|property_type| property_type.value().into()).collect();
                if sort.unwrap_or_default() {
                    properties.sort();
                }
                properties
            }
        }
    }

    /// The count of properties.
    async fn count_properties(&self) -> usize {
        self.relation_type.properties.len()
    }

    /// The extensions which are defined by the relation type.
    async fn extensions(
        &self,
        #[graphql(name = "type")] namespace: Option<String>,
        #[graphql(desc = "If true, the extensions are sorted by type")] sort: Option<bool>,
    ) -> Result<Vec<GraphQLExtension>> {
        let ty = match namespace {
            Some(namespace) => Some(ExtensionTypeId::parse_namespace(&namespace)?),
            None => None,
        };
        let extensions: GraphQLExtensions = self
            .relation_type
            .extensions
            .iter()
            .filter(|extension| match &ty {
                Some(ty) => &extension.ty == ty,
                None => true,
            })
            .map(|extension| extension.value().clone())
            .collect();
        Ok(if sort.unwrap_or_default() { extensions.sorted() } else { extensions.into() })
    }

    /// The count of extensions.
    async fn count_extensions(&self) -> usize {
        self.relation_type.extensions.len()
    }

    /// Returns true, if the relation type is valid.
    ///
    /// This means all components exists and the outbound and inbound entity types are valid.
    async fn is_valid(&self, context: &Context<'_>) -> bool {
        match context.data::<Arc<dyn RelationTypeManager + Send + Sync>>() {
            Ok(relation_type_manager) => relation_type_manager.validate(&self.relation_type.ty),
            Err(_) => false,
        }
    }

    async fn behaviours(&self, context: &Context<'_>) -> Result<Vec<GraphQLRelationBehaviour>> {
        let relation_behaviour_registry = context.data::<Arc<dyn RelationBehaviourRegistry + Send + Sync>>()?;
        let relation_behaviour_types = relation_behaviour_registry
            .get_behaviour_types(&self.relation_type.ty)
            .iter()
            .map(|relation_behaviour_ty| GraphQLRelationBehaviour::from(relation_behaviour_ty.clone()))
            .collect();
        Ok(relation_behaviour_types)
    }

    /// Returns the JSON schema of the relation type.
    async fn json_schema(&self) -> Value {
        self.relation_type.json_schema().to_value()
    }

    /// Returns the JSON schema identifier ($id) of the relation type.
    async fn json_schema_id(&self) -> String {
        self.relation_type.json_schema_id().to_string()
    }
}

impl From<RelationType> for GraphQLRelationType {
    fn from(relation_type: RelationType) -> Self {
        GraphQLRelationType { relation_type }
    }
}

pub struct GraphQLRelationTypes(RelationTypes);

impl GraphQLRelationTypes {
    pub fn new(relation_types: RelationTypes) -> Self {
        Self(relation_types)
    }
}

impl Deref for GraphQLRelationTypes {
    type Target = RelationTypes;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<GraphQLRelationTypes> for Vec<GraphQLRelationType> {
    fn from(relation_types: GraphQLRelationTypes) -> Self {
        relation_types.0.into_iter().map(|(_, relation_type)| relation_type.into()).collect()
    }
}

impl From<RelationTypes> for GraphQLRelationTypes {
    fn from(relation_types: RelationTypes) -> Self {
        GraphQLRelationTypes::new(relation_types)
    }
}
