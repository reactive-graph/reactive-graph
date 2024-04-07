use std::collections::HashSet;
use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;

use reactive_graph_graph::ComponentOrEntityTypeId;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::ComponentTypeIdContainer;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::ExtensionContainer;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::PropertyTypeContainer;
use reactive_graph_graph::RelationTypeId;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::FlowTypeManager;
use reactive_graph_type_system_api::NamespaceManager;
use reactive_graph_type_system_api::RelationTypeManager;

use crate::mutation::ComponentTypeIdDefinition;
use crate::mutation::EntityTypeIdDefinition;
use crate::mutation::ExtensionTypeIdDefinition;
use crate::mutation::FlowTypeIdDefinition;
use crate::mutation::RelationTypeIdDefinition;
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
    async fn components(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The component type")] ty: Option<ComponentTypeIdDefinition>,
        #[graphql(name = "namespace", desc = "Searches by the namespace of the components.")] namespace: Option<String>,
        #[graphql(desc = "Searches by the name of the components. Allowed wildcards are: ? and *")] search: Option<String>,
        #[graphql(desc = "Filters by having all of these properties.")] properties: Option<Vec<String>>,
        #[graphql(desc = "Filters by having all of these extensions.")] extensions: Option<Vec<ExtensionTypeIdDefinition>>,
    ) -> Result<Vec<GraphQLComponent>> {
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;
        if let Some(ty) = ty {
            return match component_manager.get(&ty.into()) {
                Some(component) => Ok(vec![component.into()]),
                None => Ok(vec![]),
            };
        }
        let extensions = extensions.map(|extensions| extensions.into_iter().map(|ty| ty.into()).collect());
        if let Some(namespace) = namespace {
            let components = component_manager
                .get_by_namespace(&namespace)
                .into_iter()
                .filter(|(_, component)| {
                    properties
                        .as_ref()
                        .map(|properties| component.has_all_own_properties(properties))
                        .unwrap_or(true)
                })
                .filter(|(_, component)| {
                    extensions
                        .as_ref()
                        .map(|extensions| component.has_all_own_extensions(extensions))
                        .unwrap_or(true)
                })
                .map(|(_, component)| component.into())
                .collect();
            return Ok(components);
        }
        if let Some(search) = search {
            let components = component_manager
                .find_by_type_name(&search)
                .into_iter()
                .filter(|(_, component)| {
                    properties
                        .as_ref()
                        .map(|properties| component.has_all_own_properties(properties))
                        .unwrap_or(true)
                })
                .filter(|(_, component)| {
                    extensions
                        .as_ref()
                        .map(|extensions| component.has_all_own_extensions(extensions))
                        .unwrap_or(true)
                })
                .map(|(_, component)| component.into())
                .collect();
            return Ok(components);
        }
        let components = component_manager
            .get_all()
            .into_iter()
            .filter(|(_, component)| {
                properties
                    .as_ref()
                    .map(|properties| component.has_all_own_properties(properties))
                    .unwrap_or(true)
            })
            .filter(|(_, component)| {
                extensions
                    .as_ref()
                    .map(|extensions| component.has_all_own_extensions(extensions))
                    .unwrap_or(true)
            })
            .map(|(_, component)| component.into())
            .collect();
        Ok(components)
    }

    async fn count_components(&self, context: &Context<'_>) -> usize {
        if let Ok(component_manager) = context.data::<Arc<dyn ComponentManager + Send + Sync>>() {
            return component_manager.count();
        }
        0
    }

    /// Search for entity types.
    ///
    /// Optionally the list of entity types can be filtered by name.
    async fn entities(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The entity type")] ty: Option<EntityTypeIdDefinition>,
        // TODO: #[graphql(name = "component_type", desc = "The component type")] ty: Option<ComponentTypeIdDefinition>,
        #[graphql(desc = "Searches by the name of the entity types. Allowed wildcards are: ? and *")] search: Option<String>,
        #[graphql(desc = "Filters by having all of these properties.")] properties: Option<Vec<String>>,
        #[graphql(desc = "Filters by having all of these components.")] components: Option<Vec<ComponentTypeIdDefinition>>,
        #[graphql(desc = "Filters by having all of these extensions.")] extensions: Option<Vec<ExtensionTypeIdDefinition>>,
    ) -> Result<Vec<GraphQLEntityType>> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        if let Some(ty) = ty {
            if let Some(entity_type) = entity_type_manager.get(&ty.into()) {
                let entity_type: GraphQLEntityType = entity_type.into();
                return Ok(vec![entity_type]);
            }
        }
        let components = components.map(|components| components.into_iter().map(|ty| ty.into()).collect());
        let extensions = extensions.map(|extensions| extensions.into_iter().map(|ty| ty.into()).collect());
        if let Some(search) = search {
            let entity_types = entity_type_manager
                .find_by_type_name(&search)
                .into_iter()
                .filter(|(_, entity_type)| {
                    properties
                        .as_ref()
                        .map(|properties| entity_type.has_all_own_properties(properties))
                        .unwrap_or(true)
                })
                .filter(|(_, entity_type)| components.as_ref().map(|components| entity_type.is_all(components)).unwrap_or(true))
                .filter(|(_, entity_type)| {
                    extensions
                        .as_ref()
                        .map(|extensions| entity_type.has_all_own_extensions(extensions))
                        .unwrap_or(true)
                })
                .map(|(_, entity_type)| entity_type.into())
                .collect();
            return Ok(entity_types);
        }
        let entity_types = entity_type_manager
            .get_all()
            .iter()
            .filter(|entity_type| {
                properties
                    .as_ref()
                    .map(|properties| entity_type.has_all_own_properties(properties))
                    .unwrap_or(true)
            })
            .filter(|entity_type| components.as_ref().map(|components| entity_type.is_all(components)).unwrap_or(true))
            .filter(|entity_type| {
                extensions
                    .as_ref()
                    .map(|extensions| entity_type.has_all_own_extensions(extensions))
                    .unwrap_or(true)
            })
            .map(|entity_type| {
                let entity_type: GraphQLEntityType = entity_type.clone().into();
                entity_type
            })
            .collect();
        Ok(entity_types)
    }

    async fn count_entity_types(&self, context: &Context<'_>) -> usize {
        if let Ok(entity_type_manager) = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>() {
            return entity_type_manager.count();
        }
        0
    }

    /// Search for relation types.
    async fn relations(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The relation type.")] ty: Option<RelationTypeIdDefinition>,
        #[graphql(desc = "Searches by the name of the relation types. Allowed wildcards are: ? and *")] search: Option<String>,
        #[graphql(desc = "Filters by outbound component")] outbound_component: Option<ComponentTypeIdDefinition>,
        #[graphql(desc = "Filters by outbound entity type")] outbound_entity_type: Option<EntityTypeIdDefinition>,
        #[graphql(desc = "Filters by inbound component")] inbound_component: Option<ComponentTypeIdDefinition>,
        #[graphql(desc = "Filters by inbound entity type")] inbound_entity_type: Option<EntityTypeIdDefinition>,
        #[graphql(desc = "Filters by having all of these properties.")] properties: Option<Vec<String>>,
        #[graphql(desc = "Filters by having all of these components.")] components: Option<Vec<ComponentTypeIdDefinition>>,
        #[graphql(desc = "Filters by having all of these extensions.")] extensions: Option<Vec<ExtensionTypeIdDefinition>>,
    ) -> Result<Vec<GraphQLRelationType>> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let ty: Option<RelationTypeId> = ty.map(|ty| ty.into());
        let outbound_component: Option<ComponentTypeId> = outbound_component.map(|o| o.into());
        let outbound_entity_type: Option<EntityTypeId> = outbound_entity_type.map(|o| o.into());
        let inbound_component: Option<ComponentTypeId> = inbound_component.map(|o| o.into());
        let inbound_entity_type: Option<EntityTypeId> = inbound_entity_type.map(|o| o.into());
        let components = components.map(|components| components.into_iter().map(|ty| ty.into()).collect());
        let extensions = extensions.map(|extensions| extensions.into_iter().map(|ty| ty.into()).collect());

        // Return the specified relation type
        if let Some(ty) = ty {
            // Exact search
            if !ty.type_name().is_empty() && !ty.namespace().is_empty() {
                if let Some(relation_type) = relation_type_manager.get(&ty) {
                    let relation_type: GraphQLRelationType = relation_type.into();
                    return Ok(vec![relation_type]);
                }
            }

            // Search by type name
            if !ty.type_name().is_empty() {
                let relation_types = relation_type_manager
                    .find_by_type_name(&ty.type_name())
                    .iter()
                    .filter(|relation_type| relation_type.type_name() == ty.type_name())
                    .filter(|relation_type| {
                        outbound_component.is_none() || {
                            match &relation_type.outbound_type {
                                ComponentOrEntityTypeId::Component(component_ty) => component_ty == &outbound_component.clone().unwrap(),
                                ComponentOrEntityTypeId::EntityType(_) => true,
                            }
                        }
                    })
                    .filter(|relation_type| {
                        outbound_entity_type.is_none() || {
                            match &relation_type.outbound_type {
                                ComponentOrEntityTypeId::EntityType(entity_ty) => entity_ty == &outbound_entity_type.clone().unwrap(),
                                ComponentOrEntityTypeId::Component(_) => true,
                            }
                        }
                    })
                    .filter(|relation_type| {
                        inbound_component.is_none() || {
                            match &relation_type.outbound_type {
                                ComponentOrEntityTypeId::Component(component_ty) => component_ty == &inbound_component.clone().unwrap(),
                                ComponentOrEntityTypeId::EntityType(_) => true,
                            }
                        }
                    })
                    .filter(|relation_type| {
                        inbound_entity_type.is_none() || {
                            match &relation_type.outbound_type {
                                ComponentOrEntityTypeId::EntityType(entity_ty) => entity_ty == &inbound_entity_type.clone().unwrap(),
                                ComponentOrEntityTypeId::Component(_) => true,
                            }
                        }
                    })
                    .filter(|relation_type| {
                        properties
                            .as_ref()
                            .map(|properties| relation_type.has_all_own_properties(properties))
                            .unwrap_or(true)
                    })
                    .filter(|relation_type| components.as_ref().map(|components| relation_type.is_all(components)).unwrap_or(true))
                    .filter(|relation_type| {
                        extensions
                            .as_ref()
                            .map(|extensions| relation_type.has_all_own_extensions(extensions))
                            .unwrap_or(true)
                    })
                    .map(|relation_type| relation_type.value().clone().into())
                    .collect();
                return Ok(relation_types);
            }

            // Search by namespace
            if ty.type_name().is_empty() && !ty.namespace().is_empty() {
                let relation_types = relation_type_manager
                    .get_by_namespace(&ty.namespace())
                    .iter()
                    .filter(|relation_type| {
                        outbound_component.is_none() || {
                            match &relation_type.outbound_type {
                                ComponentOrEntityTypeId::Component(component_ty) => component_ty == &outbound_component.clone().unwrap(),
                                ComponentOrEntityTypeId::EntityType(_) => true,
                            }
                        }
                    })
                    .filter(|relation_type| {
                        outbound_entity_type.is_none() || {
                            match &relation_type.outbound_type {
                                ComponentOrEntityTypeId::EntityType(entity_ty) => entity_ty == &outbound_entity_type.clone().unwrap(),
                                ComponentOrEntityTypeId::Component(_) => true,
                            }
                        }
                    })
                    .filter(|relation_type| {
                        inbound_component.is_none() || {
                            match &relation_type.outbound_type {
                                ComponentOrEntityTypeId::Component(component_ty) => component_ty == &inbound_component.clone().unwrap(),
                                ComponentOrEntityTypeId::EntityType(_) => true,
                            }
                        }
                    })
                    .filter(|relation_type| {
                        inbound_entity_type.is_none() || {
                            match &relation_type.outbound_type {
                                ComponentOrEntityTypeId::EntityType(entity_ty) => entity_ty == &inbound_entity_type.clone().unwrap(),
                                ComponentOrEntityTypeId::Component(_) => true,
                            }
                        }
                    })
                    .filter(|relation_type| {
                        properties
                            .as_ref()
                            .map(|properties| relation_type.has_all_own_properties(properties))
                            .unwrap_or(true)
                    })
                    .filter(|relation_type| components.as_ref().map(|components| relation_type.is_all(components)).unwrap_or(true))
                    .filter(|relation_type| {
                        extensions
                            .as_ref()
                            .map(|extensions| relation_type.has_all_own_extensions(extensions))
                            .unwrap_or(true)
                    })
                    .map(|relation_type| relation_type.value().clone().into())
                    .collect();
                return Ok(relation_types);
            }
        }

        if let Some(search) = search {
            let relation_types = relation_type_manager
                .find_by_type_name(&search)
                .into_iter()
                .map(|(_, relation_type)| relation_type.clone().into())
                .collect();
            return Ok(relation_types);
        }

        // Search all
        let relation_types = relation_type_manager
            .get_all()
            .iter()
            .filter(|relation_type| {
                outbound_component.is_none() || {
                    match &relation_type.outbound_type {
                        ComponentOrEntityTypeId::Component(component_ty) => component_ty == &outbound_component.clone().unwrap(),
                        ComponentOrEntityTypeId::EntityType(_) => true,
                    }
                }
            })
            .filter(|relation_type| {
                outbound_entity_type.is_none() || {
                    match &relation_type.outbound_type {
                        ComponentOrEntityTypeId::EntityType(entity_ty) => entity_ty == &outbound_entity_type.clone().unwrap(),
                        ComponentOrEntityTypeId::Component(_) => true,
                    }
                }
            })
            .filter(|relation_type| {
                inbound_component.is_none() || {
                    match &relation_type.outbound_type {
                        ComponentOrEntityTypeId::Component(component_ty) => component_ty == &inbound_component.clone().unwrap(),
                        ComponentOrEntityTypeId::EntityType(_) => true,
                    }
                }
            })
            .filter(|relation_type| {
                inbound_entity_type.is_none() || {
                    match &relation_type.outbound_type {
                        ComponentOrEntityTypeId::EntityType(entity_ty) => entity_ty == &inbound_entity_type.clone().unwrap(),
                        ComponentOrEntityTypeId::Component(_) => true,
                    }
                }
            })
            .filter(|relation_type| {
                properties
                    .as_ref()
                    .map(|properties| relation_type.has_all_own_properties(properties))
                    .unwrap_or(true)
            })
            .filter(|relation_type| components.as_ref().map(|components| relation_type.is_all(components)).unwrap_or(true))
            .filter(|relation_type| {
                extensions
                    .as_ref()
                    .map(|extensions| relation_type.has_all_own_extensions(extensions))
                    .unwrap_or(true)
            })
            .map(|relation_type| relation_type.value().clone().into())
            .collect();
        Ok(relation_types)
    }

    async fn count_relation_types(&self, context: &Context<'_>) -> usize {
        if let Ok(relation_type_manager) = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>() {
            return relation_type_manager.count();
        }
        0
    }

    /// Search for flow types.
    ///
    /// Optionally the list of flow types can be filtered by name.
    async fn flows(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The flow type")] ty: Option<FlowTypeIdDefinition>,
        #[graphql(desc = "Searches by the name of the flow types. Allowed wildcards are: ? and *")] search: Option<String>,
        #[graphql(desc = "Filters by having all of these extensions.")] extensions: Option<Vec<ExtensionTypeIdDefinition>>,
    ) -> Result<Vec<GraphQLFlowType>> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        if let Some(ty) = ty {
            let ty = ty.into();
            if let Some(flow_type) = flow_type_manager.get(&ty) {
                let flow_type: GraphQLFlowType = flow_type.into();
                return Ok(vec![flow_type]);
            }
        }
        let extensions = extensions.map(|extensions| extensions.into_iter().map(|ty| ty.into()).collect());
        if search.is_some() {
            let flow_types = flow_type_manager
                .find_by_type_name(search.unwrap().as_str())
                .into_iter()
                .map(|(_, flow_type)| flow_type.clone().into())
                .collect();
            return Ok(flow_types);
        }
        let flow_types = flow_type_manager
            .get_all()
            .iter()
            .filter(|flow_type| {
                extensions
                    .as_ref()
                    .map(|extensions| flow_type.has_all_own_extensions(extensions))
                    .unwrap_or(true)
            })
            .map(|flow_type| {
                let flow_type: GraphQLFlowType = flow_type.clone().into();
                flow_type
            })
            .collect();
        Ok(flow_types)
    }

    async fn count_flow_types(&self, context: &Context<'_>) -> usize {
        if let Ok(flow_type_manager) = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>() {
            return flow_type_manager.count();
        }
        0
    }

    async fn namespaces(&self, context: &Context<'_>) -> HashSet<String> {
        let Ok(namespace_manager) = context.data::<Arc<dyn NamespaceManager + Send + Sync>>() else {
            return HashSet::new();
        };
        namespace_manager.get_all()
    }
}
