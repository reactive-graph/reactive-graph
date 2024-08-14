use std::sync::Arc;

use cynic::http::ReqwestExt;

use crate::client::types::entity_types::add_extension::queries::add_extension_mutation;
use crate::client::types::entity_types::add_extension::queries::add_extension_with_variables;
use crate::client::types::entity_types::add_extension::queries::AddExtensionVariables;
use crate::client::types::entity_types::add_property::queries::add_property_mutation;
use crate::client::types::entity_types::add_property::queries::add_property_with_variables;
use crate::client::types::entity_types::add_property::queries::AddPropertyVariables;
use crate::client::types::entity_types::create::queries::create_entity_type_mutation;
use crate::client::types::entity_types::create::queries::create_entity_type_with_variables;
use crate::client::types::entity_types::create::queries::CreateEntityTypeVariables;
use crate::client::types::entity_types::delete::queries::delete_entity_type_mutation;
use crate::client::types::entity_types::delete::queries::delete_entity_type_with_variables;
use crate::client::types::entity_types::get_all::queries::get_all_entity_types_query;
use crate::client::types::entity_types::get_by_type::queries::get_entity_type_by_type_query;
use crate::client::types::entity_types::remove_extension::queries::remove_extension_mutation;
use crate::client::types::entity_types::remove_extension::queries::remove_extension_with_variables;
use crate::client::types::entity_types::remove_extension::queries::RemoveExtensionVariables;
use crate::client::types::entity_types::remove_property::queries::remove_property_mutation;
use crate::client::types::entity_types::remove_property::queries::remove_property_with_variables;
use crate::client::types::entity_types::remove_property::queries::RemovePropertyVariables;
use crate::client::types::entity_types::type_id::queries::EntityTypeIdVariables;
use crate::client::types::entity_types::update_description::queries::update_description_mutation;
use crate::client::types::entity_types::update_description::queries::update_description_with_variables;
use crate::client::types::entity_types::update_description::queries::UpdateDescriptionVariables;
use crate::client::InexorRgfClient;
use crate::client::InexorRgfClientExecutionError;
use crate::schema_graphql::types::entity_type::EntityTypes as EntityTypesVec;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::ExtensionTypeId;

pub struct EntityTypes {
    client: Arc<InexorRgfClient>,
}

impl EntityTypes {
    pub fn new(client: Arc<InexorRgfClient>) -> Self {
        Self { client }
    }

    pub async fn get_all_entity_types(&self) -> Result<Option<Vec<EntityType>>, InexorRgfClientExecutionError> {
        let entity_types = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(get_all_entity_types_query())
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| EntityTypesVec(data.types.entities))
            .map(From::from);
        Ok(entity_types)
    }

    pub async fn get_entity_type_by_type<C: Into<EntityTypeId>>(&self, ty: C) -> Result<Option<EntityType>, InexorRgfClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(get_entity_type_by_type_query(&ty.into()))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| data.types.entities.first().cloned())
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn create_entity_type(&self, entity_type: EntityType) -> Result<Option<EntityType>, InexorRgfClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(create_entity_type_mutation(entity_type))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.create)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn create_entity_type_with_variables(&self, variables: CreateEntityTypeVariables) -> Result<Option<EntityType>, InexorRgfClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(create_entity_type_with_variables(variables))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.create)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn delete_entity_type<C: Into<EntityTypeId>>(&self, ty: C) -> Result<Option<bool>, InexorRgfClientExecutionError> {
        let ty: EntityTypeId = ty.into();
        let entity_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(delete_entity_type_mutation(ty))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.delete)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn delete_entity_type_with_variables(&self, variables: EntityTypeIdVariables) -> Result<Option<bool>, InexorRgfClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(delete_entity_type_with_variables(variables))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.delete)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn add_property(
        &self,
        ty: EntityTypeId,
        property_type: reactive_graph_graph::PropertyType,
    ) -> Result<Option<EntityType>, InexorRgfClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(add_property_mutation(ty, property_type))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.add_property)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn add_property_with_variables(&self, variables: AddPropertyVariables) -> Result<Option<EntityType>, InexorRgfClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(add_property_with_variables(variables))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.add_property)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn remove_property(&self, ty: EntityTypeId, property_name: String) -> Result<Option<EntityType>, InexorRgfClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(remove_property_mutation(ty, property_name))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.remove_property)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn remove_property_with_variables(&self, variables: RemovePropertyVariables) -> Result<Option<EntityType>, InexorRgfClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(remove_property_with_variables(variables))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.remove_property)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn add_extension(
        &self,
        ty: EntityTypeId,
        extension: reactive_graph_graph::Extension,
    ) -> Result<Option<EntityType>, InexorRgfClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(add_extension_mutation(ty, extension))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.add_extension)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn add_extension_with_variables(&self, variables: AddExtensionVariables) -> Result<Option<EntityType>, InexorRgfClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(add_extension_with_variables(variables))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.add_extension)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn remove_extension(&self, ty: EntityTypeId, extension_ty: ExtensionTypeId) -> Result<Option<EntityType>, InexorRgfClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(remove_extension_mutation(ty, extension_ty))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.remove_extension)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn remove_extension_with_variables(&self, variables: RemoveExtensionVariables) -> Result<Option<EntityType>, InexorRgfClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(remove_extension_with_variables(variables))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.remove_extension)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn update_description(&self, ty: EntityTypeId, description: String) -> Result<Option<EntityType>, InexorRgfClientExecutionError> {
        let entity_type = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(update_description_mutation(ty, description))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.update_description)
            .map(From::from);
        Ok(entity_type)
    }

    pub async fn update_description_with_variables(&self, variables: UpdateDescriptionVariables) -> Result<Option<EntityType>, InexorRgfClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(update_description_with_variables(variables))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.update_description)
            .map(From::from);
        Ok(component)
    }
}
