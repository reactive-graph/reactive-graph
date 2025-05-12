use std::sync::Arc;

use crate::client::ReactiveGraphClient;
use crate::client::ReactiveGraphClientExecutionError;
use crate::client::types::common::variables::type_id::variables::TypeIdVariables;
use crate::client::types::common::variables::update_description::variables::UpdateDescriptionVariables;
use crate::client::types::components::variables::container::variables::ComponentContainerVariables;
use crate::client::types::extensions::variables::add_extension::variables::AddExtensionVariables;
use crate::client::types::extensions::variables::container::variables::ExtensionContainerVariables;
use crate::client::types::properties::variables::add_property::variables::AddPropertyVariables;
use crate::client::types::properties::variables::container::variables::PropertyContainerVariables;
use crate::client::types::relations::mutations::add_component::mutations::add_component_mutation;
use crate::client::types::relations::mutations::add_component::mutations::add_component_with_variables;
use crate::client::types::relations::mutations::add_extension::mutations::add_extension_mutation;
use crate::client::types::relations::mutations::add_extension::mutations::add_extension_with_variables;
use crate::client::types::relations::mutations::add_property::mutations::add_property_mutation;
use crate::client::types::relations::mutations::add_property::mutations::add_property_with_variables;
use crate::client::types::relations::mutations::create::mutations::CreateRelationTypeVariables;
use crate::client::types::relations::mutations::create::mutations::create_relation_type_mutation;
use crate::client::types::relations::mutations::create::mutations::create_relation_type_with_variables;
use crate::client::types::relations::mutations::delete::mutations::delete_relation_type_mutation;
use crate::client::types::relations::mutations::delete::mutations::delete_relation_type_with_variables;
use crate::client::types::relations::mutations::remove_component::mutations::remove_component_mutation;
use crate::client::types::relations::mutations::remove_component::mutations::remove_component_with_variables;
use crate::client::types::relations::mutations::remove_extension::mutations::remove_extension_mutation;
use crate::client::types::relations::mutations::remove_extension::mutations::remove_extension_with_variables;
use crate::client::types::relations::mutations::remove_property::mutations::remove_property_mutation;
use crate::client::types::relations::mutations::remove_property::mutations::remove_property_with_variables;
use crate::client::types::relations::mutations::update_description::mutations::update_description_mutation;
use crate::client::types::relations::mutations::update_description::mutations::update_description_with_variables;
use crate::client::types::relations::queries::get_all::queries::get_all_relation_types_query;
use crate::client::types::relations::queries::get_by_type::queries::get_relation_type_by_type_query;
use crate::client::types::relations::queries::get_components::queries::get_relation_type_components_query;
use crate::schema_graphql::types::component::Components as ComponentsVec;
use crate::schema_graphql::types::relation_type::RelationTypes as RelationTypesVec;
use cynic::http::ReqwestExt;
use reactive_graph_graph::Component;
use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::RelationComponentTypeId;
use reactive_graph_graph::RelationType;
use reactive_graph_graph::RelationTypeId;

pub struct RelationTypes {
    client: Arc<ReactiveGraphClient>,
}

impl RelationTypes {
    pub fn new(client: Arc<ReactiveGraphClient>) -> Self {
        Self { client }
    }

