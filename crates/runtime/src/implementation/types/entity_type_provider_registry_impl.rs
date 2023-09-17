use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use log::trace;

use crate::api::EntityTypeManager;
use crate::api::EntityTypeProviderRegistry;
use crate::api::Lifecycle;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::EntityTypes;
use crate::plugins::TypeProvider;

#[wrapper]
pub struct EntityTypesProviders(DashMap<String, Arc<dyn TypeProvider<EntityTypes>>>);

#[provides]
fn create_entity_type_provider_storage() -> EntityTypesProviders {
    EntityTypesProviders(DashMap::new())
}

#[component]
pub struct EntityTypeProviderRegistryImpl {
    entity_type_manager: Wrc<dyn EntityTypeManager>,
    providers: EntityTypesProviders,
}

impl EntityTypeProviderRegistryImpl {
    fn get_ids(&self) -> Vec<String> {
        self.providers.iter().map(|provider| String::from(provider.id())).collect()
    }
}

#[async_trait]
#[provides]
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
            self.unregister_provider(&id).await;
        }
    }
}
