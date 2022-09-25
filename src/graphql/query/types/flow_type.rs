use std::sync::Arc;

use async_graphql::*;

use crate::api::EntityTypeManager;
use crate::graphql::query::GraphQLEntityType;
use crate::graphql::query::GraphQLExtension;
use crate::graphql::query::GraphQLPropertyType;
use crate::model::FlowType;

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
            return entity_type_manager.get(self.flow_type.type_name.as_str()).map(|e| e.into());
        }
        None
    }

    /// The name of the flow type.
    ///
    /// The name is the unique identifier for flow types.
    async fn name(&self) -> String {
        self.flow_type.name.clone()
    }

    /// The namespace the flow type belongs to.
    async fn namespace(&self) -> String {
        self.flow_type.namespace.clone()
    }

    /// Textual description of the flow type.
    async fn description(&self) -> String {
        self.flow_type.description.clone()
    }

    // TODO: Entity Instances (these are no reactive entity instances!)
    // TODO: Relation Instances (these are no reactive relation instances!)

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
}

impl From<FlowType> for GraphQLFlowType {
    fn from(flow_type: FlowType) -> Self {
        GraphQLFlowType { flow_type }
    }
}
