use std::sync::Arc;

// use crate::client::instances::relations::add_component::queries::add_component;
// use crate::client::instances::relations::add_property::queries::add_property;
// use crate::client::instances::relations::create::queries::create;
// use crate::client::instances::relations::delete::queries::delete_relation_instance_mutation;
// use crate::client::instances::relations::get_by_id::queries::get_relation_instance_by_id;
// use crate::client::instances::relations::get_by_label::queries::get_relation_instance_by_label;
// use crate::client::instances::relations::remove_component::queries::remove_component;
// use crate::client::instances::relations::remove_property::queries::remove_property;
use crate::client::instances::relations::search::queries::search;
use crate::client::instances::relations::search::queries::SearchRelationInstancesVariables;
// use crate::client::instances::relations::set_property::queries::set_property;
use crate::client::InexorRgfClient;
use crate::client::InexorRgfClientExecutionError;
use cynic::http::ReqwestExt;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::PropertyInstances;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::RelationInstance;
use reactive_graph_graph::RelationTypeId;
use serde_json::Value;
use uuid::Uuid;

pub struct RelationInstances {
    client: Arc<InexorRgfClient>,
}

impl RelationInstances {
    pub fn new(client: Arc<InexorRgfClient>) -> Self {
        Self { client }
    }

    pub async fn search(&self, search_query: SearchRelationInstancesVariables) -> Result<Option<Vec<RelationInstance>>, InexorRgfClientExecutionError> {
        let relation_instances = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(search(search_query))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| crate::schema_graphql::instances::relation_instance::RelationInstances(data.instances.relations))
            .map(From::from);
        Ok(relation_instances)
    }
    //
    // pub async fn get_relation_instance_by_id<ID: Into<Uuid>>(&self, id: ID) -> Result<Option<RelationInstance>, InexorRgfClientExecutionError> {
    //     let id = id.into();
    //     let relation_instance = self
    //         .client
    //         .client
    //         .post(self.client.url_graphql())
    //         .run_graphql(get_relation_instance_by_id(id))
    //         .await
    //         .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
    //         .data
    //         .and_then(|data| data.instances.relations.first().cloned())
    //         .map(From::from);
    //     Ok(relation_instance)
    // }
    //
    // pub async fn get_relation_instance_by_label<L: Into<String>>(&self, label: L) -> Result<Option<RelationInstance>, InexorRgfClientExecutionError> {
    //     let label = label.into();
    //     let relation_instance = self
    //         .client
    //         .client
    //         .post(self.client.url_graphql())
    //         .run_graphql(get_relation_instance_by_label(label))
    //         .await
    //         .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
    //         .data
    //         .and_then(|data| data.instances.relations.first().cloned())
    //         .map(From::from);
    //     Ok(relation_instance)
    // }
    //
    // pub async fn set_property<ID: Into<Uuid>, S: Into<String>, V: Into<Value>>(
    //     &self,
    //     id: ID,
    //     name: S,
    //     value: V,
    // ) -> Result<Option<RelationInstance>, InexorRgfClientExecutionError> {
    //     let id = id.into();
    //     let name = name.into();
    //     let value = value.into();
    //     let relation_instance = self
    //         .client
    //         .client
    //         .post(self.client.url_graphql())
    //         .run_graphql(set_property(id, name, value))
    //         .await
    //         .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
    //         .data
    //         .map(|data| data.instances.relations.update)
    //         .map(From::from);
    //     Ok(relation_instance)
    // }
    //
    // pub async fn add_property<ID: Into<Uuid>, PT: Into<PropertyType>>(
    //     &self,
    //     id: ID,
    //     property_type: PT,
    // ) -> Result<Option<RelationInstance>, InexorRgfClientExecutionError> {
    //     let id = id.into();
    //     let property_type = property_type.into();
    //     let relation_instance = self
    //         .client
    //         .client
    //         .post(self.client.url_graphql())
    //         .run_graphql(add_property(id, property_type))
    //         .await
    //         .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
    //         .data
    //         .map(|data| data.instances.relations.update)
    //         .map(From::from);
    //     Ok(relation_instance)
    // }
    //
    // pub async fn remove_property<ID: Into<Uuid>, S: Into<String>>(
    //     &self,
    //     id: ID,
    //     property_name: S,
    // ) -> Result<Option<RelationInstance>, InexorRgfClientExecutionError> {
    //     let id = id.into();
    //     let property_name = property_name.into();
    //     let relation_instance = self
    //         .client
    //         .client
    //         .post(self.client.url_graphql())
    //         .run_graphql(remove_property(id, property_name))
    //         .await
    //         .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
    //         .data
    //         .map(|data| data.instances.relations.update)
    //         .map(From::from);
    //     Ok(relation_instance)
    // }
    //
    // pub async fn add_component<ID: Into<Uuid>, C: Into<ComponentTypeId>>(
    //     &self,
    //     id: ID,
    //     component_ty: C,
    // ) -> Result<Option<RelationInstance>, InexorRgfClientExecutionError> {
    //     let id = id.into();
    //     let component_ty = component_ty.into();
    //     let relation_instance = self
    //         .client
    //         .client
    //         .post(self.client.url_graphql())
    //         .run_graphql(add_component(id, component_ty))
    //         .await
    //         .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
    //         .data
    //         .map(|data| data.instances.relations.update)
    //         .map(From::from);
    //     Ok(relation_instance)
    // }
    //
    // pub async fn remove_component<ID: Into<Uuid>, C: Into<ComponentTypeId>>(
    //     &self,
    //     id: ID,
    //     component_ty: C,
    // ) -> Result<Option<RelationInstance>, InexorRgfClientExecutionError> {
    //     let id = id.into();
    //     let component_ty = component_ty.into();
    //     let relation_instance = self
    //         .client
    //         .client
    //         .post(self.client.url_graphql())
    //         .run_graphql(remove_component(id, component_ty))
    //         .await
    //         .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
    //         .data
    //         .map(|data| data.instances.relations.update)
    //         .map(From::from);
    //     Ok(relation_instance)
    // }
    //
    // pub async fn create<TY: Into<RelationTypeId>, ID: Into<Uuid>>(
    //     &self,
    //     ty: TY,
    //     id: Option<ID>,
    //     description: Option<String>,
    //     properties: PropertyInstances,
    // ) -> Result<Option<RelationInstance>, InexorRgfClientExecutionError> {
    //     let ty = ty.into();
    //     let id = id.map(|id| id.into());
    //     let relation_instance = self
    //         .client
    //         .client
    //         .post(self.client.url_graphql())
    //         .run_graphql(create(ty, id, description, properties))
    //         .await
    //         .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
    //         .data
    //         .map(|data| data.instances.relations.create)
    //         .map(From::from);
    //     Ok(relation_instance)
    // }
    //
    // pub async fn delete_relation_instance<ID: Into<Uuid>>(&self, id: ID) -> Result<Option<bool>, InexorRgfClientExecutionError> {
    //     let id = id.into();
    //     let relation_instance = self
    //         .client
    //         .client
    //         .post(self.client.url_graphql())
    //         .run_graphql(delete_relation_instance_mutation(id))
    //         .await
    //         .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
    //         .data
    //         .map(|data| data.instances.relations.delete)
    //         .map(From::from);
    //     Ok(relation_instance)
    // }
}
