use std::sync::Arc;

use async_trait::async_trait;
use reactive_graph_di::get_shared_component_factory;
use springtime_di::Component;
use springtime_di::component_alias;
use springtime_di::instance_provider::TypedComponentInstanceProvider;

use reactive_graph_graphql_api::GraphQLQueryService;
use reactive_graph_graphql_api::GraphQLSchemaManager;
use reactive_graph_graphql_api::GraphQLSystem;
use reactive_graph_lifecycle::Lifecycle;

#[derive(Component)]
pub struct GraphQLSystemImpl {
    graphql_query_service: Arc<dyn GraphQLQueryService + Send + Sync>,
    graphql_schema_manager: Arc<dyn GraphQLSchemaManager + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl GraphQLSystem for GraphQLSystemImpl {
    fn get_graphql_query_service(&self) -> Arc<dyn GraphQLQueryService + Send + Sync> {
        self.graphql_query_service.clone()
    }

    fn get_graphql_schema_manager(&self) -> Arc<dyn GraphQLSchemaManager + Send + Sync> {
        self.graphql_schema_manager.clone()
    }
}

#[async_trait]
impl Lifecycle for GraphQLSystemImpl {
    async fn init(&self) {
        self.graphql_schema_manager.init().await;
        self.graphql_query_service.init().await;
    }

    async fn post_init(&self) {
        self.graphql_schema_manager.post_init().await;
        self.graphql_query_service.post_init().await;
    }

    async fn pre_shutdown(&self) {
        self.graphql_query_service.pre_shutdown().await;
        self.graphql_schema_manager.pre_shutdown().await;
    }

    async fn shutdown(&self) {
        self.graphql_query_service.shutdown().await;
        self.graphql_schema_manager.shutdown().await;
    }
}

pub fn get_graphql_system() -> Arc<dyn GraphQLSystem + Send + Sync> {
    let mut component_factory = get_shared_component_factory();
    match TypedComponentInstanceProvider::primary_instance_typed::<dyn GraphQLSystem + Send + Sync>(&mut component_factory) {
        Ok(runtime) => runtime,
        Err(e) => {
            panic!("{}", e);
        }
    }
    // match ComponentFactoryBuilder::new() {
    //     Ok(component_factory) => {
    //         let mut component_factory = component_factory.build();
    //         match TypedComponentInstanceProvider::primary_instance_typed::<GraphQLSystemImpl>(&mut component_factory) {
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
