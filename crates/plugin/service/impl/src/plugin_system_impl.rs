use std::sync::Arc;

use async_trait::async_trait;
use reactive_graph_di::get_shared_component_factory;
use springtime_di::component_alias;
// use springtime_di::factory::ComponentFactoryBuilder;
use springtime_di::instance_provider::TypedComponentInstanceProvider;
use springtime_di::Component;

use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_plugin_service_api::PluginContainerManager;
use reactive_graph_plugin_service_api::PluginContextFactory;
use reactive_graph_plugin_service_api::PluginRepositoryManager;
use reactive_graph_plugin_service_api::PluginResolver;
use reactive_graph_plugin_service_api::PluginSystem;

#[derive(Component)]
pub struct PluginSystemImpl {
    plugin_container_manager: Arc<dyn PluginContainerManager + Send + Sync>,
    plugin_context_factory: Arc<dyn PluginContextFactory + Send + Sync>,
    plugin_repository_manager: Arc<dyn PluginRepositoryManager + Send + Sync>,
    plugin_resolver: Arc<dyn PluginResolver + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl PluginSystem for PluginSystemImpl {
    fn get_plugin_context_factory(&self) -> Arc<dyn PluginContextFactory + Send + Sync> {
        self.plugin_context_factory.clone()
    }

    fn get_plugin_container_manager(&self) -> Arc<dyn PluginContainerManager + Send + Sync> {
        self.plugin_container_manager.clone()
    }

    fn get_plugin_repository_manager(&self) -> Arc<dyn PluginRepositoryManager + Send + Sync> {
        self.plugin_repository_manager.clone()
    }

    fn get_plugin_resolver(&self) -> Arc<dyn PluginResolver + Send + Sync> {
        self.plugin_resolver.clone()
    }
}

#[async_trait]
impl Lifecycle for PluginSystemImpl {
    async fn init(&self) {
        self.plugin_context_factory.init().await;
        self.plugin_repository_manager.init().await;
        self.plugin_resolver.init().await;
    }

    async fn post_init(&self) {
        self.plugin_context_factory.post_init().await;
        self.plugin_repository_manager.post_init().await;
        self.plugin_resolver.post_init().await;
    }

    async fn pre_shutdown(&self) {
        self.plugin_resolver.pre_shutdown().await;
        self.plugin_repository_manager.pre_shutdown().await;
        self.plugin_context_factory.pre_shutdown().await;
    }

    async fn shutdown(&self) {
        self.plugin_resolver.shutdown().await;
        self.plugin_repository_manager.shutdown().await;
        self.plugin_context_factory.shutdown().await;
    }
}

pub fn get_plugin_system() -> Arc<dyn PluginSystem + Send + Sync> {
    let mut component_factory = get_shared_component_factory();
    match TypedComponentInstanceProvider::primary_instance_typed::<dyn PluginSystem + Send + Sync>(&mut component_factory) {
        Ok(runtime) => runtime,
        Err(e) => {
            panic!("{}", e);
        }
    }

    // match ComponentFactoryBuilder::new() {
    //     Ok(component_factory) => {
    //         let mut component_factory = component_factory.build();
    //         match TypedComponentInstanceProvider::primary_instance_typed::<PluginSystemImpl>(&mut component_factory) {
    //             Ok(runtime) => runtime,
    //             Err(e) => {
    //                 panic!("{}", e);
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         panic!("{}", e);
    //     }
    // }
}
