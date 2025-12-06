use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;

use crate::validator::NamespacedTypeValidator;
use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::ComponentTypeIdContainer;
use reactive_graph_graph::ComponentTypeIds;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::ExtensionContainer;
use reactive_graph_graph::ExtensionTypeIds;
use reactive_graph_graph::FlowTypeId;
use reactive_graph_graph::InboundOutboundType;
use reactive_graph_graph::MatchingInboundOutboundType;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::PropertyTypeContainer;
use reactive_graph_graph::RelationTypeId;
use reactive_graph_graph::VariablesContainer;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::FlowTypeManager;
use reactive_graph_type_system_api::NamespacedTypeManager;
use reactive_graph_type_system_api::RelationTypeManager;

use crate::query::GraphQLComponent;
use crate::query::GraphQLEntityType;
use crate::query::GraphQLFlowType;
use crate::query::GraphQLRelationType;

#[derive(Default)]
pub struct Types;

/// Search for types (components, entity types or relation types)
#[Object]
impl Types {
    /// Search for components
    ///
    /// Optionally the list of components can be filtered by name.
    /// "^[a-z_]+(?:::[a-z_]+)*(?:::([A-Z][a-zA-Z0-9]*))$"
    async fn components(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the component.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: Option<String>,
        #[graphql(name = "namespace", desc = "Searches by the namespace of the components.")] namespace: Option<String>,
        #[graphql(desc = "Searches by the name of the components. Allowed wildcards are: ? and *")] search: Option<String>,
        #[graphql(desc = "Filters by having all of these properties.")] properties: Option<Vec<String>>,
        #[graphql(desc = "Filters by having all of these extensions.")] extensions: Option<Vec<String>>,
    ) -> Result<Vec<GraphQLComponent>> {
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;

        // Return the specified component
        if let Some(ty) = ComponentTypeId::parse_optional_namespace(_type)? {
            return match component_manager.get(&ty) {
                Some(component) => Ok(vec![component.into()]),
                None => Ok(vec![]),
            };
        }

        let extensions = ExtensionTypeIds::parse_optional_namespaces(extensions).unwrap_or_default();

        // Namespace search
        if let Some(namespace) = namespace {
            let namespace = Namespace::from_str(&namespace)?;
            let components = component_manager
                .get_by_namespace(&namespace)
                .iter_mut()
                .filter(|component| {
                    properties
                        .as_ref()
                        .map(|properties| component.has_all_own_properties(properties))
                        .unwrap_or(true)
                })
                .filter(|component| {
                    component.has_all_own_extensions(&extensions)
                    // extensions
                    //     // .as_ref()
                    //     .map(|extensions| component.has_all_own_extensions(extensions))
                    //     .unwrap_or(true)
                })
                .map(|component| {
                    let component: GraphQLComponent = component.clone().into();
                    component
                })
                .collect();
            return Ok(components);
        }

        // Type name search
        if let Some(search) = search {
            let components = component_manager
                .find(&search)
                .iter_mut()
                .filter(|component| {
                    properties
                        .as_ref()
                        .map(|properties| component.has_all_own_properties(properties))
                        .unwrap_or(true)
                })
                .filter(|component| {
                    component.has_all_own_extensions(&extensions)
                    // extensions
                    //     .as_ref()
                    //     .map(|extensions| component.has_all_own_extensions(extensions))
                    //     .unwrap_or(true)
                })
                .map(|component| {
                    let component: GraphQLComponent = component.clone().into();
                    component
                })
                .collect();
            return Ok(components);
        }

        // Apply filters only
        let components = component_manager
            .get_all()
            .iter_mut()
            .filter(|component| {
                properties
                    .as_ref()
                    .map(|properties| component.has_all_own_properties(properties))
                    .unwrap_or(true)
            })
            .filter(|component| {
                component.has_all_own_extensions(&extensions)
                // extensions
                //     .as_ref()
                //     .map(|extensions| component.has_all_own_extensions(extensions))
                //     .unwrap_or(true)
            })
            .map(|component| {
                let component: GraphQLComponent = component.clone().into();
                component
            })
            .collect();
        Ok(components)
    }

    async fn count_components(&self, context: &Context<'_>) -> Result<usize> {
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;
        Ok(component_manager.count())
    }

    /// Search for entity types.
    ///
    /// Optionally the list of entity types can be filtered by name.
    async fn entities(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the entity type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: Option<String>,
        #[graphql(name = "namespace", desc = "Searches by the namespace of the entity types.")] namespace: Option<String>,
        #[graphql(desc = "Searches by the name of the entity types. Allowed wildcards are: ? and *")] search: Option<String>,
        #[graphql(desc = "Filters by having all of these properties.")] properties: Option<Vec<String>>,
        #[graphql(desc = "Filters by having all of these components.")] components: Option<Vec<String>>,
        #[graphql(desc = "Filters by having all of these extensions.")] extensions: Option<Vec<String>>,
    ) -> Result<Vec<GraphQLEntityType>> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;

