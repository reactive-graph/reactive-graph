use std::sync::Arc;

use async_graphql::EmptySubscription;
use async_graphql::Schema;
use async_trait::async_trait;
use springtime_di::component_alias;
use springtime_di::Component;

use reactive_graph_command_api::CommandManager;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_remotes_api::InstanceService;
use reactive_graph_remotes_api::RemotesManager;
use reactive_graph_runtime_graphql_api::RuntimeSchemaManager;
use reactive_graph_runtime_graphql_schema::mutation::RuntimeMutation;
use reactive_graph_runtime_graphql_schema::query::RuntimeQuery;
use reactive_graph_runtime_graphql_schema::RuntimeSchema;
use reactive_graph_runtime_service_api::ShutdownManager;

#[derive(Component)]
pub struct RuntimeSchemaManagerImpl {
    instance_service: Arc<dyn InstanceService + Send + Sync>,

    remotes_manager: Arc<dyn RemotesManager + Send + Sync>,

    command_manager: Arc<dyn CommandManager + Send + Sync>,

    shutdown_manager: Arc<dyn ShutdownManager + Send + Sync>,
}

impl RuntimeSchemaManagerImpl {}

#[async_trait]
#[component_alias]
impl RuntimeSchemaManager for RuntimeSchemaManagerImpl {
    fn get_schema(&self) -> RuntimeSchema {
        Schema::build(RuntimeQuery, RuntimeMutation, EmptySubscription)
            .data(self.instance_service.clone())
            .data(self.remotes_manager.clone())
            .data(self.command_manager.clone())
            .data(self.shutdown_manager.clone())
            .finish()
    }
}

#[async_trait]
impl Lifecycle for RuntimeSchemaManagerImpl {}
