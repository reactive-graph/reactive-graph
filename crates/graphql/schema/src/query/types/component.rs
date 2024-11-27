use std::sync::Arc;

use crate::mutation::ExtensionTypeIdDefinition;
use crate::query::GraphQLComponentBehaviour;
use crate::query::GraphQLEntityType;
use crate::query::GraphQLEntityTypes;
use crate::query::GraphQLExtension;
use crate::query::GraphQLPropertyType;
use crate::query::GraphQLRelationType;
use crate::query::GraphQLRelationTypes;
use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;
use reactive_graph_behaviour_service_api::EntityComponentBehaviourRegistry;
use reactive_graph_behaviour_service_api::RelationComponentBehaviourRegistry;
use reactive_graph_graph::Component;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::RelationTypeManager;

pub struct GraphQLComponent {
    component: Component,
}

/// Components are composable parts which can be used by types (entity type, relation type).
#[Object(name = "Component")]
impl GraphQLComponent {
    /// The namespace the component belongs to.
    async fn namespace(&self) -> String {
        self.component.namespace()
    }

    /// The name of the component.
    async fn name(&self) -> String {
        self.component.type_name()
    }

    /// Textual description of the component.
    async fn description(&self) -> String {
        self.component.description.clone()
    }

    /// The properties which are applied on entity or relation instances.
    async fn properties(
        &self,
        #[graphql(desc = "The name of the property")] name: Option<String>,
        #[graphql(desc = "If true, the properties are sorted by name")] sort: Option<bool>,
    ) -> Vec<GraphQLPropertyType> {
        match name {
            Some(name) => {
                self.component
                    .properties
                    .iter()
                    .filter(|property_type| property_type.name == name.clone())
                    // .cloned()
                    .map(|property_type| property_type.value().into())
                    .collect()
            }
            None => {
                let mut properties: Vec<GraphQLPropertyType> = self.component.properties.iter().map(|property_type| property_type.value().into()).collect();
                if sort.unwrap_or_default() {
                    properties.sort();
                }
                properties
            }
        }
    }

    /// The count of properties.
    async fn count_properties(&self) -> usize {
        self.component.properties.len()
    }

    /// The extensions which are defined by the component.
    async fn extensions(
        &self,
        #[graphql(name = "type")] extension_ty: Option<ExtensionTypeIdDefinition>,
        #[graphql(desc = "If true, the extensions are sorted by type")] sort: Option<bool>,
    ) -> Vec<GraphQLExtension> {
        match extension_ty {
            Some(extension_ty) => {
                let extension_ty = extension_ty.into();
                return self
                    .component
                    .extensions
                    .iter()
                    .filter(|extension| extension.ty == extension_ty)
                    .map(|extension| extension.value().into())
                    .collect();
            }
            None => {
                let mut extensions: Vec<GraphQLExtension> = self.component.extensions.iter().map(|extension| extension.value().into()).collect();
                if sort.unwrap_or_default() {
                    extensions.sort();
                }
                extensions
            }
        }
    }

    /// The count of extensions.
    async fn count_extensions(&self) -> usize {
        self.component.extensions.len()
    }

    /// Query which entity types are using this component
    async fn entity_types(&self, context: &Context<'_>) -> Vec<GraphQLEntityType> {
        let Ok(entity_type_manager) = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>() else {
            return Vec::new();
        };
        let entity_types: GraphQLEntityTypes = entity_type_manager.get_by_having_component(&self.component.ty).into();
        entity_types.into()
    }

    /// Query which relation types are using this component
    async fn relation_types(&self, context: &Context<'_>) -> Vec<GraphQLRelationType> {
        let Ok(relation_type_manager) = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>() else {
            return Vec::new();
        };
        let relation_types: GraphQLRelationTypes = relation_type_manager.get_by_having_component(&self.component.ty).into();
        relation_types.into()
    }

    /// Query which relation types are using this component as outbound type
    async fn outbound_of(&self, context: &Context<'_>) -> Result<Vec<GraphQLRelationType>> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        Ok(relation_type_manager
            .get_all()
            .iter()
            .filter(|relation_type| {
                // Either the outbound type is the component or the outbound type is an entity type having the component
                relation_type.outbound_type.eq_component(&self.component.ty)
                    || entity_type_manager
                        .get_by_having_component(&self.component.ty)
                        .iter()
                        .any(|e| relation_type.outbound_type.eq_entity_type(&e.ty))
            })
            // .cloned()
            .map(|relation_type| relation_type.value().clone().into())
            .collect())
    }

    /// Query which relation types are using this component as inbound type
    async fn inbound_of(&self, context: &Context<'_>) -> Result<Vec<GraphQLRelationType>> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        Ok(relation_type_manager
            .get_all()
            .iter()
            .filter(|relation_type| {
                // Either the outbound type is the component or the outbound type is an entity type having the component
                relation_type.inbound_type.eq_component(&self.component.ty)
                    || entity_type_manager
                        .get_by_having_component(&self.component.ty)
                        .iter()
                        .any(|e| relation_type.inbound_type.eq_entity_type(&e.ty))
            })
            // .cloned()
            .map(|relation_type| relation_type.value().clone().into())
            .collect())
    }

    async fn behaviours(&self, context: &Context<'_>) -> Result<Vec<GraphQLComponentBehaviour>> {
        let entity_component_behaviour_registry = context.data::<Arc<dyn EntityComponentBehaviourRegistry + Send + Sync>>()?;
        let relation_component_behaviour_registry = context.data::<Arc<dyn RelationComponentBehaviourRegistry + Send + Sync>>()?;
        let entity_component_behaviour_types: Vec<GraphQLComponentBehaviour> = entity_component_behaviour_registry
            .get_behaviour_types(&self.component.ty)
            .iter()
            .map(|component_behaviour_ty| GraphQLComponentBehaviour::from(component_behaviour_ty.clone()))
            .collect();
        let relation_component_behaviour_types: Vec<GraphQLComponentBehaviour> = relation_component_behaviour_registry
            .get_behaviour_types(&self.component.ty)
            .iter()
            .map(|component_behaviour_ty| GraphQLComponentBehaviour::from(component_behaviour_ty.clone()))
            .collect();
        let component_behaviour_types = vec![entity_component_behaviour_types, relation_component_behaviour_types]
            .into_iter()
            .flatten()
            .collect();
        Ok(component_behaviour_types)
    }

    // /// Type category.
    // async fn type_category(&self) -> Option<GraphQLTypeCategory> {
    //     get_type_category_extension(&self.component)
    //         .and_then(get_type_category)
    //         .map(|category| category.into())
    // }
}

impl From<Component> for GraphQLComponent {
    fn from(component: Component) -> Self {
        GraphQLComponent { component }
    }
}
