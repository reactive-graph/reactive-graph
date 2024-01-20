use std::collections::HashSet;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use dashmap::DashSet;
use gql_client::Client;
use log::error;
use log::info;
use serde::Deserialize;

use inexor_rgf_config_api::ConfigManager;
use inexor_rgf_config_model::RemotesConfig;
use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_remotes_api::FailedToAddInstance;
use inexor_rgf_remotes_api::FailedToFetchInstanceInfo;
use inexor_rgf_remotes_api::FailedToFetchRemoteInstances;
use inexor_rgf_remotes_api::FailedToUpdateInstance;
use inexor_rgf_remotes_api::RemotesManager;
use inexor_rgf_remotes_model::InstanceAddress;
use inexor_rgf_remotes_model::InstanceInfo;
use springtime_di::component_alias;
use springtime_di::Component;

#[derive(Component)]
pub struct RemotesManagerImpl {
    config_manager: Arc<dyn ConfigManager + Send + Sync>,

    #[component(default = "DashSet::new")]
    remote_instances: DashSet<InstanceInfo>,
}

#[async_trait]
#[component_alias]
impl RemotesManager for RemotesManagerImpl {
    // Returns a copy
    fn get_all(&self) -> Vec<InstanceInfo> {
        self.remote_instances.iter().map(|i| i.key().clone()).collect()
        // self.remote_instances.to_vec()
    }

    fn get(&self, address: &InstanceAddress) -> Option<InstanceInfo> {
        self.remote_instances.iter().find(|i| i.key() == address).map(|i| i.key().clone()) // .cloned()
    }

    fn has(&self, address: &InstanceAddress) -> bool {
        self.remote_instances.iter().any(|i| i.key() == address)
    }

    fn get_all_addresses(&self) -> HashSet<InstanceAddress> {
        self.remote_instances.iter().map(|i| i.address()).collect()
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
        // let mut writer = self.remote_instances.0.write().unwrap();
        // writer.retain(|i| i != address);
        self.remote_instances.retain(|i| i != address);
        let mut remotes = self.config_manager.get_remotes_config();
        remotes.remove(address);
        self.config_manager.set_remotes_config(remotes);
        self.config_manager.write_remotes_config();
        true
    }

    fn remove_all(&self) {
        // let mut writer = self.remote_instances.0.write().unwrap();
        // writer.clear();
        self.remote_instances.clear();
        self.config_manager.set_remotes_config(RemotesConfig::new(HashSet::new()));
        self.config_manager.write_remotes_config();
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

    async fn update_all(&self) -> Vec<InstanceInfo> {
        let mut updated_remotes = vec![];
        for address in self.get_all_addresses().iter() {
            if let Ok(instance) = self.update(address).await {
                self.replace(instance.clone());
                updated_remotes.push(instance);
            }
        }
        updated_remotes
    }

    async fn fetch_and_add_remotes_from_remote(&self, address: &InstanceAddress) -> Result<Vec<InstanceInfo>, FailedToFetchRemoteInstances> {
        let remote_instances = self.fetch_remotes_from_remote(address).await?;
        let mut added_instances = Vec::new();
        for remote_instance in remote_instances {
            info!("{}", remote_instance.url_runtime());
            if let Ok(instance) = self.add(&remote_instance).await {
                added_instances.push(instance);
            }
        }
        Ok(added_instances)
    }

    async fn fetch_and_add_remotes_from_all_remotes(&self) -> Vec<InstanceInfo> {
        let mut all_added_instances = Vec::new();
        for address in self.get_all_addresses().iter() {
            if let Ok(mut added_instances) = self.fetch_and_add_remotes_from_remote(address).await {
                all_added_instances.append(&mut added_instances);
            };
        }
        all_added_instances
    }
}

impl RemotesManagerImpl {
    async fn inspect_remote(&self, address: &InstanceAddress) -> Result<InstanceInfo, FailedToFetchInstanceInfo> {
        let query = include_str!("get_instance_info.graphql");
        let client = Client::new(address.url_runtime());
        let data = client.query::<InstanceInfoQuery>(query).await;
        match data {
            Ok(Some(query)) => Ok(query.instance_info),
            Ok(None) => Err(FailedToFetchInstanceInfo::InvalidResponseData),
            Err(e) => {
                error!("{}", e);
                Err(FailedToFetchInstanceInfo::RequestError(e))
            }
        }
    }

    async fn fetch_remotes_from_remote(&self, address: &InstanceAddress) -> Result<Vec<InstanceAddress>, FailedToFetchRemoteInstances> {
        let query = include_str!("get_all_remotes.graphql");
        let client = Client::new(address.url_runtime());
        let data = client.query::<FetchRemotesFromRemoteQuery>(query).await;
        match data {
            Ok(Some(query)) => Ok(query.remotes),
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
        let address = instance.address.clone();
        // let mut writer = self.remote_instances.0.write().unwrap();
        // writer.push(instance);
        self.remote_instances.insert(instance);
        let mut remotes = self.config_manager.get_remotes_config();
        remotes.merge(address);
        self.config_manager.set_remotes_config(remotes);
        self.config_manager.write_remotes_config();
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
    async fn post_init(&self) {
        for address in self.config_manager.get_remotes_config().into_iter() {
            match self.add(&address).await {
                Ok(instance_info) => {
                    info!("Added remote instance {} from {}", instance_info.name, instance_info.address().url_runtime());
                }
                Err(_) => {
                    error!("Failed to add remote instance {}", address.url_runtime())
                }
            }
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct InstanceInfoQuery {
    // system: InstanceInfoSystem,
    instance_info: InstanceInfo,
}

// #[derive(Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct InstanceInfoSystem {
//     instance_info: InstanceInfo,
// }

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FetchRemotesFromRemoteQuery {
    // system: FetchRemotesFromRemoteSystem,
    remotes: Vec<InstanceAddress>,
}

// #[derive(Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct FetchRemotesFromRemoteSystem {
//     remotes: Vec<InstanceAddress>,
// }