    pub async fn get_all_relation_types(&self) -> Result<Option<Vec<RelationType>>, ReactiveGraphClientExecutionError> {
        let relation_types = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(get_all_relation_types_query())
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| RelationTypesVec(data.types.relations))
            .map(From::from);
        Ok(relation_types)
    }

    pub async fn get_relation_type_by_type<C: Into<RelationTypeId>>(&self, ty: C) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let ty = ty.into();
        let relation_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(get_relation_type_by_type_query(&ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| data.types.relations.first().cloned())
            .map(From::from);
        Ok(relation_type)
    }

    pub async fn get_relation_type_components<C: Into<RelationTypeId>>(&self, ty: C) -> Result<Option<Vec<Component>>, ReactiveGraphClientExecutionError> {
        let ty = ty.into();
        let components = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(get_relation_type_components_query(&ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| {
                data.types
                    .relations
                    .first()
                    .cloned()
                    .map(|relation_type| ComponentsVec(relation_type.components))
            })
            .map(From::from);
        Ok(components)
    }

    pub async fn create_relation_type(&self, relation_type: RelationType) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let relation_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(create_relation_type_mutation(relation_type))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.create)
            .map(From::from);
        Ok(relation_type)
    }

    pub async fn create_relation_type_with_variables(
        &self,
        variables: CreateRelationTypeVariables,
    ) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let relation_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(create_relation_type_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.create)
            .map(From::from);
        Ok(relation_type)
    }

    pub async fn delete_relation_type<C: Into<RelationTypeId>>(&self, ty: C) -> Result<Option<bool>, ReactiveGraphClientExecutionError> {
        let ty: RelationTypeId = ty.into();
        let relation_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(delete_relation_type_mutation(ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.delete);
        Ok(relation_type)
    }

    pub async fn delete_relation_type_with_variables(&self, variables: TypeIdVariables) -> Result<Option<bool>, ReactiveGraphClientExecutionError> {
        let relation_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(delete_relation_type_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.delete);
        Ok(relation_type)
    }

    pub async fn add_property(
        &self,
        ty: RelationTypeId,
        property_type: reactive_graph_graph::PropertyType,
    ) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let relation_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(add_property_mutation(ty, property_type))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.add_property)
            .map(From::from);
        Ok(relation_type)
    }

    pub async fn add_property_with_variables(&self, variables: AddPropertyVariables) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let relation_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(add_property_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.add_property)
            .map(From::from);
        Ok(relation_type)
    }

    pub async fn remove_property(&self, ty: RelationTypeId, property_name: String) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let relation_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(remove_property_mutation(ty, property_name))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.remove_property)
            .map(From::from);
        Ok(relation_type)
    }

    pub async fn remove_property_with_variables(
        &self,
        variables: PropertyContainerVariables,
    ) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let relation_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(remove_property_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.remove_property)
            .map(From::from);
        Ok(relation_type)
    }

    pub async fn add_extension(
        &self,
        ty: RelationTypeId,
        extension: reactive_graph_graph::Extension,
    ) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let relation_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(add_extension_mutation(ty, extension))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.add_extension)
            .map(From::from);
        Ok(relation_type)
    }

    pub async fn add_extension_with_variables(&self, variables: AddExtensionVariables) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let relation_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(add_extension_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.add_extension)
            .map(From::from);
        Ok(relation_type)
    }

    pub async fn remove_extension(&self, ty: RelationTypeId, extension_ty: ExtensionTypeId) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let relation_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(remove_extension_mutation(ty, extension_ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.remove_extension)
            .map(From::from);
        Ok(relation_type)
    }

    pub async fn remove_extension_with_variables(
        &self,
        variables: ExtensionContainerVariables,
    ) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let relation_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(remove_extension_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.remove_extension)
            .map(From::from);
        Ok(relation_type)
    }

    pub async fn add_component(&self, ty: RelationComponentTypeId) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let relation_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(add_component_mutation(ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.add_component)
            .map(From::from);
        Ok(relation_type)
    }

    pub async fn add_component_with_variables(
        &self,
        variables: ComponentContainerVariables,
    ) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let relation_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(add_component_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.add_component)
            .map(From::from);
        Ok(relation_type)
    }

    pub async fn remove_component(&self, ty: RelationComponentTypeId) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let relation_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(remove_component_mutation(ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.remove_component)
            .map(From::from);
        Ok(relation_type)
    }

    pub async fn remove_component_with_variables(
        &self,
        variables: ComponentContainerVariables,
    ) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let relation_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(remove_component_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.remove_component)
            .map(From::from);
        Ok(relation_type)
    }

    pub async fn update_description(&self, ty: RelationTypeId, description: String) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let relation_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(update_description_mutation(ty, description))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.update_description)
            .map(From::from);
        Ok(relation_type)
    }

    pub async fn update_description_with_variables(
        &self,
        variables: UpdateDescriptionVariables,
    ) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(update_description_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.update_description)
            .map(From::from);
        Ok(component)
    }
}