        // Return the specified entity type
        if let Some(ty) = EntityTypeId::parse_optional_namespace(_type)? {
            return match entity_type_manager.get(&ty.into()) {
                Some(entity_type) => Ok(vec![entity_type.into()]),
                None => Ok(vec![]),
            };
        }

        let components = ComponentTypeIds::parse_optional_namespaces(components)?;
        let extensions = ExtensionTypeIds::parse_optional_namespaces(extensions).unwrap_or_default();

        // Search entity type by namespace
        if let Some(namespace) = namespace {
            let namespace = Namespace::from_str(&namespace)?;
            let entity_types = entity_type_manager
                .get_by_namespace(&namespace)
                .iter_mut()
                .filter(|entity_type| {
                    properties
                        .as_ref()
                        .map(|properties| entity_type.has_all_own_properties(properties))
                        .unwrap_or(true)
                })
                .filter(|entity_type| entity_type.is_all(&components))
                .filter(|entity_type| entity_type.has_all_own_extensions(&extensions))
                .map(|entity_type| {
                    let entity_type: GraphQLEntityType = entity_type.clone().into();
                    entity_type
                })
                .collect();
            return Ok(entity_types);
        }

        // Search entity type by fully qualified type name
        if let Some(search) = search {
            let entity_types = entity_type_manager
                .find(&search)
                .iter_mut()
                .filter(|entity_type| {
                    properties
                        .as_ref()
                        .map(|properties| entity_type.has_all_own_properties(properties))
                        .unwrap_or(true)
                })
                .filter(|entity_type| entity_type.is_all(&components))
                .filter(|entity_type| entity_type.has_all_own_extensions(&extensions))
                .map(|entity_type| {
                    let entity_type: GraphQLEntityType = entity_type.clone().into();
                    entity_type
                })
                .collect();
            return Ok(entity_types);
        }

