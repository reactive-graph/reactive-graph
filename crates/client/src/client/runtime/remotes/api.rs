use std::sync::Arc;

use crate::client::runtime::remotes::mutations::add_remote::mutations::add;
use crate::client::runtime::remotes::mutations::fetch_remotes_from_all_remotes::mutations::fetch_remotes_from_all_remotes;
use crate::client::runtime::remotes::mutations::fetch_remotes_from_remote::mutations::fetch_remotes_from_remote;
use crate::client::runtime::remotes::mutations::remove_all_remotes::mutations::remove_all;
use crate::client::runtime::remotes::mutations::remove_remote::mutations::remove;
use crate::client::runtime::remotes::mutations::update_all_remotes::mutations::update_all;
use crate::client::runtime::remotes::mutations::update_remote::mutations::update;
use crate::client::runtime::remotes::queries::get_all::queries::get_all;
use crate::schema_runtime::InstanceInfos;
use crate::ReactiveGraphClient;
use crate::ReactiveGraphClientExecutionError;
use reactive_graph_remotes_model::InstanceAddress;
use reactive_graph_remotes_model::InstanceInfo;

pub struct Remotes {
    client: Arc<ReactiveGraphClient>,
}

impl Remotes {
    pub fn new(client: Arc<ReactiveGraphClient>) -> Self {
        Self { client }
    }

    pub async fn get_all(&self) -> Result<Vec<InstanceInfo>, ReactiveGraphClientExecutionError> {
        self.client.execute_runtime(get_all(), |data| InstanceInfos(data.remotes).into()).await
    }

    pub async fn add(&self, address: &InstanceAddress) -> Result<InstanceInfo, ReactiveGraphClientExecutionError> {
        self.client.execute_runtime(add(address.into()), |data| data.remotes.add.into()).await
    }

    pub async fn remove(&self, address: &InstanceAddress) -> Result<bool, ReactiveGraphClientExecutionError> {
        self.client.execute_runtime(remove(address.into()), |data| data.remotes.remove).await
    }

    pub async fn remove_all(&self) -> Result<bool, ReactiveGraphClientExecutionError> {
        self.client.execute_runtime(remove_all(), |data| data.remotes.remove_all).await
    }

    pub async fn update(&self, address: &InstanceAddress) -> Result<InstanceInfo, ReactiveGraphClientExecutionError> {
        self.client.execute_runtime(update(address.into()), |data| data.remotes.update.into()).await
    }

    pub async fn update_all(&self) -> Result<Vec<InstanceInfo>, ReactiveGraphClientExecutionError> {
        self.client
            .execute_runtime(update_all(), |data| InstanceInfos(data.remotes.update_all).into())
            .await
    }

    pub async fn fetch_remotes_from_remote(&self, address: &InstanceAddress) -> Result<Vec<InstanceInfo>, ReactiveGraphClientExecutionError> {
        self.client
            .execute_runtime(fetch_remotes_from_remote(address.into()), |data| InstanceInfos(data.remotes.fetch_remotes_from_remote).into())
            .await
    }

    pub async fn fetch_remotes_from_all_remotes(&self) -> Result<Vec<InstanceInfo>, ReactiveGraphClientExecutionError> {
        self.client
            .execute_runtime(fetch_remotes_from_all_remotes(), |data| InstanceInfos(data.remotes.fetch_remotes_from_all_remotes).into())
            .await
    }
}
