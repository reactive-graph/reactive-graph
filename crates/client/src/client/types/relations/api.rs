use std::sync::Arc;

use crate::client::ReactiveGraphClient;
use crate::client::ReactiveGraphClientExecutionError;
use crate::client::types::relations::mutations::add_component::mutations::add_component_mutation;
use crate::client::types::relations::mutations::add_extension::mutations::add_extension_mutation;
use crate::client::types::relations::mutations::add_property::mutations::add_property_mutation;
use crate::client::types::relations::mutations::create::mutations::create_relation_type_mutation;
use crate::client::types::relations::mutations::delete::mutations::delete_relation_type_mutation;
use crate::client::types::relations::mutations::remove_component::mutations::remove_component_mutation;
use crate::client::types::relations::mutations::remove_extension::mutations::remove_extension_mutation;
use crate::client::types::relations::mutations::remove_property::mutations::remove_property_mutation;
use crate::client::types::relations::mutations::update_description::mutations::update_description_mutation;
use crate::client::types::relations::queries::get_all::queries::get_all_relation_types_query;
use crate::client::types::relations::queries::get_by_type::queries::get_relation_type_by_type_query;
use crate::client::types::relations::queries::get_components::queries::get_relation_type_components_query;
use crate::schema_graphql::types::component::Components as ComponentsVec;
use crate::schema_graphql::types::relation_type::RelationTypes as RelationTypesVec;
use cynic::http::ReqwestExt;
use reactive_graph_graph::Extension;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::RelationComponentTypeId;
use reactive_graph_graph::RelationExtensionTypeId;
use reactive_graph_graph::RelationType;
use reactive_graph_graph::RelationTypeId;
use serde_json::Value;

pub struct RelationTypes {
    client: Arc<ReactiveGraphClient>,
}

impl RelationTypes {
    pub fn new(client: Arc<ReactiveGraphClient>) -> Self {
        Self { client }
    }

    pub async fn get_all_relation_types(&self) -> Result<Option<reactive_graph_graph::RelationTypes>, ReactiveGraphClientExecutionError> {
        let Some(relation_types) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(get_all_relation_types_query())
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| RelationTypesVec(data.types.relations))
        else {
            return Ok(None);
        };
        Ok(Some(
            reactive_graph_graph::RelationTypes::try_from(relation_types).map_err(|e| ReactiveGraphClientExecutionError::InvalidRelationType(e))?,
        ))
    }

    pub async fn get_relation_type_by_type<R: Into<RelationTypeId>>(&self, ty: R) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let ty = ty.into();
        let Some(relation_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(get_relation_type_by_type_query(&ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| data.types.relations.first().cloned())
        else {
            return Ok(None);
        };
        Ok(Some(
            RelationType::try_from(relation_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidRelationType(e))?,
        ))
    }

    pub async fn json_schema_for_relation_type_by_type<R: Into<RelationTypeId>>(&self, ty: R) -> Result<Option<Value>, ReactiveGraphClientExecutionError> {
        let ty = ty.into();
        let json_schema = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(get_relation_type_by_type_query(&ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| data.types.relations.first().cloned())
            .map(|relation_type| relation_type.json_schema);
        Ok(json_schema)
    }

    pub async fn get_relation_type_components<R: Into<RelationTypeId>>(
        &self,
        ty: R,
    ) -> Result<Option<reactive_graph_graph::Components>, ReactiveGraphClientExecutionError> {
        let ty = ty.into();
        let Some(components) = self
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
        else {
            return Ok(None);
        };
        Ok(Some(
            reactive_graph_graph::Components::try_from(components).map_err(|e| ReactiveGraphClientExecutionError::InvalidComponent(e))?,
        ))
    }

    pub async fn create_relation_type(&self, relation_type: RelationType) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let Some(relation_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(create_relation_type_mutation(relation_type))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.create)
        else {
            return Ok(None);
        };
        Ok(Some(
            RelationType::try_from(relation_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidRelationType(e))?,
        ))
    }

    pub async fn delete_relation_type<R: Into<RelationTypeId>>(&self, ty: R) -> Result<Option<bool>, ReactiveGraphClientExecutionError> {
        let ty: RelationTypeId = ty.into();
        let success = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(delete_relation_type_mutation(ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.delete);
        Ok(success)
    }

    pub async fn add_property<R: Into<RelationTypeId>, PT: Into<PropertyType>>(
        &self,
        ty: R,
        property_type: PT,
    ) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let Some(relation_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(add_property_mutation(ty, property_type))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.add_property)
        else {
            return Ok(None);
        };
        Ok(Some(
            RelationType::try_from(relation_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidRelationType(e))?,
        ))
    }

    pub async fn remove_property<R: Into<RelationTypeId>>(
        &self,
        ty: R,
        property_name: String,
    ) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let Some(relation_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(remove_property_mutation(ty, property_name))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.remove_property)
        else {
            return Ok(None);
        };
        Ok(Some(
            RelationType::try_from(relation_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidRelationType(e))?,
        ))
    }

    pub async fn add_extension<R: Into<RelationTypeId>, EXT: Into<Extension>>(
        &self,
        ty: R,
        extension: EXT,
    ) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let Some(relation_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(add_extension_mutation(ty, extension))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.add_extension)
        else {
            return Ok(None);
        };
        Ok(Some(
            RelationType::try_from(relation_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidRelationType(e))?,
        ))
    }

    pub async fn remove_extension<RE: Into<RelationExtensionTypeId>>(
        &self,
        relation_extension_ty: RE,
    ) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let Some(relation_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(remove_extension_mutation(relation_extension_ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.remove_extension)
        else {
            return Ok(None);
        };
        Ok(Some(
            RelationType::try_from(relation_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidRelationType(e))?,
        ))
    }

    pub async fn add_component<RC: Into<RelationComponentTypeId>>(&self, ty: RC) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let Some(relation_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(add_component_mutation(ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.add_component)
        else {
            return Ok(None);
        };
        Ok(Some(
            RelationType::try_from(relation_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidRelationType(e))?,
        ))
    }

    pub async fn remove_component<RC: Into<RelationComponentTypeId>>(&self, ty: RC) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let Some(relation_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(remove_component_mutation(ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.remove_component)
        else {
            return Ok(None);
        };
        Ok(Some(
            RelationType::try_from(relation_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidRelationType(e))?,
        ))
    }

    pub async fn update_description<R: Into<RelationTypeId>>(
        &self,
        ty: R,
        description: String,
    ) -> Result<Option<RelationType>, ReactiveGraphClientExecutionError> {
        let Some(relation_type) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(update_description_mutation(ty, description))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.relations.update_description)
        else {
            return Ok(None);
        };
        Ok(Some(
            RelationType::try_from(relation_type).map_err(|e| ReactiveGraphClientExecutionError::InvalidRelationType(e))?,
        ))
    }
}
