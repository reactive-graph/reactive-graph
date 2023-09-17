use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use log::trace;

use crate::api::FlowTypeManager;
use crate::api::FlowTypeProviderRegistry;
use crate::api::Lifecycle;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::FlowTypes;
use crate::plugins::TypeProvider;

#[wrapper]
pub struct FlowTypesProviders(DashMap<String, Arc<dyn TypeProvider<FlowTypes>>>);

#[provides]
fn create_flow_type_provider_storage() -> FlowTypesProviders {
    FlowTypesProviders(DashMap::new())
}

#[component]
pub struct FlowTypeProviderRegistryImpl {
    flow_type_manager: Wrc<dyn FlowTypeManager>,
    providers: FlowTypesProviders,
}

impl FlowTypeProviderRegistryImpl {
    fn get_ids(&self) -> Vec<String> {
        self.providers.iter().map(|provider| String::from(provider.id())).collect()
    }
}

#[async_trait]
#[provides]
impl FlowTypeProviderRegistry for FlowTypeProviderRegistryImpl {
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<FlowTypes>>) {
        trace!("Registering provider {}", provider.id());
        for (ty, flow_type) in provider.get_types().into_iter() {
            trace!("Registering flow type: {ty}");
            if self.flow_type_manager.register(flow_type.clone()).is_err() {
                // trace!("Merging flow type: {ty}");
                // let _ = self.flow_type_manager.merge(flow_type.clone());
            }
        }
        self.providers.insert(String::from(provider.id()), provider);
    }

    async fn unregister_provider(&self, id: &str) {
        if let Some((id, provider)) = self.providers.remove(id) {
            trace!("Unregistering provider {id}");
            for ty in provider.get_type_ids().into_iter() {
                trace!("Unregistering flow type: {ty}");
                self.flow_type_manager.delete(&ty);
            }
        }
    }
}

#[async_trait]
impl Lifecycle for FlowTypeProviderRegistryImpl {
    async fn shutdown(&self) {
        for id in self.get_ids().iter() {
            self.unregister_provider(&id).await;
        }
    }
}
