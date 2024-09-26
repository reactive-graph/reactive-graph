use std::sync::Arc;

use crate::client::instances::relations::mutations::add_component::mutations::add_component;
use crate::client::instances::relations::mutations::add_property::mutations::add_property;
use crate::client::instances::relations::mutations::create::mutations::create;
use crate::client::instances::relations::mutations::delete::mutations::delete;
use crate::client::instances::relations::mutations::remove_component::mutations::remove_component;
use crate::client::instances::relations::mutations::remove_property::mutations::remove_property;
use crate::client::instances::relations::mutations::set_property::mutations::set_property;
use crate::client::instances::relations::queries::get_by_id::queries::get_by_id;
use crate::client::instances::relations::queries::search::queries::search;
use crate::client::instances::relations::variables::search::variables::SearchRelationInstancesVariables;
use crate::client::ReactiveGraphClient;
use crate::client::ReactiveGraphClientExecutionError;
use cynic::http::ReqwestExt;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::PropertyInstances;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::RelationInstance;
use reactive_graph_graph::RelationInstanceId;
use serde_json::Value;

pub struct RelationInstances {
    client: Arc<ReactiveGraphClient>,
}

impl RelationInstances {
    pub fn new(client: Arc<ReactiveGraphClient>) -> Self {
        Self { client }
    }

    pub async fn search(&self, search_query: SearchRelationInstancesVariables) -> Result<Option<Vec<RelationInstance>>, ReactiveGraphClientExecutionError> {
        let relation_instances = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(search(search_query))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| crate::schema_graphql::instances::relation_instance::RelationInstances(data.instances.relations))
            .map(From::from);
        Ok(relation_instances)
    }

    pub async fn get_by_id<ID: Into<RelationInstanceId>>(&self, id: ID) -> Result<Option<RelationInstance>, ReactiveGraphClientExecutionError> {
        let id = id.into();
        let relation_instance = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(get_by_id(&id))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| data.instances.relations.first().cloned())
            .map(From::from);
        Ok(relation_instance)
    }

    pub async fn set_property<ID: Into<RelationInstanceId>, S: Into<String>, V: Into<Value>>(
        &self,
        id: ID,
        name: S,
        value: V,
    ) -> Result<Option<RelationInstance>, ReactiveGraphClientExecutionError> {
        let id = id.into();
        let name = name.into();
        let value = value.into();
        let relation_instance = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(set_property(&id, name, value))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.instances.relations.update)
            .map(From::from);
        Ok(relation_instance)
    }

    pub async fn add_property<ID: Into<RelationInstanceId>, PT: Into<PropertyType>>(
        &self,
        id: ID,
        property_type: PT,
    ) -> Result<Option<RelationInstance>, ReactiveGraphClientExecutionError> {
        let id = id.into();
        let property_type = property_type.into();
        let relation_instance = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(add_property(&id, property_type))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.instances.relations.update)
            .map(From::from);
        Ok(relation_instance)
    }

    pub async fn remove_property<ID: Into<RelationInstanceId>, S: Into<String>>(
        &self,
        id: ID,
        property_name: S,
    ) -> Result<Option<RelationInstance>, ReactiveGraphClientExecutionError> {
        let id = id.into();
        let property_name = property_name.into();
        let relation_instance = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(remove_property(&id, property_name))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.instances.relations.update)
            .map(From::from);
        Ok(relation_instance)
    }

    pub async fn add_component<ID: Into<RelationInstanceId>, C: Into<ComponentTypeId>>(
        &self,
        id: ID,
        component_ty: C,
    ) -> Result<Option<RelationInstance>, ReactiveGraphClientExecutionError> {
        let id = id.into();
        let component_ty = component_ty.into();
        let relation_instance = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(add_component(&id, component_ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.instances.relations.update)
            .map(From::from);
        Ok(relation_instance)
    }

    pub async fn remove_component<ID: Into<RelationInstanceId>, C: Into<ComponentTypeId>>(
        &self,
        id: ID,
        component_ty: C,
    ) -> Result<Option<RelationInstance>, ReactiveGraphClientExecutionError> {
        let id = id.into();
        let component_ty = component_ty.into();
        let relation_instance = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(remove_component(&id, component_ty))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.instances.relations.update)
            .map(From::from);
        Ok(relation_instance)
    }

    pub async fn create<ID: Into<RelationInstanceId>>(
        &self,
        id: ID,
        description: Option<String>,
        properties: PropertyInstances,
    ) -> Result<Option<RelationInstance>, ReactiveGraphClientExecutionError> {
        let id = id.into();
        let relation_instance = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(create(&id, description, properties))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.instances.relations.create)
            .map(From::from);
        Ok(relation_instance)
    }

    pub async fn delete<ID: Into<RelationInstanceId>>(&self, id: ID) -> Result<Option<bool>, ReactiveGraphClientExecutionError> {
        let id = id.into();
        let relation_instance = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(delete(&id))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.instances.relations.delete)
            .map(From::from);
        Ok(relation_instance)
    }
}
