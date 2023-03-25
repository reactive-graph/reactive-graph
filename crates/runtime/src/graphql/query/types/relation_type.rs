use std::sync::Arc;

use async_graphql::*;

use crate::api::ComponentManager;
use crate::api::EntityTypeManager;
use crate::api::RelationBehaviourRegistry;
use crate::api::RelationTypeManager;
use crate::graphql::mutation::ExtensionTypeIdDefinition;
use crate::graphql::query::GraphQLComponent;
use crate::graphql::query::GraphQLEntityType;
use crate::graphql::query::GraphQLExtension;
use crate::graphql::query::GraphQLPropertyType;
use crate::graphql::query::GraphQLRelationBehaviour;
use crate::model::ComponentOrEntityTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::RelationType;

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
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        if self.relation_type.outbound_type.type_name() == "*" {
            let entity_types = entity_type_manager.get_all().iter().cloned().map(|entity_type| entity_type.into()).collect();
            return Ok(entity_types);
        }
        match &self.relation_type.outbound_type {
            ComponentOrEntityTypeId::Component(component_ty) => {
                let entity_types = entity_type_manager
                    .get_by_having_component(component_ty)
                    .iter()
                    .cloned()
                    .map(|entity_type| entity_type.into())
                    .collect();
                return Ok(entity_types);
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
        let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
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
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        if self.relation_type.inbound_type.type_name() == "*" {
            let entity_types = entity_type_manager.get_all().iter().cloned().map(|entity_type| entity_type.into()).collect();
            return Ok(entity_types);
        }
        match &self.relation_type.inbound_type {
            ComponentOrEntityTypeId::Component(component_ty) => {
                let entity_types = entity_type_manager
                    .get_by_having_component(component_ty)
                    .iter()
                    .cloned()
                    .map(|entity_type| entity_type.into())
                    .collect();
                return Ok(entity_types);
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
        let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
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
        let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
        let components = self
            .relation_type
            .components
            .iter()
            .filter_map(|component_ty| component_manager.get(component_ty))
            .map(|component| component.into())
            .collect();
        Ok(components)
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
                .map(|property_type| property_type.into())
                .collect(),
            None => {
                let mut properties: Vec<GraphQLPropertyType> = self.relation_type.properties.iter().map(|property_type| property_type.into()).collect();
                if sort.unwrap_or_default() {
                    properties.sort();
                }
                properties
            }
        }
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
                    .map(|extension| extension.into())
                    .collect();
            }
            None => {
                let mut extensions: Vec<GraphQLExtension> = self.relation_type.extensions.iter().map(|extension| extension.into()).collect();
                if sort.unwrap_or_default() {
                    extensions.sort();
                }
                extensions
            }
        }
    }

    /// Returns true, if the relation type is valid.
    ///
    /// This means all components exists and the outbound and inbound entity types are valid.
    async fn is_valid(&self, context: &Context<'_>) -> bool {
        match context.data::<Arc<dyn RelationTypeManager>>() {
            Ok(relation_type_manager) => relation_type_manager.validate(&self.relation_type.ty),
            Err(_) => false,
        }
    }

    async fn behaviours(&self, context: &Context<'_>) -> Result<Vec<GraphQLRelationBehaviour>> {
        let relation_behaviour_registry = context.data::<Arc<dyn RelationBehaviourRegistry>>()?;
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
