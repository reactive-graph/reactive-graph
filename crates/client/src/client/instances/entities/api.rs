use std::sync::Arc;

use crate::client::instances::entities::create::queries::create;
use crate::client::instances::entities::delete::queries::delete_entity_instance_mutation;
use crate::client::instances::entities::get_by_id::queries::get_entity_instance_by_id;
use crate::client::instances::entities::get_by_label::queries::get_entity_instance_by_label;
use crate::client::instances::entities::search::queries::search;
use crate::client::instances::entities::search::queries::SearchEntityInstancesVariables;
use crate::client::instances::entities::set_property::queries::set_property;
use crate::client::InexorRgfClient;
use crate::client::InexorRgfClientExecutionError;
use cynic::http::ReqwestExt;
use reactive_graph_graph::EntityInstance;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::PropertyInstances;
use serde_json::Value;
use uuid::Uuid;

pub struct EntityInstances {
    client: Arc<InexorRgfClient>,
}

impl EntityInstances {
    pub fn new(client: Arc<InexorRgfClient>) -> Self {
        Self { client }
    }

    pub async fn search(&self, search_query: SearchEntityInstancesVariables) -> Result<Option<Vec<EntityInstance>>, InexorRgfClientExecutionError> {
        let entity_instances = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(search(search_query))
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

    pub async fn set_property<ID: Into<Uuid>>(&self, id: ID, name: String, value: Value) -> Result<Option<EntityInstance>, InexorRgfClientExecutionError> {
        let id = id.into();
        let entity_instance = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(set_property(id, name, value))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.instances.entities.update)
            .map(From::from);
        Ok(entity_instance)
    }

    pub async fn create<TY: Into<EntityTypeId>, ID: Into<Uuid>>(
        &self,
        ty: TY,
        id: Option<ID>,
        description: Option<String>,
        properties: PropertyInstances,
    ) -> Result<Option<EntityInstance>, InexorRgfClientExecutionError> {
        let ty = ty.into();
        let id = id.map(|id| id.into());
        let entity_instance = self
            .client
            .client
            .post(self.client.url_graphql())
            .run_graphql(create(ty, id, description, properties))
            .await
            .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
            .data
            .map(|data| data.instances.entities.create)
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
