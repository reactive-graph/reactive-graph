use std::ops::Deref;
use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;

use reactive_graph_behaviour_service_api::RelationBehaviourRegistry;
use reactive_graph_graph::ComponentOrEntityTypeId;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::RelationType;
use reactive_graph_graph::RelationTypes;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::RelationTypeManager;

use crate::mutation::ExtensionTypeIdDefinition;
use crate::query::GraphQLComponent;
use crate::query::GraphQLEntityType;
use crate::query::GraphQLEntityTypes;
use crate::query::GraphQLExtension;
use crate::query::GraphQLPropertyType;
use crate::query::GraphQLRelationBehaviour;

pub struct GraphQLRelationType {
    relation_type: RelationType,
}

/// A relation type defines the type of an relation instance.
///
/// The relation type defines the entity types of the outbound and inbound entity instances.
/// Also the relation type defines the properties of the relation instance.
#[Object(name = "RelationType")]
impl GraphQLRelationType {
    /// The namespace the relation type belongs to.
    async fn namespace(&self) -> String {
        self.relation_type.namespace()
    }

    /// The outbound entity type(s).
    async fn outbound_types(&self, context: &Context<'_>) -> Result<Vec<GraphQLEntityType>> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        if self.relation_type.outbound_type.type_name() == "*" {
            let entity_types: GraphQLEntityTypes = entity_type_manager.get_all().into();
            return Ok(entity_types.into());
        }
        match &self.relation_type.outbound_type {
            ComponentOrEntityTypeId::Component(component_ty) => {
                let entity_types: GraphQLEntityTypes = entity_type_manager.get_by_having_component(component_ty).into();
                // let entity_types = entity_type_manager
                //     .get_by_having_component(component_ty)
                //     .iter()
                //     .map(|entity_type| entity_type.value().into())
                //     .collect();
                return Ok(entity_types.into());
            }
            ComponentOrEntityTypeId::EntityType(entity_ty) => {
                if let Some(entity_type) = entity_type_manager.get(entity_ty) {
                    let entity_type = entity_type.into();
                    return Ok(vec![entity_type]);
                }
            }
        }
        Ok(Vec::new())
    }

    /// The outbound components.
    async fn outbound_components(&self, context: &Context<'_>) -> Result<Vec<GraphQLComponent>> {
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;
        if let ComponentOrEntityTypeId::Component(component_ty) = &self.relation_type.outbound_type {
            let components = component_manager.get(component_ty).iter().cloned().map(|component| component.into()).collect();
            return Ok(components);
        }
        Ok(Vec::new())
    }

    /// The name of the relation type.
    ///
    /// The name is the unique identifier for relation types.
    ///
    /// Returns "default_connector" for "default_connector__property_name__property_name"
    /// (without type suffix).
    async fn name(&self) -> String {
        self.relation_type.type_name()
    }

    /// The inbound entity type(s).
    async fn inbound_types(&self, context: &Context<'_>) -> Result<Vec<GraphQLEntityType>> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        if self.relation_type.inbound_type.type_name() == "*" {
            let entity_types: GraphQLEntityTypes = entity_type_manager.get_all().into();
            return Ok(entity_types.into());
        }
        match &self.relation_type.inbound_type {
            ComponentOrEntityTypeId::Component(component_ty) => {
                let entity_types: GraphQLEntityTypes = entity_type_manager.get_by_having_component(component_ty).into();
                // let entity_types = entity_type_manager
                //     .get_by_having_component(component_ty)
                //     .iter()
                //     .cloned()
                //     .map(|entity_type| entity_type.into())
                //     .collect();
                return Ok(entity_types.into());
            }
            ComponentOrEntityTypeId::EntityType(entity_ty) => {
                if let Some(entity_type) = entity_type_manager.get(entity_ty) {
                    let entity_type = entity_type.into();
                    return Ok(vec![entity_type]);
                }
            }
        }
        Ok(Vec::new())
    }

    /// The inbound components.
    async fn inbound_components(&self, context: &Context<'_>) -> Result<Vec<GraphQLComponent>> {
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;
        if let ComponentOrEntityTypeId::Component(component_ty) = &self.relation_type.inbound_type {
            let components = component_manager.get(component_ty).iter().cloned().map(|component| component.into()).collect();
            return Ok(components);
        }
        Ok(Vec::new())
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
            .filter_map(|component_ty| component_manager.get(&component_ty))
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
        #[graphql(name = "type")] extension_ty: Option<ExtensionTypeIdDefinition>,
        #[graphql(desc = "If true, the extensions are sorted by type")] sort: Option<bool>,
    ) -> Vec<GraphQLExtension> {
        match extension_ty {
            Some(extension_ty) => {
                let extension_ty = extension_ty.into();
                return self
                    .relation_type
                    .extensions
                    .iter()
                    .filter(|extension| extension.ty == extension_ty)
                    .map(|extension| extension.value().into())
                    .collect();
            }
            None => {
                let mut extensions: Vec<GraphQLExtension> = self.relation_type.extensions.iter().map(|extension| extension.value().into()).collect();
                if sort.unwrap_or_default() {
                    extensions.sort();
                }
                extensions
            }
        }
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
}

impl From<RelationType> for GraphQLRelationType {
    fn from(relation_type: RelationType) -> Self {
        GraphQLRelationType { relation_type }
    }
}

pub struct GraphQLRelationTypes(Vec<GraphQLRelationType>);

impl Deref for GraphQLRelationTypes {
    type Target = Vec<GraphQLRelationType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<GraphQLRelationTypes> for Vec<GraphQLRelationType> {
    fn from(value: GraphQLRelationTypes) -> Self {
        value.0
    }
}

impl From<RelationTypes> for GraphQLRelationTypes {
    fn from(relation_types: RelationTypes) -> Self {
        let relation_types = relation_types.into_iter().map(|(_, relation_type)| relation_type.into()).collect();
        GraphQLRelationTypes(relation_types)
    }
}
