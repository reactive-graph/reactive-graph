use std::sync::Arc;

use cynic::http::ReqwestExt;

use crate::client::types::components::add_property::queries::add_property_mutation;
use crate::client::types::components::add_property::queries::add_property_with_variables;
use crate::client::types::components::add_property::queries::AddPropertyVariables;
use crate::client::types::components::create_component::queries::create_component_mutation;
use crate::client::types::components::create_component::queries::create_component_with_variables;
use crate::client::types::components::create_component::queries::CreateComponentVariables;
use crate::client::types::components::delete_component::queries::delete_component_mutation;
use crate::client::types::components::delete_component::queries::delete_component_with_variables;
use crate::client::types::components::get_all_components::queries::get_all_components_query;
use crate::client::types::components::get_component_by_type::queries::get_component_by_type_query;
use crate::client::types::components::remove_property::queries::remove_property_mutation;
use crate::client::types::components::remove_property::queries::remove_property_with_variables;
use crate::client::InexorRgfClient;
use crate::client::InexorRgfClientExecutionError;
use crate::schema_graphql::types::component::Components as ComponentsVec;
use crate::types::components::common::queries::ComponentTypeIdVariables;
use crate::types::components::remove_property::queries::RemovePropertyVariables;
use reactive_graph_graph::Component;
use reactive_graph_graph::ComponentTypeId;

pub struct Components {
    client: Arc<InexorRgfClient>,
}

impl Components {
    pub fn new(client: Arc<InexorRgfClient>) -> Self {
        Self { client }
    }

    pub async fn get_all_components(&self) -> Result<Option<Vec<Component>>, InexorRgfClientExecutionError> {
        let components = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(get_all_components_query())
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| ComponentsVec(data.types.components))
            .map(From::from);
        Ok(components)
    }

    pub async fn get_component_by_type<C: Into<ComponentTypeId>>(&self, ty: C) -> Result<Option<Component>, InexorRgfClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(get_component_by_type_query(&ty.into()))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| data.types.components.first().cloned())
            .map(From::from);
        Ok(component)
    }

    pub async fn create_component<C: Into<Component>>(&self, component: C) -> Result<Option<Component>, InexorRgfClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(create_component_mutation(component.into()))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.create)
            .map(From::from);
        Ok(component)
    }

    pub async fn create_component_with_variables(&self, variables: CreateComponentVariables) -> Result<Option<Component>, InexorRgfClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(create_component_with_variables(variables))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.create)
            .map(From::from);
        Ok(component)
    }

    pub async fn delete_component<C: Into<ComponentTypeId>>(&self, ty: C) -> Result<Option<bool>, InexorRgfClientExecutionError> {
        let ty: ComponentTypeId = ty.into();
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(delete_component_mutation(ty))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.delete)
            .map(From::from);
        Ok(component)
    }

    pub async fn delete_component_with_variables(&self, variables: ComponentTypeIdVariables) -> Result<Option<bool>, InexorRgfClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(delete_component_with_variables(variables))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.delete)
            .map(From::from);
        Ok(component)
    }

    pub async fn add_property(
        &self,
        ty: ComponentTypeId,
        property_type: reactive_graph_graph::PropertyType,
    ) -> Result<Option<Component>, InexorRgfClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(add_property_mutation(ty, property_type))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.add_property)
            .map(From::from);
        Ok(component)
    }

    pub async fn add_property_with_variables(&self, variables: AddPropertyVariables) -> Result<Option<Component>, InexorRgfClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(add_property_with_variables(variables))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.add_property)
            .map(From::from);
        Ok(component)
    }

    pub async fn remove_property(&self, ty: ComponentTypeId, property_name: String) -> Result<Option<Component>, InexorRgfClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(remove_property_mutation(ty, property_name))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.remove_property)
            .map(From::from);
        Ok(component)
    }

    pub async fn remove_property_with_variables(&self, variables: RemovePropertyVariables) -> Result<Option<Component>, InexorRgfClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(remove_property_with_variables(variables))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.types.components.remove_property)
            .map(From::from);
        Ok(component)
    }
}
