use std::sync::RwLock;

use async_trait::async_trait;
use chrono::Utc;
use gql_client::Client;
use log::error;
use log::info;
use serde::Deserialize;

use crate::api::FailedToAddInstance;
use crate::api::FailedToFetchInstanceInfo;
use crate::api::FailedToFetchRemoteInstances;
use crate::api::FailedToUpdateInstance;
use crate::api::Lifecycle;
use crate::api::RemotesManager;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::model_runtime::InstanceAddress;
use crate::model_runtime::InstanceInfo;

#[wrapper]
pub struct RemoteInstancesStorage(RwLock<Vec<InstanceInfo>>);

#[provides]
fn create_remote_instances_storage() -> RemoteInstancesStorage {
    RemoteInstancesStorage(RwLock::new(Vec::new()))
}

#[component]
pub struct RemotesManagerImpl {
    remote_instances: RemoteInstancesStorage,
}

#[async_trait]
#[provides]
impl RemotesManager for RemotesManagerImpl {
    // Returns a copy
    fn get_all(&self) -> Vec<InstanceInfo> {
        self.remote_instances.0.read().unwrap().to_vec()
    }

    fn get(&self, address: &InstanceAddress) -> Option<InstanceInfo> {
        self.remote_instances.0.read().unwrap().iter().find(|i| i == &address).cloned()
    }

    fn has(&self, address: &InstanceAddress) -> bool {
        self.remote_instances.0.read().unwrap().iter().any(|i| i == address)
    }

    async fn add(&self, address: &InstanceAddress) -> Result<InstanceInfo, FailedToAddInstance> {
        if self.has(address) {
            return Err(FailedToAddInstance::InstanceAddressAlreadyExists);
        }
        match self.inspect_remote(address).await {
            Ok(instance) => {
                self.add_checked(instance);
                self.get(address).ok_or(FailedToAddInstance::InstanceNotAdded)
            }
            Err(e) => Err(FailedToAddInstance::FailedToFetchInstanceInfo(e)),
        }
    }

    fn remove(&self, address: &InstanceAddress) -> bool {
        if !self.has(address) {
            return false;
        }
        let mut writer = self.remote_instances.0.write().unwrap();
        writer.retain(|i| i != address);
        true
    }

    fn remove_all(&self) {
        let mut writer = self.remote_instances.0.write().unwrap();
        writer.clear();
    }

    async fn update(&self, address: &InstanceAddress) -> Result<InstanceInfo, FailedToUpdateInstance> {
        if !self.has(address) {
            return Err(FailedToUpdateInstance::InstanceAddressDoesNotExist);
        }
        match self.inspect_remote(address).await {
            Ok(instance) => {
                self.replace(instance);
                self.get(address).ok_or(FailedToUpdateInstance::InstanceNotUpdated)
            }
            Err(e) => Err(FailedToUpdateInstance::FailedToFetchInstanceInfo(e)),
        }
    }

    async fn fetch_and_add_remotes_from_remote(&self, address: &InstanceAddress) -> Result<Vec<InstanceInfo>, FailedToFetchRemoteInstances> {
        let remote_instances = self.fetch_remotes_from_remote(address).await?;
        let mut added_instances = Vec::new();
        for remote_instance in remote_instances {
            info!("{}", remote_instance.url());
            if let Ok(instance) = self.add(&remote_instance).await {
                added_instances.push(instance);
            }
        }
        Ok(added_instances)
    }
}

impl RemotesManagerImpl {
    async fn inspect_remote(&self, address: &InstanceAddress) -> Result<InstanceInfo, FailedToFetchInstanceInfo> {
        let query = include_str!("../../graphql/system/instance/get_instance_info.graphql");
        let client = Client::new(address.url());
        let data = client.query::<InstanceInfoQuery>(query).await;
        match data {
            Ok(Some(query)) => Ok(query.system.instance_info),
            Ok(None) => Err(FailedToFetchInstanceInfo::InvalidResponseData),
            Err(e) => {
                error!("{}", e);
                Err(FailedToFetchInstanceInfo::RequestError(e))
            }
        }
    }

    async fn fetch_remotes_from_remote(&self, address: &InstanceAddress) -> Result<Vec<InstanceAddress>, FailedToFetchRemoteInstances> {
        let query = include_str!("../../graphql/system/remotes/get_all.graphql");
        let client = Client::new(address.url());
        let data = client.query::<RemoveInstancesQuery>(query).await;
        match data {
            Ok(Some(query)) => Ok(query.system.remote_instances),
            Ok(None) => Err(FailedToFetchRemoteInstances::InvalidResponseData),
            Err(e) => {
                error!("{}", e);
                Err(FailedToFetchRemoteInstances::RequestError(e))
            }
        }
    }

    fn add_checked(&self, instance: InstanceInfo) {
        if self.has(&instance.address()) {
            return;
        }
        let mut instance = instance;
        instance.last_seen = Utc::now();
        let mut writer = self.remote_instances.0.write().unwrap();
        writer.push(instance);
    }

    fn replace(&self, instance: InstanceInfo) {
        self.remove(&instance.address());
        let mut instance = instance;
        instance.last_seen = Utc::now();
        self.add_checked(instance);
    }
}

#[async_trait]
impl Lifecycle for RemotesManagerImpl {
    async fn post_init(&self) {}
}

#[derive(Deserialize)]
struct InstanceInfoQuery {
    system: InstanceInfoSystem,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct InstanceInfoSystem {
    instance_info: InstanceInfo,
}

#[derive(Deserialize)]
struct RemoveInstancesQuery {
    system: RemoveInstancesSystem,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RemoveInstancesSystem {
    remote_instances: Vec<InstanceAddress>,
}