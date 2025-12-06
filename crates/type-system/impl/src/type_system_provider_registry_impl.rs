use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use log::info;
use log::trace;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_graph::Namespace;
use reactive_graph_graph::TypeSystem;
use reactive_graph_graph::TypeSystemProvider;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::FlowTypeManager;
use reactive_graph_type_system_api::RelationTypeManager;
use reactive_graph_type_system_api::TypeSystemProviderRegistry;

#[derive(Component)]
pub struct TypeSystemProviderRegistryImpl {
    component_manager: Arc<dyn ComponentManager + Send + Sync>,

    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,

    relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,

    flow_type_manager: Arc<dyn FlowTypeManager + Send + Sync>,

    #[component(default = "DashMap::new")]
    providers: DashMap<Namespace, TypeSystem>,
}

impl TypeSystemProviderRegistryImpl {
    fn get_ids(&self) -> Vec<Namespace> {
        self.providers.iter().map(|provider| provider.key().clone()).collect()
    }
}

#[async_trait]
#[component_alias]
impl TypeSystemProviderRegistry for TypeSystemProviderRegistryImpl {
    async fn register_provider(&self, provider: TypeSystemProvider) {
        let (id, type_system) = provider.unpack();
        info!("Registering type system provider {id}");
        for component in type_system.components().iter() {
            trace!("Registering component: {}", component.ty);
            if self.component_manager.register(component.clone()).is_err() {
                trace!("Merging component: {}", component.ty);
                let _ = self.component_manager.merge(component.clone());
            }
        }
        for entity_type in type_system.entity_types().iter() {
            trace!("Registering entity type: {}", entity_type.ty);
            if self.entity_type_manager.register(entity_type.clone()).is_err() {
                trace!("Merging entity type: {}", entity_type.ty);
                let _ = self.entity_type_manager.merge(entity_type.clone());
            }
        }
        for relation_type in type_system.relation_types().iter() {
            trace!("Registering relation type: {}", relation_type.ty);
            if self.relation_type_manager.register(relation_type.clone()).is_err() {
                trace!("Merging relation type: {}", relation_type.ty);
                let _ = self.relation_type_manager.merge(relation_type.clone());
            }
        }
        for flow_type in type_system.flow_types().iter() {
            trace!("Registering flow type: {}", flow_type.ty);
            if self.flow_type_manager.register(flow_type.clone()).is_err() {
                trace!("Merging flow type: {}", flow_type.ty);
                let _ = self.flow_type_manager.register(flow_type.clone());
            }
        }
        self.providers.insert(id, type_system);
    }

    async fn unregister_provider(&self, id: &Namespace) {
        if let Some((id, type_system)) = self.providers.remove(id) {
            trace!("Unregistering type system provider {id}");
            for flow_type in type_system.flow_types().iter() {
                let ty = flow_type.key();
                trace!("Unregistering flow type: {ty}");
                self.flow_type_manager.delete(ty);
            }
            for relation_type in type_system.relation_types().iter() {
                let ty = relation_type.key();
                trace!("Unregistering relation type: {ty}");
                self.relation_type_manager.delete(ty);
            }
            for entity_type in type_system.entity_types().iter() {
                let ty = entity_type.key();
                trace!("Unregistering entity type: {ty}");
                self.entity_type_manager.delete(ty);
            }
            for component in type_system.components().iter() {
                let ty = component.key();
                trace!("Unregistering component: {ty}");
                self.component_manager.delete(ty);
            }
        }
    }
}

#[async_trait]
impl Lifecycle for TypeSystemProviderRegistryImpl {
    async fn shutdown(&self) {
        for id in self.get_ids().iter() {
            self.unregister_provider(id).await;
        }
    }
}
