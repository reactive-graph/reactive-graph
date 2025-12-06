use crate::client::ReactiveGraphClient;
use crate::client::ReactiveGraphClientExecutionError;
use crate::client::types::components::mutations::add_extension::mutations::add_extension_mutation;
use crate::client::types::components::mutations::add_property::mutations::add_property_mutation;
use crate::client::types::components::mutations::create::mutations::create_component_mutation;
use crate::client::types::components::mutations::delete::mutations::delete_component_mutation;
use crate::client::types::components::mutations::remove_extension::mutations::remove_extension_mutation;
use crate::client::types::components::mutations::remove_property::mutations::remove_property_mutation;
use crate::client::types::components::mutations::update_description::mutations::update_description_mutation;
use crate::client::types::components::queries::get_all::queries::get_all_components_query;
use crate::client::types::components::queries::get_by_type::queries::get_component_by_type_query;
use crate::schema_graphql::types::component::Components as ComponentsVec;
use cynic::http::ReqwestExt;
use reactive_graph_graph::Component;
use reactive_graph_graph::ComponentExtensionTypeId;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::Extension;
use reactive_graph_graph::PropertyType;
use serde_json::Value;
use std::sync::Arc;

pub struct Components {
    client: Arc<ReactiveGraphClient>,
}

impl Components {
    pub fn new(client: Arc<ReactiveGraphClient>) -> Self {
        Self { client }
    }

    pub async fn get_all_components(&self) -> Result<Option<reactive_graph_graph::Components>, ReactiveGraphClientExecutionError> {
        let Some(components) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(get_all_components_query())
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| ComponentsVec(data.types.components))
        else {
            return Ok(None);
        };
        Ok(Some(
            reactive_graph_graph::Components::try_from(components).map_err(ReactiveGraphClientExecutionError::InvalidComponent)?,
        ))
    }

    pub async fn get_component_by_type<C: Into<ComponentTypeId>>(&self, ty: C) -> Result<Option<Component>, ReactiveGraphClientExecutionError> {
        let Some(component) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(get_component_by_type_query(ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| data.types.components.first().cloned())
        else {
            return Ok(None);
        };
        Ok(Some(Component::try_from(component).map_err(ReactiveGraphClientExecutionError::InvalidComponent)?))
    }

    pub async fn json_schema_for_component_by_type<C: Into<ComponentTypeId>>(&self, ty: C) -> Result<Option<Value>, ReactiveGraphClientExecutionError> {
        let json_schema = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(get_component_by_type_query(ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| data.types.components.first().cloned())
            .map(|component| component.json_schema);
        Ok(json_schema)
    }

    pub async fn create_component<C: Into<Component>>(&self, component: C) -> Result<Option<Component>, ReactiveGraphClientExecutionError> {
        let Some(component) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(create_component_mutation(component))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.create)
        else {
            return Ok(None);
        };
        Ok(Some(Component::try_from(component).map_err(ReactiveGraphClientExecutionError::InvalidComponent)?))
    }

    pub async fn delete_component<C: Into<ComponentTypeId>>(&self, ty: C) -> Result<Option<bool>, ReactiveGraphClientExecutionError> {
        let ty: ComponentTypeId = ty.into();
        let success = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(delete_component_mutation(ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.delete);
        Ok(success)
    }

    pub async fn add_property<C: Into<ComponentTypeId>, PT: Into<PropertyType>>(
        &self,
        ty: C,
        property_type: PT,
    ) -> Result<Option<Component>, ReactiveGraphClientExecutionError> {
        let Some(component) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(add_property_mutation(ty, property_type))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.add_property)
        else {
            return Ok(None);
        };
        Ok(Some(Component::try_from(component).map_err(ReactiveGraphClientExecutionError::InvalidComponent)?))
    }

    pub async fn remove_property<C: Into<ComponentTypeId>>(
        &self,
        ty: C,
        property_name: String,
    ) -> Result<Option<Component>, ReactiveGraphClientExecutionError> {
        let Some(component) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(remove_property_mutation(ty, property_name))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.remove_property)
        else {
            return Ok(None);
        };
        Ok(Some(Component::try_from(component).map_err(ReactiveGraphClientExecutionError::InvalidComponent)?))
    }

    pub async fn add_extension<C: Into<ComponentTypeId>, EXT: Into<Extension>>(
        &self,
        ty: C,
        extension: EXT,
    ) -> Result<Option<Component>, ReactiveGraphClientExecutionError> {
        let Some(component) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(add_extension_mutation(ty, extension))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.add_extension)
        else {
            return Ok(None);
        };
        Ok(Some(Component::try_from(component).map_err(ReactiveGraphClientExecutionError::InvalidComponent)?))
    }

    pub async fn remove_extension<CE: Into<ComponentExtensionTypeId>>(
        &self,
        component_extension_ty: CE,
    ) -> Result<Option<Component>, ReactiveGraphClientExecutionError> {
        let Some(component) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(remove_extension_mutation(component_extension_ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.remove_extension)
        else {
            return Ok(None);
        };
        Ok(Some(Component::try_from(component).map_err(ReactiveGraphClientExecutionError::InvalidComponent)?))
    }

    pub async fn update_description<C: Into<ComponentTypeId>>(
        &self,
        ty: C,
        description: String,
    ) -> Result<Option<Component>, ReactiveGraphClientExecutionError> {
        let Some(component) = self
            .client
            .client
            .post(self.client.url_reactive_graph())
            .run_graphql(update_description_mutation(ty, description))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.update_description)
        else {
            return Ok(None);
        };
        Ok(Some(Component::try_from(component).map_err(ReactiveGraphClientExecutionError::InvalidComponent)?))
    }
}
