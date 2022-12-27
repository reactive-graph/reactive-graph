use std::sync::Arc;

use async_graphql::*;

use crate::api::EntityTypeManager;
use crate::api::FlowTypeManager;
use crate::graphql::query::GraphQLEntityInstance;
use crate::graphql::query::GraphQLEntityType;
use crate::graphql::query::GraphQLExtension;
use crate::graphql::query::GraphQLPropertyInstance;
use crate::graphql::query::GraphQLPropertyType;
use crate::graphql::query::GraphQLRelationInstance;
use crate::model::FlowType;
use crate::model::NamespacedTypeGetter;
use crate::model::ReactiveEntityInstance;
use crate::model::ReactiveRelationInstance;

pub struct GraphQLFlowType {
    flow_type: FlowType,
}

/// Flow types are templates for flow instances.
#[Object(name = "FlowType")]
impl GraphQLFlowType {
    /// The entity type of the flow type.
    #[graphql(name = "type")]
    async fn entity_type(&self, context: &Context<'_>) -> Option<GraphQLEntityType> {
        if let Ok(entity_type_manager) = context.data::<Arc<dyn EntityTypeManager>>() {
            return entity_type_manager.get(&self.flow_type.wrapper_entity_instance.ty).map(|e| e.into());
        }
        None
    }

    /// The namespace the flow type belongs to.
    async fn namespace(&self) -> String {
        self.flow_type.namespace()
    }

    /// The name of the flow type.
    ///
    /// The name is the unique identifier for flow types of the same namespace.
    async fn name(&self) -> String {
        self.flow_type.type_name()
    }

    /// Textual description of the flow type.
    async fn description(&self) -> String {
        self.flow_type.description.clone()
    }

    /// The entity instances contained by the flow type
    async fn entity_instances(&self) -> Vec<GraphQLEntityInstance> {
        self.flow_type
            .entity_instances
            .iter()
            .map(|entity_instance| {
                // Construct a temporary instance, do not register!
                let entity_instance: GraphQLEntityInstance = Arc::new(ReactiveEntityInstance::from(entity_instance.clone())).into();
                entity_instance
            })
            .collect()
    }

    async fn count_entity_instances(&self) -> usize {
        self.flow_type.entity_instances.len()
    }

    /// The relation instances contained by the flow type
    async fn relation_instances(&self) -> Vec<GraphQLRelationInstance> {
        self.flow_type
            .relation_instances
            .iter()
            .filter_map(|relation_instance| {
                // Construct temporary entity instances and a temporary relation instance, do not register!
                let outbound = if relation_instance.outbound_id == self.flow_type.wrapper_entity_instance.id {
                    Arc::new(ReactiveEntityInstance::from(self.flow_type.wrapper_entity_instance.clone()))
                } else {
                    self.flow_type
                        .entity_instances
                        .iter()
                        .find(|e| e.id == relation_instance.outbound_id)
                        .map(|e| Arc::new(ReactiveEntityInstance::from(e.clone())))?
                };
                let inbound = if relation_instance.inbound_id == self.flow_type.wrapper_entity_instance.id {
                    Arc::new(ReactiveEntityInstance::from(self.flow_type.wrapper_entity_instance.clone()))
                } else {
                    self.flow_type
                        .entity_instances
                        .iter()
                        .find(|e| e.id == relation_instance.inbound_id)
                        .map(|e| Arc::new(ReactiveEntityInstance::from(e.clone())))?
                };
                let relation_instance = Arc::new(ReactiveRelationInstance::new_from_instance(outbound, inbound, relation_instance.clone()));
                let relation_instance: GraphQLRelationInstance = relation_instance.into();
                Some(relation_instance)
            })
            .collect()
    }

    async fn count_relation_instances(&self) -> usize {
        self.flow_type.relation_instances.len()
    }

    /// The properties of the flow type.
    /// This are the properties of the wrapper entity instance.
    async fn properties(
        &self,
        #[graphql(desc = "Filters by property name")] name: Option<String>,
        #[graphql(desc = "Filters by property names")] names: Option<Vec<String>>,
    ) -> Vec<GraphQLPropertyInstance> {
        self.flow_type
            .wrapper_entity_instance
            .properties
            .iter()
            .filter(|(property_name, _)| name.is_none() || name.clone().unwrap().as_str() == property_name.as_str())
            .filter(|(property_name, _)| names.is_none() || names.clone().unwrap().contains(property_name))
            .map(|(property_name, property_value)| {
                // TODO: Resolve mutability using entity_type_manager and property_name
                GraphQLPropertyInstance::new_entity_property(self.flow_type.wrapper_entity_instance.ty.clone(), property_name.clone(), property_value.clone())
            })
            .collect()
    }

    /// The variables of the flow type.
    async fn variables(&self, name: Option<String>) -> Vec<GraphQLPropertyType> {
        if name.is_some() {
            let name = name.unwrap();
            return self
                .flow_type
                .variables
                .to_vec()
                .iter()
                .filter(|property_type| property_type.name == name.clone())
                .cloned()
                .map(|property_type| property_type.into())
                .collect();
        }
        self.flow_type.variables.iter().cloned().map(|property_type| property_type.into()).collect()
    }

    /// The extensions which are defined by the flow type.
    async fn extensions(&self, name: Option<String>) -> Vec<GraphQLExtension> {
        if name.is_some() {
            let name = name.unwrap();
            return self
                .flow_type
                .extensions
                .to_vec()
                .iter()
                .filter(|extension| extension.name == name.clone())
                .cloned()
                .map(|extension| extension.into())
                .collect();
        }
        self.flow_type.extensions.iter().cloned().map(|extension| extension.into()).collect()
    }

    /// Returns true, if the relation type is valid.
    ///
    /// This means all components exists and the outbound and inbound entity types are valid.
    async fn is_valid(&self, context: &Context<'_>) -> bool {
        match context.data::<Arc<dyn FlowTypeManager>>() {
            Ok(flow_type_manager) => flow_type_manager.validate(&self.flow_type.ty),
            Err(_) => false,
        }
    }
}

impl From<FlowType> for GraphQLFlowType {
    fn from(flow_type: FlowType) -> Self {
        GraphQLFlowType { flow_type }
    }
}
