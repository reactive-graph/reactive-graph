use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Error;
use async_graphql::Object;
use async_graphql::Result;
use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::JsonSchemaIdGetter;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_model_impl::ReactiveRelation;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::FlowTypeManager;
use serde_json::Value;

use crate::query::GraphQLEntityInstance;
use crate::query::GraphQLEntityType;
use crate::query::GraphQLExtension;
use crate::query::GraphQLExtensions;
use crate::query::GraphQLNamespacedType;
use crate::query::GraphQLPropertyInstance;
use crate::query::GraphQLPropertyType;
use crate::query::GraphQLRelationInstance;
use crate::validator::NamespacedTypeValidator;

pub struct GraphQLFlowType {
    flow_type: FlowType,
}

/// Flow types are templates for flow instances.
#[Object(name = "FlowType")]
impl GraphQLFlowType {
    /// The wrapper entity type of the flow type.
    #[graphql(name = "type")]
    async fn entity_type(&self, context: &Context<'_>) -> Option<GraphQLEntityType> {
        if let Ok(entity_type_manager) = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>() {
            return entity_type_manager.get(&self.flow_type.wrapper_entity_instance.ty).map(|e| e.into());
        }
        None
    }

    /// The fully qualified namespace of the flow type.
    #[graphql(name = "type")]
    async fn ty(&self) -> String {
        self.flow_type.namespace().to_string()
    }

    /// The namespaced type.
    async fn namespaced_type(&self) -> GraphQLNamespacedType {
        self.flow_type.namespaced_type().into()
    }

    /// Textual description of the flow type.
    async fn description(&self) -> String {
        self.flow_type.description.clone()
    }

    /// The wrapper entity instance.
    async fn wrapper_entity_instance(&self) -> GraphQLEntityInstance {
        let entity_instance = self.flow_type.wrapper_entity_instance.clone();
        let reactive_entity = ReactiveEntity::from(entity_instance); // temporary, non-registered
        GraphQLEntityInstance::from(reactive_entity)
    }

    /// The entity instances contained by the flow type
    async fn entity_instances(&self) -> Vec<GraphQLEntityInstance> {
        self.flow_type
            .entity_instances
            .iter()
            .map(|entity_instance| {
                // Construct a temporary instance, do not register!
                let entity_instance: GraphQLEntityInstance = ReactiveEntity::from(entity_instance.clone()).into();
                entity_instance
            })
            .collect()
    }

    /// The count of entity instances.
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
                    ReactiveEntity::from(self.flow_type.wrapper_entity_instance.clone())
                } else {
                    self.flow_type
                        .entity_instances
                        .iter()
                        .find(|e| e.id == relation_instance.outbound_id)
                        .map(|e| ReactiveEntity::from(e.clone()))?
                };
                let inbound = if relation_instance.inbound_id == self.flow_type.wrapper_entity_instance.id {
                    ReactiveEntity::from(self.flow_type.wrapper_entity_instance.clone())
                } else {
                    self.flow_type
                        .entity_instances
                        .iter()
                        .find(|e| e.id == relation_instance.inbound_id)
                        .map(|e| ReactiveEntity::from(e.clone()))?
                };
                let relation_instance = ReactiveRelation::new_from_instance(outbound, inbound, relation_instance.clone());
                let relation_instance: GraphQLRelationInstance = relation_instance.into();
                Some(relation_instance)
            })
            .collect()
    }

    /// The count of relation instances.
    async fn count_relation_instances(&self) -> usize {
        self.flow_type.relation_instances.len()
    }

    /// The properties of the flow type.
    /// These are the properties of the wrapper entity instance.
    async fn properties(
        &self,
        #[graphql(desc = "Filters by property name")] name: Option<String>,
        #[graphql(desc = "Filters by property names")] names: Option<Vec<String>>,
    ) -> Vec<GraphQLPropertyInstance> {
        self.flow_type
            .wrapper_entity_instance
            .properties
            .iter()
            .filter(|property| name.is_none() || name.clone().unwrap().as_str() == property.key())
            .filter(|property| names.is_none() || names.clone().unwrap().contains(property.key()))
            .map(|property| {
                // TODO: Resolve mutability using entity_type_manager and property_name
                GraphQLPropertyInstance::new_entity_property(
                    self.flow_type.wrapper_entity_instance.ty.clone(),
                    property.key().clone(),
                    property.value().clone(),
                )
            })
            .collect()
    }

    /// The count of properties.
    async fn count_properties(&self) -> usize {
        self.flow_type.wrapper_entity_instance.properties.len()
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
        self.flow_type
            .variables
            .iter()
            .map(|property_type| property_type.value().clone().into())
            .collect()
    }

    /// The count of variables.
    async fn count_variables(&self) -> usize {
        self.flow_type.variables.len()
    }

    /// The extensions which are defined by the flow type.
    async fn extensions(
        &self,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the extension",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: Option<String>,
        #[graphql(desc = "If true, the extensions are sorted by type")] sort: Option<bool>,
    ) -> Result<Vec<GraphQLExtension>> {
        let ty = ExtensionTypeId::parse_optional_namespace(_type)?;
        let extensions: GraphQLExtensions = self
            .flow_type
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
        self.flow_type.extensions.len()
    }

    /// Returns true, if the relation type is valid.
    ///
    /// This means all components exists and the outbound and inbound entity types are valid.
    async fn is_valid(&self, context: &Context<'_>) -> bool {
        match context.data::<Arc<dyn FlowTypeManager + Send + Sync>>() {
            Ok(flow_type_manager) => flow_type_manager.validate(&self.flow_type.ty),
            Err(_) => false,
        }
    }

    /// Returns the JSON schema of the flow type.
    async fn json_schema(&self, context: &Context<'_>) -> Result<Value> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let entity_ty = self.flow_type.wrapper_type();
        let entity_type = entity_type_manager
            .get(&entity_ty)
            .ok_or(Error::new(format!("Missing wrapper entity type {entity_ty}")))?;
        match self.flow_type.json_schema(&entity_type) {
            Ok(schema) => Ok(schema.to_value()),
            Err(e) => Err(e.into()),
        }
    }

    /// Returns the JSON schema identifier ($id) of the flow type.
    async fn json_schema_id(&self) -> String {
        self.flow_type.json_schema_id().to_string()
    }
}

impl From<FlowType> for GraphQLFlowType {
    fn from(flow_type: FlowType) -> Self {
        GraphQLFlowType { flow_type }
    }
}