        // Apply filters only
        let entity_types = entity_type_manager
            .get_all()
            .iter_mut()
            .filter(|entity_type| {
                properties
                    .as_ref()
                    .map(|properties| entity_type.has_all_own_properties(properties))
                    .unwrap_or(true)
            })
            .filter(|entity_type| entity_type.is_all(&components))
            .filter(|entity_type| entity_type.has_all_own_extensions(&extensions))
            .map(|entity_type| {
                let entity_type: GraphQLEntityType = entity_type.clone().into();
                entity_type
            })
            .collect();
        Ok(entity_types)
    }

    async fn count_entity_types(&self, context: &Context<'_>) -> Result<usize> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        Ok(entity_type_manager.count())
    }

    /// Search for relation types.
    #[allow(clippy::too_many_arguments)]
    async fn relations(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the relation type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: Option<String>,
        #[graphql(name = "namespace", desc = "Searches by the namespace of the components.")] namespace: Option<String>,
        #[graphql(desc = "Searches by the name of the relation types. Allowed wildcards are: ? and *")] search: Option<String>,
        #[graphql(name = "outboundComponent", desc = "Filters by outbound component")] outbound_component_namespace: Option<String>,
        #[graphql(name = "outboundEntityType", desc = "Filters by outbound entity type")] outbound_entity_namespace: Option<String>,
        #[graphql(name = "inboundComponent", desc = "Filters by inbound component")] inbound_component_namespace: Option<String>,
        #[graphql(name = "inboundEntityType", desc = "Filters by inbound entity type")] inbound_entity_namespace: Option<String>,
        #[graphql(desc = "Filters by having all of these properties.")] properties: Option<Vec<String>>,
        #[graphql(desc = "Filters by having all of these components.")] components: Option<Vec<String>>,
        #[graphql(desc = "Filters by having all of these extensions.")] extensions: Option<Vec<String>>,
    ) -> Result<Vec<GraphQLRelationType>> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;

        // Return the specified relation type
        if let Some(ty) = RelationTypeId::parse_optional_namespace(_type)? {
            return match relation_type_manager.get(&ty.into()) {
                Some(relation_type) => Ok(vec![relation_type.into()]),
                None => Ok(vec![]),
            };
        }

        let outbound_component_ty = match &outbound_component_namespace {
            Some(outbound_component_namespace) => Some(ComponentTypeId::from_str(outbound_component_namespace)?),
            None => None,
        };
        let outbound_entity_ty = match &outbound_entity_namespace {
            Some(outbound_entity_namespace) => Some(EntityTypeId::from_str(outbound_entity_namespace)?),
            None => None,
        };
        let inbound_component_ty = match &inbound_component_namespace {
            Some(inbound_component_namespace) => Some(ComponentTypeId::from_str(inbound_component_namespace)?),
            None => None,
        };
        let inbound_entity_ty = match &inbound_entity_namespace {
            Some(inbound_entity_namespace) => Some(EntityTypeId::from_str(inbound_entity_namespace)?),
            None => None,
        };
        let components = ComponentTypeIds::parse_optional_namespaces(components)?;
        let extensions = ExtensionTypeIds::parse_optional_namespaces(extensions)?;

        // Namespace search
        if let Some(namespace) = namespace {
            let namespace = Namespace::from_str(&namespace)?;
            let relation_types = relation_type_manager
                .get_by_namespace(&namespace)
                .iter_mut()
                .filter(|relation_type| match &outbound_component_ty {
                    Some(outbound_component_ty) => match &relation_type.outbound_type {
                        InboundOutboundType::Component(MatchingInboundOutboundType::NamespacedType(component_ty)) => component_ty == outbound_component_ty,
                        _ => true,
                    },
                    None => true,
                })
                .filter(|relation_type| match &outbound_entity_ty {
                    Some(outbound_entity_ty) => match &relation_type.outbound_type {
                        InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(entity_ty)) => entity_ty == outbound_entity_ty,
                        _ => true,
                    },
                    None => true,
                })
                .filter(|relation_type| match &inbound_component_ty {
                    Some(inbound_component_ty) => match &relation_type.inbound_type {
                        InboundOutboundType::Component(MatchingInboundOutboundType::NamespacedType(component_ty)) => component_ty == inbound_component_ty,
                        _ => true,
                    },
                    None => true,
                })
                .filter(|relation_type| match &inbound_entity_ty {
                    Some(inbound_entity_ty) => match &relation_type.inbound_type {
                        InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(entity_ty)) => entity_ty == inbound_entity_ty,
                        _ => true,
                    },
                    None => true,
                })
                .filter(|relation_type| {
                    properties
                        .as_ref()
                        .map(|properties| relation_type.has_all_own_properties(properties))
                        .unwrap_or(true)
                })
                .filter(|relation_type| relation_type.is_all(&components))
                .filter(|relation_type| relation_type.has_all_own_extensions(&extensions))
                .map(|relation_type| {
                    let relation_type: GraphQLRelationType = relation_type.clone().into();
                    relation_type
                })
                .collect();
            return Ok(relation_types);
        }

        // Type name search
        if let Some(search) = search {
            let relation_types = relation_type_manager
                .find(&search)
                .iter_mut()
                .filter(|relation_type| match &outbound_component_ty {
                    Some(outbound_component_ty) => match &relation_type.outbound_type {
                        InboundOutboundType::Component(MatchingInboundOutboundType::NamespacedType(component_ty)) => component_ty == outbound_component_ty,
                        _ => true,
                    },
                    None => true,
                })
                .filter(|relation_type| match &outbound_entity_ty {
                    Some(outbound_entity_ty) => match &relation_type.outbound_type {
                        InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(entity_ty)) => entity_ty == outbound_entity_ty,
                        _ => true,
                    },
                    None => true,
                })
                .filter(|relation_type| match &inbound_component_ty {
                    Some(inbound_component_ty) => match &relation_type.inbound_type {
                        InboundOutboundType::Component(MatchingInboundOutboundType::NamespacedType(component_ty)) => component_ty == inbound_component_ty,
                        _ => true,
                    },
                    None => true,
                })
                .filter(|relation_type| match &inbound_entity_ty {
                    Some(inbound_entity_ty) => match &relation_type.inbound_type {
                        InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(entity_ty)) => entity_ty == inbound_entity_ty,
                        _ => true,
                    },
                    None => true,
                })
                .filter(|relation_type| {
                    properties
                        .as_ref()
                        .map(|properties| relation_type.has_all_own_properties(properties))
                        .unwrap_or(true)
                })
                .filter(|relation_type| relation_type.is_all(&components))
                .filter(|relation_type| relation_type.has_all_own_extensions(&extensions))
                .map(|relation_type| {
                    let relation_type: GraphQLRelationType = relation_type.clone().into();
                    relation_type
                })
                .collect();
            return Ok(relation_types);
        }

        // Apply filters only
        let relation_types = relation_type_manager
            .get_all()
            .iter_mut()
            .filter(|relation_type| match &outbound_component_ty {
                Some(outbound_component_ty) => match &relation_type.outbound_type {
                    InboundOutboundType::Component(MatchingInboundOutboundType::NamespacedType(component_ty)) => component_ty == outbound_component_ty,
                    _ => true,
                },
                None => true,
            })
            .filter(|relation_type| match &outbound_entity_ty {
                Some(outbound_entity_ty) => match &relation_type.outbound_type {
                    InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(entity_ty)) => entity_ty == outbound_entity_ty,
                    _ => true,
                },
                None => true,
            })
            .filter(|relation_type| match &inbound_component_ty {
                Some(inbound_component_ty) => match &relation_type.inbound_type {
                    InboundOutboundType::Component(MatchingInboundOutboundType::NamespacedType(component_ty)) => component_ty == inbound_component_ty,
                    _ => true,
                },
                None => true,
            })
            .filter(|relation_type| match &inbound_entity_ty {
                Some(inbound_entity_ty) => match &relation_type.inbound_type {
                    InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(entity_ty)) => entity_ty == inbound_entity_ty,
                    _ => true,
                },
                None => true,
            })
            .filter(|relation_type| {
                properties
                    .as_ref()
                    .map(|properties| relation_type.has_all_own_properties(properties))
                    .unwrap_or(true)
            })
            .filter(|relation_type| relation_type.is_all(&components))
            .filter(|relation_type| relation_type.has_all_own_extensions(&extensions))
            .map(|relation_type| {
                let relation_type: GraphQLRelationType = relation_type.clone().into();
                relation_type
            })
            .collect();
        Ok(relation_types)
    }

    async fn count_relation_types(&self, context: &Context<'_>) -> Result<usize> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        Ok(relation_type_manager.count())
    }

    /// Search for flow types.
    ///
    /// Optionally the list of flow types can be filtered by name.
    async fn flows(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the flow type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: Option<String>,
        #[graphql(name = "namespace", desc = "Searches by the namespace of the flow types.")] namespace: Option<String>,
        #[graphql(desc = "Searches by the name of the flow types. Allowed wildcards are: ? and *")] search: Option<String>,
        #[graphql(desc = "Filters by having all of these variables.")] variables: Option<Vec<String>>,
        #[graphql(desc = "Filters by having all of these extensions.")] extensions: Option<Vec<String>>,
    ) -> Result<Vec<GraphQLFlowType>> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;

        // Return the specified flow type
        if let Some(ty) = FlowTypeId::parse_optional_namespace(_type)? {
            return match flow_type_manager.get(&ty.into()) {
                Some(flow_type) => Ok(vec![flow_type.into()]),
                None => Ok(vec![]),
            };
        }

        let extensions = ExtensionTypeIds::parse_optional_namespaces(extensions).unwrap_or_default();

        // Search flow type by namespace
        if let Some(namespace) = Namespace::parse_optional_namespace(namespace)? {
            let flow_types = flow_type_manager
                .get_by_namespace(&namespace)
                .iter_mut()
                .filter(|flow_type| variables.as_ref().map(|variables| flow_type.has_all_variables(variables)).unwrap_or(true))
                .filter(|flow_type| flow_type.has_all_own_extensions(&extensions))
                .map(|flow_type| {
                    let flow_type: GraphQLFlowType = flow_type.clone().into();
                    flow_type
                })
                .collect();
            return Ok(flow_types);
        }

        // Search flow type by fully qualified type name
        if let Some(search) = search {
            let flow_types = flow_type_manager
                .find(search.as_str())
                .iter_mut()
                .filter(|flow_type| variables.as_ref().map(|variables| flow_type.has_all_variables(variables)).unwrap_or(true))
                .filter(|flow_type| flow_type.has_all_own_extensions(&extensions))
                .map(|flow_type| {
                    let flow_type: GraphQLFlowType = flow_type.clone().into();
                    flow_type
                })
                .collect();
            return Ok(flow_types);
        }

        // Apply filters only
        let flow_types = flow_type_manager
            .get_all()
            .iter_mut()
            .filter(|flow_type| variables.as_ref().map(|variables| flow_type.has_all_variables(variables)).unwrap_or(true))
            .filter(|flow_type| flow_type.has_all_own_extensions(&extensions))
            .map(|flow_type| {
                let flow_type: GraphQLFlowType = flow_type.clone().into();
                flow_type
            })
            .collect();
        Ok(flow_types)
    }

    async fn count_flow_types(&self, context: &Context<'_>) -> Result<usize> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        Ok(flow_type_manager.count())
    }

    async fn namespaces(&self, context: &Context<'_>) -> Result<HashSet<String>> {
        let namespaced_type_manager = context.data::<Arc<dyn NamespacedTypeManager + Send + Sync>>()?;
        let namespaces = namespaced_type_manager.get_all().iter().map(|namespace| namespace.to_string()).collect();
        Ok(namespaces)
    }
}
