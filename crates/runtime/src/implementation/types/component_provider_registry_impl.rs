use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use log::trace;

use crate::api::ComponentManager;
use crate::api::ComponentProviderRegistry;
use crate::api::Lifecycle;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::Components;
use crate::plugins::TypeProvider;

#[wrapper]
pub struct ComponentsProviders(DashMap<String, Arc<dyn TypeProvider<Components>>>);

#[provides]
fn create_component_provider_storage() -> ComponentsProviders {
    ComponentsProviders(DashMap::new())
}

#[component]
pub struct ComponentProviderRegistryImpl {
    component_manager: Wrc<dyn ComponentManager>,
    providers: ComponentsProviders,
}

impl ComponentProviderRegistryImpl {
    fn get_ids(&self) -> Vec<String> {
        self.providers.iter().map(|provider| String::from(provider.id())).collect()
    }
}

#[async_trait]
#[provides]
impl ComponentProviderRegistry for ComponentProviderRegistryImpl {
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<Components>>) {
        trace!("Registering provider {}", provider.id());
        for (ty, component) in provider.get_types().into_iter() {
            trace!("Registering component: {ty}");
            if self.component_manager.register(component.clone()).is_err() {
                trace!("Merging component: {ty}");
                let _ = self.component_manager.merge(component.clone());
            }
        }
        self.providers.insert(String::from(provider.id()), provider);
    }

    async fn unregister_provider(&self, id: &str) {
        if let Some((id, provider)) = self.providers.remove(id) {
            trace!("Unregistering provider {id}");
            for ty in provider.get_type_ids().into_iter() {
                trace!("Unregistering component: {ty}");
                self.component_manager.delete(&ty);
            }
        }
    }
}

#[async_trait]
impl Lifecycle for ComponentProviderRegistryImpl {
    async fn shutdown(&self) {
        for id in self.get_ids().iter() {
            self.unregister_provider(&id).await;
        }
    }
}
