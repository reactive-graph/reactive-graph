use std::sync::Arc;

use async_graphql::*;
use log::debug;

use crate::api::FlowTypeManager;
use crate::builder::FlowTypeBuilder;
use crate::graphql::mutation::GraphQLEntityInstanceDefinition;
use crate::graphql::mutation::GraphQLRelationInstanceDefinition;
use crate::graphql::mutation::PropertyTypeDefinition;
use crate::graphql::query::GraphQLExtension;
use crate::graphql::query::GraphQLFlowType;

#[derive(Default)]
pub struct MutationFlowTypes;

/// Mutations for flow types
#[Object]
impl MutationFlowTypes {
    /// Creates a new flow type with the given name and components and properties.
    async fn create(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "The namespace the flow type belongs to.")] namespace: Option<String>,
        #[graphql(desc = "The name of the entity type.")] type_name: String,
        #[graphql(desc = "The name of the flow type.")] name: String,
        entity_instances: Option<Vec<GraphQLEntityInstanceDefinition>>,
        relation_instances: Option<Vec<GraphQLRelationInstanceDefinition>>,
        #[graphql(desc = "The variables of the flow type.")] variables: Option<Vec<PropertyTypeDefinition>>,
        #[graphql(desc = "The extension on the entity type.")] extensions: Option<Vec<GraphQLExtension>>,
    ) -> Result<GraphQLFlowType> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager>>()?;

        let namespace = namespace.unwrap_or_default();
        if flow_type_manager.has(&name) {
            return Err(Error::new(format!("Flow type {} already exists", name)));
        }

        let mut flow_type_builder = FlowTypeBuilder::new(&namespace, &name, &type_name);
        if entity_instances.is_some() {
            let entity_instances = entity_instances.unwrap();
            for entity_instance in entity_instances {
                flow_type_builder.entity_instance(entity_instance.into());
            }
        }
        if relation_instances.is_some() {
            let relation_instances = relation_instances.unwrap();
            for relation_instance in relation_instances {
                flow_type_builder.relation_instance(relation_instance.into());
            }
        }
        if variables.is_some() {
            for variable in variables.unwrap() {
                debug!("{} {}", variable.name, variable.data_type.to_string());
                flow_type_builder.variable(variable.name, variable.data_type.into());
            }
        }
        if extensions.is_some() {
            for extension in extensions.unwrap() {
                debug!("{} {}", extension.name, extension.extension.to_string());
                flow_type_builder.extension(extension.name, extension.extension.clone());
            }
        }

        let flow_type = flow_type_builder.build();
        flow_type_manager.register(flow_type.clone());
        Ok(flow_type.into())
    }

    /// Deletes the flow type with the given name.
    async fn delete(&self, context: &Context<'_>, name: String) -> Result<bool> {
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager>>()?;
        flow_type_manager.delete(name.as_str());
        Ok(true)
    }
}
