use std::time::Duration;

use async_trait::async_trait;
use inexor_rgf_behaviour_service_api::BehaviourSystem;
use springtime_di::injectable;
use tokio::time::error::Elapsed;

use inexor_rgf_command_api::CommandSystem;
use inexor_rgf_config_api::ConfigSystem;
use inexor_rgf_dynamic_graph_api::DynamicGraphSystem;
use inexor_rgf_graphql_api::GraphQLSystem;
use inexor_rgf_instance_system_api::InstanceSystem;
use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_plugin_graphql_api::PluginGraphQLSystem;
use inexor_rgf_plugin_service_api::PluginSystem;
use inexor_rgf_reactive_service_api::ReactiveSystem;
use inexor_rgf_remotes_api::RemotesSystem;
use inexor_rgf_remotes_model::InstanceAddress;
use inexor_rgf_runtime_graphql_api::RuntimeGraphQLSystem;
use inexor_rgf_runtime_service_api::RuntimeSystem;
use inexor_rgf_runtime_web_api::WebSystem;
use inexor_rgf_type_system_api::TypeSystem;

#[async_trait]
#[injectable]
pub trait Runtime:
    TypeSystem
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
