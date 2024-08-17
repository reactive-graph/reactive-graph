use async_graphql::*;
use reactive_graph_plugin_api::PluginState;
use reactive_graph_plugin_service_api::PluginContainerManager;
use std::sync::Arc;
use uuid::Uuid;

pub use plugin::*;

pub mod plugin;

pub struct PluginQuery;

/// Search queries for the type system, the instances and the flows.
#[Object(name = "Query")]
impl PluginQuery {
    #[allow(clippy::too_many_arguments)]
    async fn plugins(
        &self,
        context: &Context<'_>,
        id: Option<Uuid>,
        stem: Option<String>,
        name: Option<String>,
        state: Option<String>,
        has_dependencies: Option<bool>,
        has_unsatisfied_dependencies: Option<bool>,
    ) -> Result<Vec<GraphQLPlugin>> {
        let plugin_container_manager = context.data::<Arc<dyn PluginContainerManager + Send + Sync>>()?;
        let plugins = plugin_container_manager
            .get_plugins()
            .into_iter()
            .filter(|plugin_id| match &id {
                Some(id) => plugin_id == id,
                None => true,
            })
            .filter(|plugin_id| match &stem {
                Some(stem) => match plugin_container_manager.get_id(stem.as_ref()) {
                    Some(id) => plugin_id == &id,
                    None => false,
                },
                None => true,
            })
            .filter(|plugin_id| match &name {
                Some(name) => match plugin_container_manager.name(plugin_id) {
                    Some(plugin_name) => &plugin_name == name,
                    None => false,
                },
                None => true,
            })
            .filter(|plugin_id| match &state {
                Some(state) => match plugin_container_manager.get_plugin_state(plugin_id) {
                    Some(PluginState::Installed) => state == "Installed",
                    Some(PluginState::Resolving(_)) => state == "Resolving",
                    Some(PluginState::Resolved) => state == "Resolved",
                    Some(PluginState::Starting(_)) => state == "Starting",
                    Some(PluginState::Active) => state == "Active",
                    Some(PluginState::Stopping(_)) => state == "Stopping",
                    Some(PluginState::Refreshing(_)) => state == "Refreshing",
                    Some(PluginState::Uninstalling(_)) => state == "Uninstalling",
                    Some(PluginState::Uninstalled) => state == "Uninstalled",
                    Some(PluginState::Disabled) => state == "Disabled",
                    None => false,
                },
                None => true,
            })
            .filter(|plugin_id| match &has_dependencies {
                Some(true) => plugin_container_manager.has_dependencies(plugin_id),
                Some(false) => !plugin_container_manager.has_dependencies(plugin_id),
                None => true,
            })
            .filter(|plugin_id| match &has_unsatisfied_dependencies {
                Some(true) => plugin_container_manager.has_unsatisfied_dependencies(plugin_id),
                Some(false) => !plugin_container_manager.has_unsatisfied_dependencies(plugin_id),
                None => true,
            })
            .map(|id| GraphQLPlugin { id })
            .collect();
        Ok(plugins)
    }
}

// #[cfg(test)]
// mod tests {
//     use std::sync::Arc;
//
//     use serde::Deserialize;
//     use uuid::Uuid;
//
//     use reactive_graph_graphql_api::GraphQLQueryService;
//     use reactive_graph_plugin_api::serde_json;
//     use reactive_graph_runtime_api::Runtime;
//     use reactive_graph_runtime_impl::RuntimeBuilder;
//
//     #[derive(Deserialize, Debug)]
//     #[serde(rename_all = "camelCase")]
//     struct Plugin {
//         id: Uuid,
//     }
//
//     #[derive(Deserialize, Debug)]
//     #[serde(rename_all = "camelCase")]
//     struct System {
//         plugins: Vec<Plugin>,
//     }
//
//     #[derive(Deserialize, Debug)]
//     struct Data {
//         system: System,
//     }
//
//     #[tokio::test(flavor = "multi_thread")]
//     async fn test_get_all_plugins() {
//         RuntimeBuilder::new()
//             .ignore_config_files()
//             .disable_all_plugins(true)
//             .pick_free_port()
//             .init()
//             .await
//             .post_init()
//             .await
//             .spawn()
//             .await
//             .with_runtime(|runtime: Arc<dyn Runtime + Send + Sync>| async move {
//                 let query_service = runtime.get_graphql_query_service();
//                 let plugin_container_manager = runtime.get_plugin_container_manager();
//                 let rt_plugins = plugin_container_manager.get_plugins();
//                 let gql_plugins: Vec<Uuid> = query_get_all_plugins(&query_service).await.iter().map(|plugin| plugin.id).collect();
//                 assert_eq!(rt_plugins, gql_plugins);
//             })
//             .await
//             .stop()
//             .await
//             .pre_shutdown()
//             .await
//             .shutdown()
//             .await;
//     }
//
//     const QUERY_GET_ALL_PLUGIN_IDS: &str = include_str!("../../graphql/get_all_ids.graphql");
//
//     async fn query_get_all_plugins(query_service: &Arc<dyn GraphQLQueryService + Send + Sync>) -> Vec<Plugin> {
//         let response = query_service.query_response(QUERY_GET_ALL_PLUGIN_IDS).await;
//         assert!(response.errors.is_empty());
//         let data = response.data.into_json().expect("Failed to get json data from graphql response");
//         let data: Data = serde_json::from_value(data).expect("Failed to deserialize json into target data model");
//         data.system.plugins
//     }
// }
