use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use log::trace;
use springtime_di::component_alias;
use springtime_di::Component;

use reactive_graph_graph::Components;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::ComponentProviderRegistry;
use reactive_graph_type_system_api::TypeProvider;

#[derive(Component)]
pub struct ComponentProviderRegistryImpl {
    component_manager: Arc<dyn ComponentManager + Send + Sync>,

    #[component(default = "DashMap::new")]
    providers: DashMap<String, Arc<dyn TypeProvider<Components>>>,
}

impl ComponentProviderRegistryImpl {
    fn get_ids(&self) -> Vec<String> {
        self.providers.iter().map(|provider| String::from(provider.id())).collect()
    }
}

#[async_trait]
#[component_alias]
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
            self.unregister_provider(id).await;
        }
    }
}
