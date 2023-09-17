use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use log::trace;

use crate::api::Lifecycle;
use crate::api::RelationTypeManager;
use crate::api::RelationTypeProviderRegistry;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::RelationTypes;
use crate::plugins::TypeProvider;

#[wrapper]
pub struct RelationTypesProviders(DashMap<String, Arc<dyn TypeProvider<RelationTypes>>>);

#[provides]
fn create_relation_type_provider_storage() -> RelationTypesProviders {
    RelationTypesProviders(DashMap::new())
}

#[component]
pub struct RelationTypeProviderRegistryImpl {
    relation_type_manager: Wrc<dyn RelationTypeManager>,
    providers: RelationTypesProviders,
}

impl RelationTypeProviderRegistryImpl {
    fn get_ids(&self) -> Vec<String> {
        self.providers.iter().map(|provider| String::from(provider.id())).collect()
    }
}

#[async_trait]
#[provides]
impl RelationTypeProviderRegistry for RelationTypeProviderRegistryImpl {
    async fn register_provider(&self, provider: Arc<dyn TypeProvider<RelationTypes>>) {
        trace!("Registering provider {}", provider.id());
        for (ty, relation_type) in provider.get_types().into_iter() {
            trace!("Registering relation type: {ty}");
            if self.relation_type_manager.register(relation_type.clone()).is_err() {
                trace!("Merging relation type: {ty}");
                let _ = self.relation_type_manager.merge(relation_type);
            }
        }
        self.providers.insert(String::from(provider.id()), provider);
    }

    async fn unregister_provider(&self, id: &str) {
        if let Some((id, provider)) = self.providers.remove(id) {
            trace!("Unregistering provider {id}");
            for ty in provider.get_type_ids().into_iter() {
                trace!("Unregistering relation type: {ty}");
                self.relation_type_manager.delete(&ty);
            }
        }
    }
}

#[async_trait]
impl Lifecycle for RelationTypeProviderRegistryImpl {
    async fn shutdown(&self) {
        for id in self.get_ids().iter() {
            self.unregister_provider(&id).await;
        }
    }
}
