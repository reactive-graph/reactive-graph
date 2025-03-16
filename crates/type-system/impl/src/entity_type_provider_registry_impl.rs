use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use log::trace;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_graph::EntityTypes;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::EntityTypeProviderRegistry;
use reactive_graph_type_system_api::TypeProvider;

#[derive(Component)]
pub struct EntityTypeProviderRegistryImpl {
    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,

    #[component(default = "DashMap::new")]
    providers: DashMap<String, Arc<dyn TypeProvider<EntityTypes>>>, // EntityTypesProviders,
}

impl EntityTypeProviderRegistryImpl {
    fn get_ids(&self) -> Vec<String> {
        self.providers.iter().map(|provider| String::from(provider.id())).collect()
    }
}

#[async_trait]
#[component_alias]
impl EntityTypeProviderRegistry for EntityTypeProviderRegistryImpl {
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<EntityTypes>>) {
        trace!("Registering provider {}", provider.id());
        for (ty, entity_type) in provider.get_types().into_iter() {
            trace!("Registering entity type: {ty}");
            if self.entity_type_manager.register(entity_type.clone()).is_err() {
                trace!("Merging entity type: {ty}");
                let _ = self.entity_type_manager.merge(entity_type.clone());
            }
        }
        self.providers.insert(String::from(provider.id()), provider);
    }

    async fn unregister_provider(&self, id: &str) {
        if let Some((id, provider)) = self.providers.remove(id) {
            trace!("Unregistering provider {id}");
            for ty in provider.get_type_ids().into_iter() {
                trace!("Unregistering entity type: {ty}");
                self.entity_type_manager.delete(&ty);
            }
        }
    }
}

#[async_trait]
impl Lifecycle for EntityTypeProviderRegistryImpl {
    async fn shutdown(&self) {
        for id in self.get_ids().iter() {
            self.unregister_provider(id).await;
        }
    }
}
