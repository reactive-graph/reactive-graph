use std::sync::Arc;

use crate::client::instances::entities::delete::queries::delete_entity_instance_mutation;
use crate::client::instances::entities::get_all::queries::get_all_entity_instances_query;
use crate::client::instances::entities::get_by_id::queries::get_entity_instance_by_id;
use crate::client::instances::entities::get_by_label::queries::get_entity_instance_by_label;
use crate::client::InexorRgfClient;
use crate::client::InexorRgfClientExecutionError;
use cynic::http::ReqwestExt;
use reactive_graph_graph::EntityInstance;
use uuid::Uuid;

pub struct EntityInstances {
    client: Arc<InexorRgfClient>,
}

impl EntityInstances {
    pub fn new(client: Arc<InexorRgfClient>) -> Self {
        Self { client }
    }

    pub async fn get_all_entity_instances(&self) -> Result<Option<Vec<EntityInstance>>, InexorRgfClientExecutionError> {
        let entity_instances = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(get_all_entity_instances_query())
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| crate::schema_graphql::instances::entity_instance::EntityInstances(data.instances.entities))
            .map(From::from);
        Ok(entity_instances)
    }

    pub async fn get_entity_instance_by_id<ID: Into<Uuid>>(&self, id: ID) -> Result<Option<EntityInstance>, InexorRgfClientExecutionError> {
        let id = id.into();
        let entity_instance = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(get_entity_instance_by_id(id))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| data.instances.entities.first().cloned())
            .map(From::from);
        Ok(entity_instance)
    }

    pub async fn get_entity_instance_by_label<L: Into<String>>(&self, label: L) -> Result<Option<EntityInstance>, InexorRgfClientExecutionError> {
        let label = label.into();
        let entity_instance = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(get_entity_instance_by_label(label))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .and_then(|data| data.instances.entities.first().cloned())
            .map(From::from);
        Ok(entity_instance)
    }

    pub async fn delete_entity_instance<ID: Into<Uuid>>(&self, id: ID) -> Result<Option<bool>, InexorRgfClientExecutionError> {
        let id = id.into();
        let entity_instance = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(delete_entity_instance_mutation(id))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.instances.entities.delete)
            .map(From::from);
        Ok(entity_instance)
    }
}
