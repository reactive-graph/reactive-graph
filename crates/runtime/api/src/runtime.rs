use std::time::Duration;

use async_trait::async_trait;
use reactive_graph_behaviour_service_api::BehaviourSystem;
use springtime_di::injectable;
use tokio::time::error::Elapsed;

use reactive_graph_command_api::CommandSystem;
use reactive_graph_config_api::ConfigSystem;
use reactive_graph_dynamic_graph_api::DynamicGraphSystem;
use reactive_graph_graphql_api::GraphQLSystem;
use reactive_graph_instance_system_api::InstanceSystem;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_plugin_graphql_api::PluginGraphQLSystem;
use reactive_graph_plugin_service_api::PluginSystem;
use reactive_graph_reactive_service_api::ReactiveSystem;
use reactive_graph_remotes_api::RemotesSystem;
use reactive_graph_remotes_model::InstanceAddress;
use reactive_graph_runtime_graphql_api::RuntimeGraphQLSystem;
use reactive_graph_runtime_service_api::RuntimeSystem;
use reactive_graph_runtime_web_api::WebSystem;
use reactive_graph_type_system_api::TypeSystemSystem;

#[async_trait]
#[injectable]
pub trait Runtime:
    TypeSystemSystem
    + CommandSystem
    + ConfigSystem
    + GraphQLSystem
    + DynamicGraphSystem
    + RuntimeGraphQLSystem
    + PluginGraphQLSystem
    + RemotesSystem
    + PluginSystem
    + BehaviourSystem
    + InstanceSystem
    + ReactiveSystem
    + RuntimeSystem
    + WebSystem
    + Send
    + Sync
    + Lifecycle
{
    async fn config(&self);

    async fn run(&self);

    fn stop(&self);

    fn is_running(&self) -> bool;

    /// Waits for the GraphQL server to be started.
    /// Times out if the GraphQL server is not running after the given duration.
    async fn wait_for_started(&self, timeout_duration: Duration) -> Result<(), Elapsed>;

    /// Waits for the GraphQL server has been stopped.
    async fn wait_for_stopped(&self);

    /// Waits for the GraphQL server has been stopped.
    /// Times out if the GraphQL server is still running after the given duration.
    async fn wait_for_stopped_with_timeout(&self, timeout_duration: Duration) -> Result<(), Elapsed>;

    /// Returns the address of the runtime.
    fn address(&self) -> InstanceAddress;
}
