use std::sync::Arc;

use crate::client::ReactiveGraphClient;
use crate::client::ReactiveGraphClientExecutionError;
use crate::client::types::entities::mutations::add_extension::mutations::add_extension_mutation;
use crate::client::types::entities::mutations::add_property::mutations::add_property_mutation;
use crate::client::types::entities::mutations::create::mutations::create_entity_type_mutation;
use crate::client::types::entities::mutations::delete::mutations::delete_entity_type_mutation;
use crate::client::types::entities::mutations::remove_extension::mutations::remove_extension_mutation;
use crate::client::types::entities::mutations::remove_property::mutations::remove_property_mutation;
use crate::client::types::entities::mutations::update_description::mutations::update_description_mutation;
use crate::client::types::entities::queries::get_all::queries::get_all_entity_types_query;
use crate::client::types::entities::queries::get_by_type::queries::get_entity_type_by_type_query;
use crate::schema_graphql::types::component::Components as ComponentsVec;
use crate::schema_graphql::types::entity_type::EntityTypes as EntityTypesVec;
use crate::types::entities::mutations::add_component::mutations::add_component_mutation;
use crate::types::entities::mutations::remove_component::mutations::remove_component_mutation;
use crate::types::entities::queries::get_components::queries::get_entity_type_components_query;
use cynic::http::ReqwestExt;
use reactive_graph_graph::EntityComponentTypeId;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::Extension;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::entity_extension_type_id::EntityExtensionTypeId;
use serde_json::Value;

pub struct EntityTypes {
    client: Arc<ReactiveGraphClient>,
}

impl EntityTypes {
    pub fn new(client: Arc<ReactiveGraphClient>) -> Self {
        Self { client }
    }

    pub async fn get_all_entity_types(&self) -> Result<Option<reactive_graph_graph::EntityTypes>, ReactiveGraphClientExecutionError> {
        let Some(entity_types) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(get_all_entity_types_query())
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| EntityTypesVec(data.types.entities))
        else {
            return Ok(None);
        };
        Ok(Some(
            reactive_graph_graph::EntityTypes::try_from(entity_types).map_err(|e| ReactiveGraphClientExecutionError::InvalidEntityType(e))?,
        ))
    }

    pub async fn get_entity_type_by_type<E: Into<EntityTypeId>>(&self, ty: E) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let Some(entity_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(get_entity_type_by_type_query(ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| data.types.entities.first().cloned())
        else {
            return Ok(None);
        };
        Ok(Some(EntityType::try_from(entity_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidEntityType(e))?))
    }

    pub async fn json_schema_for_entity_type_by_type<E: Into<EntityTypeId>>(&self, ty: E) -> Result<Option<Value>, ReactiveGraphClientExecutionError> {
        let ty = ty.into();
        let json_schema = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(get_entity_type_by_type_query(ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| data.types.entities.first().cloned())
            .map(|entity_type| entity_type.json_schema);
        Ok(json_schema)
    }

    pub async fn get_entity_type_components<E: Into<EntityTypeId>>(
        &self,
        ty: E,
    ) -> Result<Option<reactive_graph_graph::Components>, ReactiveGraphClientExecutionError> {
        let Some(components) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(get_entity_type_components_query(ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| data.types.entities.first().cloned().map(|entity_type| ComponentsVec(entity_type.components)))
        else {
            return Ok(None);
        };
        Ok(Some(
            reactive_graph_graph::Components::try_from(components).map_err(|e| ReactiveGraphClientExecutionError::InvalidComponent(e))?,
        ))
    }

    pub async fn create_entity_type(&self, entity_type: EntityType) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let Some(entity_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(create_entity_type_mutation(entity_type))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.create)
        else {
            return Ok(None);
        };
        Ok(Some(EntityType::try_from(entity_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidEntityType(e))?))
    }

    pub async fn delete_entity_type<E: Into<EntityTypeId>>(&self, ty: E) -> Result<Option<bool>, ReactiveGraphClientExecutionError> {
        let ty: EntityTypeId = ty.into();
        let success = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(delete_entity_type_mutation(ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.delete);
        Ok(success)
    }

    pub async fn add_property<E: Into<EntityTypeId>, PT: Into<PropertyType>>(
        &self,
        ty: E,
        property_type: PT,
    ) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let Some(entity_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(add_property_mutation(ty, property_type))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.add_property)
        else {
            return Ok(None);
        };
        Ok(Some(EntityType::try_from(entity_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidEntityType(e))?))
    }

    pub async fn remove_property<E: Into<EntityTypeId>>(&self, ty: E, property_name: String) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let Some(entity_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(remove_property_mutation(ty, property_name))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.remove_property)
        else {
            return Ok(None);
        };
        Ok(Some(EntityType::try_from(entity_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidEntityType(e))?))
    }

    pub async fn add_extension<E: Into<EntityTypeId>, EXT: Into<Extension>>(
        &self,
        ty: E,
        extension: EXT,
    ) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let Some(entity_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(add_extension_mutation(ty, extension))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.add_extension)
        else {
            return Ok(None);
        };
        Ok(Some(EntityType::try_from(entity_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidEntityType(e))?))
    }

    pub async fn remove_extension<EE: Into<EntityExtensionTypeId>>(
        &self,
        entity_extension_ty: EE,
    ) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let Some(entity_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(remove_extension_mutation(entity_extension_ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.remove_extension)
        else {
            return Ok(None);
        };
        Ok(Some(EntityType::try_from(entity_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidEntityType(e))?))
    }

    pub async fn add_component<EC: Into<EntityComponentTypeId>>(&self, ty: EC) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let Some(entity_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(add_component_mutation(ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.add_component)
        else {
            return Ok(None);
        };
        Ok(Some(EntityType::try_from(entity_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidEntityType(e))?))
    }

    pub async fn remove_component<EC: Into<EntityComponentTypeId>>(&self, ty: EC) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let Some(entity_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(remove_component_mutation(ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.remove_component)
        else {
            return Ok(None);
        };
        Ok(Some(EntityType::try_from(entity_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidEntityType(e))?))
    }

    pub async fn update_description<E: Into<EntityTypeId>>(&self, ty: E, description: String) -> Result<Option<EntityType>, ReactiveGraphClientExecutionError> {
        let Some(entity_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(update_description_mutation(ty, description))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.entities.update_description)
        else {
            return Ok(None);
        };
        Ok(Some(EntityType::try_from(entity_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidEntityType(e))?))
    }
}
