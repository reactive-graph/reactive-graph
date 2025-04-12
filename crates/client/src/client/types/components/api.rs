use std::sync::Arc;

use cynic::http::ReqwestExt;

use crate::client::ReactiveGraphClient;
use crate::client::ReactiveGraphClientExecutionError;
use crate::client::types::components::add_extension::queries::AddExtensionVariables;
use crate::client::types::components::add_extension::queries::add_extension_mutation;
use crate::client::types::components::add_extension::queries::add_extension_with_variables;
use crate::client::types::components::add_property::queries::AddPropertyVariables;
use crate::client::types::components::add_property::queries::add_property_mutation;
use crate::client::types::components::add_property::queries::add_property_with_variables;
use crate::client::types::components::create::queries::CreateComponentVariables;
use crate::client::types::components::create::queries::create_component_mutation;
use crate::client::types::components::create::queries::create_component_with_variables;
use crate::client::types::components::delete::queries::delete_component_mutation;
use crate::client::types::components::delete::queries::delete_component_with_variables;
use crate::client::types::components::get_all::queries::get_all_components_query;
use crate::client::types::components::get_by_type::queries::get_component_by_type_query;
use crate::client::types::components::remove_extension::queries::remove_extension_mutation;
use crate::client::types::components::remove_extension::queries::remove_extension_with_variables;
use crate::client::types::components::remove_property::queries::remove_property_mutation;
use crate::client::types::components::remove_property::queries::remove_property_with_variables;
use crate::client::types::components::type_id::queries::ComponentTypeIdVariables;
use crate::client::types::components::update_description::queries::UpdateDescriptionVariables;
use crate::client::types::components::update_description::queries::update_description_mutation;
use crate::client::types::components::update_description::queries::update_description_with_variables;
use crate::client::types::extensions::container::queries::ExtensionContainerVariables;
use crate::client::types::properties::container::queries::PropertyContainerVariables;
use crate::schema_graphql::types::component::Components as ComponentsVec;
use reactive_graph_graph::Component;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::ExtensionTypeId;

pub struct Components {
    client: Arc<ReactiveGraphClient>,
}

impl Components {
    pub fn new(client: Arc<ReactiveGraphClient>) -> Self {
        Self { client }
    }

    pub async fn get_all_components(&self) -> Result<Option<Vec<Component>>, ReactiveGraphClientExecutionError> {
        let components = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(get_all_components_query())
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| ComponentsVec(data.types.components))
            .map(From::from);
        Ok(components)
    }

    pub async fn get_component_by_type<C: Into<ComponentTypeId>>(&self, ty: C) -> Result<Option<Component>, ReactiveGraphClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(get_component_by_type_query(&ty.into()))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| data.types.components.first().cloned())
            .map(From::from);
        Ok(component)
    }

    pub async fn create_component<C: Into<Component>>(&self, component: C) -> Result<Option<Component>, ReactiveGraphClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(create_component_mutation(component.into()))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.create)
            .map(From::from);
        Ok(component)
    }

    pub async fn create_component_with_variables(&self, variables: CreateComponentVariables) -> Result<Option<Component>, ReactiveGraphClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(create_component_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.create)
            .map(From::from);
        Ok(component)
    }

    pub async fn delete_component<C: Into<ComponentTypeId>>(&self, ty: C) -> Result<Option<bool>, ReactiveGraphClientExecutionError> {
        let ty: ComponentTypeId = ty.into();
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(delete_component_mutation(ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.delete);
        Ok(component)
    }

    pub async fn delete_component_with_variables(&self, variables: ComponentTypeIdVariables) -> Result<Option<bool>, ReactiveGraphClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(delete_component_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.delete);
        Ok(component)
    }

    pub async fn add_property(
        &self,
        ty: ComponentTypeId,
        property_type: reactive_graph_graph::PropertyType,
    ) -> Result<Option<Component>, ReactiveGraphClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(add_property_mutation(ty, property_type))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.add_property)
            .map(From::from);
        Ok(component)
    }

    pub async fn add_property_with_variables(&self, variables: AddPropertyVariables) -> Result<Option<Component>, ReactiveGraphClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(add_property_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.add_property)
            .map(From::from);
        Ok(component)
    }

    pub async fn remove_property(&self, ty: ComponentTypeId, property_name: String) -> Result<Option<Component>, ReactiveGraphClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(remove_property_mutation(ty, property_name))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.remove_property)
            .map(From::from);
        Ok(component)
    }

    pub async fn remove_property_with_variables(&self, variables: PropertyContainerVariables) -> Result<Option<Component>, ReactiveGraphClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(remove_property_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.remove_property)
            .map(From::from);
        Ok(component)
    }

    pub async fn add_extension(
        &self,
        ty: ComponentTypeId,
        extension: reactive_graph_graph::Extension,
    ) -> Result<Option<Component>, ReactiveGraphClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(add_extension_mutation(ty, extension))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.add_extension)
            .map(From::from);
        Ok(component)
    }

    pub async fn add_extension_with_variables(&self, variables: AddExtensionVariables) -> Result<Option<Component>, ReactiveGraphClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(add_extension_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.add_extension)
            .map(From::from);
        Ok(component)
    }

    pub async fn remove_extension(&self, ty: ComponentTypeId, extension_ty: ExtensionTypeId) -> Result<Option<Component>, ReactiveGraphClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(remove_extension_mutation(ty, extension_ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.remove_extension)
            .map(From::from);
        Ok(component)
    }

    pub async fn remove_extension_with_variables(
        &self,
        variables: ExtensionContainerVariables,
    ) -> Result<Option<Component>, ReactiveGraphClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(remove_extension_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.remove_extension)
            .map(From::from);
        Ok(component)
    }

    pub async fn update_description(&self, ty: ComponentTypeId, description: String) -> Result<Option<Component>, ReactiveGraphClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(update_description_mutation(ty, description))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.update_description)
            .map(From::from);
        Ok(component)
    }

    pub async fn update_description_with_variables(
        &self,
        variables: UpdateDescriptionVariables,
    ) -> Result<Option<Component>, ReactiveGraphClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(update_description_with_variables(variables))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.update_description)
            .map(From::from);
        Ok(component)
    }
}
