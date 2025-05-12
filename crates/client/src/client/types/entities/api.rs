use std::sync::Arc;

use crate::client::ReactiveGraphClient;
use crate::client::ReactiveGraphClientExecutionError;
use crate::client::types::common::variables::type_id::variables::TypeIdVariables;
use crate::client::types::common::variables::update_description::variables::UpdateDescriptionVariables;
use crate::client::types::entities::mutations::add_extension::mutations::add_extension_mutation;
use crate::client::types::entities::mutations::add_extension::mutations::add_extension_with_variables;
use crate::client::types::entities::mutations::add_property::mutations::add_property_mutation;
use crate::client::types::entities::mutations::add_property::mutations::add_property_with_variables;
use crate::client::types::entities::mutations::create::mutations::create_entity_type_mutation;
use crate::client::types::entities::mutations::create::mutations::create_entity_type_with_variables;
use crate::client::types::entities::mutations::delete::mutations::delete_entity_type_mutation;
use crate::client::types::entities::mutations::delete::mutations::delete_entity_type_with_variables;
use crate::client::types::entities::mutations::remove_extension::mutations::remove_extension_mutation;
use crate::client::types::entities::mutations::remove_extension::mutations::remove_extension_with_variables;
use crate::client::types::entities::mutations::remove_property::mutations::remove_property_mutation;
use crate::client::types::entities::mutations::remove_property::mutations::remove_property_with_variables;
use crate::client::types::entities::mutations::update_description::mutations::update_description_mutation;
use crate::client::types::entities::mutations::update_description::mutations::update_description_with_variables;
use crate::client::types::entities::queries::get_all::queries::get_all_entity_types_query;
use crate::client::types::entities::queries::get_by_type::queries::get_entity_type_by_type_query;
use crate::client::types::entities::variables::create::variables::CreateEntityTypeVariables;
use crate::client::types::extensions::variables::add_extension::variables::AddExtensionVariables;
use crate::client::types::extensions::variables::container::variables::ExtensionContainerVariables;
use crate::client::types::properties::variables::add_property::variables::AddPropertyVariables;
use crate::schema_graphql::types::component::Components as ComponentsVec;
use crate::schema_graphql::types::entity_type::EntityTypes as EntityTypesVec;
use crate::types::components::variables::container::variables::ComponentContainerVariables;
use crate::types::entities::mutations::add_component::mutations::add_component_mutation;
use crate::types::entities::mutations::add_component::mutations::add_component_with_variables;
use crate::types::entities::mutations::remove_component::mutations::remove_component_mutation;
use crate::types::entities::mutations::remove_component::mutations::remove_component_with_variables;
use crate::types::entities::queries::get_components::queries::get_entity_type_components_query;
use crate::types::properties::variables::container::variables::PropertyContainerVariables;
use cynic::http::ReqwestExt;
use reactive_graph_graph::Component;
use reactive_graph_graph::EntityComponentTypeId;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::ExtensionTypeId;

pub struct EntityTypes {
    client: Arc<ReactiveGraphClient>,
}

impl EntityTypes {
    pub fn new(client: Arc<ReactiveGraphClient>) -> Self {
        Self { client }
    }

    pub async fn get_all_entity_types(&self) -> Result<Option<Vec<EntityType>>, ReactiveGraphClientExecutionError> {
        let entity_types = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(get_all_entity_types_query())
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| EntityTypesVec(data.types.entities))
            .map(From::from);
        Ok(entity_types)
    }

    pub async fn get_entity_type_by_type<C: Into<EntityTypeId>>(&self, ty: C) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let ty = ty.into();
        let entity_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(get_entity_type_by_type_query(&ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| data.types.entities.first().cloned())
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn get_entity_type_components<C: Into<EntityTypeId>>(&self, ty: C) -> Result<Option<Vec<Component>>, ReactiveGraphClientExecutionError> {
        let ty = ty.into();
        let components = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(get_entity_type_components_query(&ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| data.types.entities.first().cloned().map(|entity_type| ComponentsVec(entity_type.components)))
            .map(From::from);
        Ok(components)
    }

    pub async fn create_entity_type(&self, entity_type: EntityType) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(create_entity_type_mutation(entity_type))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.create)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn create_entity_type_with_variables(
        &self,
        variables: CreateEntityTypeVariables,
    ) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(create_entity_type_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.create)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn delete_entity_type<C: Into<EntityTypeId>>(&self, ty: C) -> Result<Option<bool>, ReactiveGraphClientExecutionError> {
        let ty: EntityTypeId = ty.into();
        let entity_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(delete_entity_type_mutation(ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.delete);
        Ok(entity_type)
    }

    pub async fn delete_entity_type_with_variables(&self, variables: TypeIdVariables) -> Result<Option<bool>, ReactiveGraphClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(delete_entity_type_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.delete);
        Ok(entity_type)
    }

    pub async fn add_property(
        &self,
        ty: EntityTypeId,
        property_type: reactive_graph_graph::PropertyType,
    ) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(add_property_mutation(ty, property_type))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.add_property)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn add_property_with_variables(&self, variables: AddPropertyVariables) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(add_property_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.add_property)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn remove_property(&self, ty: EntityTypeId, property_name: String) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(remove_property_mutation(ty, property_name))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.remove_property)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn remove_property_with_variables(&self, variables: PropertyContainerVariables) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(remove_property_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.remove_property)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn add_extension(
        &self,
        ty: EntityTypeId,
        extension: reactive_graph_graph::Extension,
    ) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(add_extension_mutation(ty, extension))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.add_extension)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn add_extension_with_variables(&self, variables: AddExtensionVariables) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(add_extension_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.add_extension)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn remove_extension(&self, ty: EntityTypeId, extension_ty: ExtensionTypeId) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(remove_extension_mutation(ty, extension_ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.remove_extension)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn remove_extension_with_variables(
        &self,
        variables: ExtensionContainerVariables,
    ) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(remove_extension_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.remove_extension)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn add_component(&self, ty: EntityComponentTypeId) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(add_component_mutation(ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.add_component)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn add_component_with_variables(&self, variables: ComponentContainerVariables) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(add_component_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.add_component)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn remove_component(&self, ty: EntityComponentTypeId) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(remove_component_mutation(ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.remove_component)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn remove_component_with_variables(
        &self,
        variables: ComponentContainerVariables,
    ) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(remove_component_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.remove_component)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn update_description(&self, ty: EntityTypeId, description: String) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(update_description_mutation(ty, description))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.update_description)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn update_description_with_variables(
        &self,
        variables: UpdateDescriptionVariables,
    ) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(update_description_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.update_description)
            .map(From::from);
        Ok(component)
    }
}
