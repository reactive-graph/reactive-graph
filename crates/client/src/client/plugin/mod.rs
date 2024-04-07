#[cynic::schema_for_derives(file = r#"schema_plugin.graphql"#, module = "crate::schema_plugin::schema")]
pub mod mapping {
    use typed_builder::TypedBuilder;

    use crate::schema_plugin::plugin::Plugin;
    use crate::schema_plugin::plugin::PluginDependencies;
    use crate::schema_plugin::plugin::PluginDependents;
    use crate::schema_plugin::plugin::PluginUnsatisfiedDependencies;

    #[derive(cynic::QueryVariables, Debug)]
    pub struct PluginByNameVariables {
        pub name: String,
    }

    impl From<String> for PluginByNameVariables {
        fn from(name: String) -> Self {
            PluginByNameVariables { name }
        }
    }

    #[derive(cynic::QueryVariables, Debug, TypedBuilder)]
    pub struct SearchPluginVariables {
        pub name: Option<String>,
        pub state: Option<String>,
        pub stem: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct GetAllPlugins {
        pub plugins: Vec<Plugin>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "SearchPluginVariables")]
    pub struct SearchPlugins {
        #[arguments(name: $name, state: $state, stem: $stem)]
        pub plugins: Vec<Plugin>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "PluginByNameVariables")]
    pub struct GetPluginByName {
        #[arguments(name: $name)]
        pub plugins: Vec<Plugin>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "PluginByNameVariables")]
    pub struct GetDependencies {
        #[arguments(name: $name)]
        pub plugins: Vec<PluginDependencies>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Plugin", variables = "PluginByNameVariables")]
    pub struct GetDependenciesPlugin {
        pub dependencies: Vec<Plugin>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "PluginByNameVariables")]
    pub struct GetDependents {
        #[arguments(name: $name)]
        pub plugins: Vec<PluginDependents>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Plugin", variables = "PluginByNameVariables")]
    pub struct GetDependentsPlugin {
        pub dependents: Vec<Plugin>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "PluginByNameVariables")]
    pub struct GetUnsatisfiedDependencies {
        #[arguments(name: $name)]
        pub plugins: Vec<PluginUnsatisfiedDependencies>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Plugin", variables = "PluginByNameVariables")]
    pub struct GetUnsatisfiedDependenciesPlugin {
        pub unsatisfied_dependencies: Vec<Plugin>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "PluginByNameVariables")]
    pub struct StartPlugin {
        #[arguments(name: $name)]
        pub start: Plugin,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "PluginByNameVariables")]
    pub struct StopPlugin {
        #[arguments(name: $name)]
        pub stop: Plugin,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "PluginByNameVariables")]
    pub struct RestartPlugin {
        #[arguments(name: $name)]
        pub restart: Plugin,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "PluginByNameVariables")]
    pub struct UninstallPlugin {
        #[arguments(name: $name)]
        pub uninstall: bool,
    }
}

pub mod queries {
    use crate::client::plugin::mapping::GetAllPlugins;
    use crate::client::plugin::mapping::GetDependencies;
    use crate::client::plugin::mapping::GetDependents;
    use crate::client::plugin::mapping::GetPluginByName;
    use crate::client::plugin::mapping::GetUnsatisfiedDependencies;
    use crate::client::plugin::mapping::PluginByNameVariables;
    use crate::client::plugin::mapping::SearchPluginVariables;
    use crate::client::plugin::mapping::SearchPlugins;

    pub fn get_all() -> cynic::Operation<GetAllPlugins, ()> {
        use cynic::QueryBuilder;
        GetAllPlugins::build(())
    }

    pub fn search(vars: SearchPluginVariables) -> cynic::Operation<SearchPlugins, SearchPluginVariables> {
        use cynic::QueryBuilder;
        SearchPlugins::build(vars)
    }

    pub fn get_by_name(name: String) -> cynic::Operation<GetPluginByName, PluginByNameVariables> {
        use cynic::QueryBuilder;
        let vars: PluginByNameVariables = name.into();
        GetPluginByName::build(vars)
    }

    pub fn get_dependencies(name: String) -> cynic::Operation<GetDependencies, PluginByNameVariables> {
        use cynic::QueryBuilder;
        GetDependencies::build(name.into())
    }

    pub fn get_dependents(name: String) -> cynic::Operation<GetDependents, PluginByNameVariables> {
        use cynic::QueryBuilder;
        GetDependents::build(name.into())
    }

    pub fn get_unsatisfied_dependencies(name: String) -> cynic::Operation<GetUnsatisfiedDependencies, PluginByNameVariables> {
        use cynic::QueryBuilder;
        GetUnsatisfiedDependencies::build(name.into())
    }
}

pub mod operations {
    use crate::client::plugin::mapping::PluginByNameVariables;
    use crate::client::plugin::mapping::RestartPlugin;
    use crate::client::plugin::mapping::StartPlugin;
    use crate::client::plugin::mapping::StopPlugin;
    use crate::client::plugin::mapping::UninstallPlugin;

    pub fn start(name: String) -> cynic::Operation<StartPlugin, PluginByNameVariables> {
        use cynic::MutationBuilder;
        StartPlugin::build(name.into())
    }

    pub fn stop(name: String) -> cynic::Operation<StopPlugin, PluginByNameVariables> {
        use cynic::MutationBuilder;
        StopPlugin::build(name.into())
    }

    pub fn restart(name: String) -> cynic::Operation<RestartPlugin, PluginByNameVariables> {
        use cynic::MutationBuilder;
        RestartPlugin::build(name.into())
    }

    pub fn uninstall(name: String) -> cynic::Operation<UninstallPlugin, PluginByNameVariables> {
        use cynic::MutationBuilder;
        UninstallPlugin::build(name.into())
    }
}

pub mod api {
    use crate::client::plugin::mapping::SearchPluginVariables;
    use crate::client::plugin::operations::restart;
    use crate::client::plugin::operations::start;
    use crate::client::plugin::operations::stop;
    use crate::client::plugin::operations::uninstall;
    use crate::client::plugin::queries::get_all;
    use crate::client::plugin::queries::get_by_name;
    use crate::client::plugin::queries::get_dependencies;
    use crate::client::plugin::queries::get_dependents;
    use crate::client::plugin::queries::get_unsatisfied_dependencies;
    use crate::client::plugin::queries::search;
    use std::sync::Arc;

    use crate::schema_plugin::plugin::Plugin;
    use crate::InexorRgfClient;
    use crate::InexorRgfClientExecutionError;

    pub struct Plugins {
        client: Arc<InexorRgfClient>,
    }

    impl Plugins {
        pub fn new(client: Arc<InexorRgfClient>) -> Self {
            Self { client }
        }

        pub async fn get_all(&self) -> Result<Vec<Plugin>, InexorRgfClientExecutionError> {
            self.client.execute_plugins(get_all(), |data| data.plugins).await
        }

        pub async fn search(&self, vars: SearchPluginVariables) -> Result<Vec<Plugin>, InexorRgfClientExecutionError> {
            self.client.execute_plugins(search(vars), |data| data.plugins).await
        }

        /// Returns the plugin with the given name.
        /// If no plugin was found an empty optional will be returned.
        pub async fn get_by_name(&self, name: String) -> Result<Option<Plugin>, InexorRgfClientExecutionError> {
            self.client
                .execute_plugins(get_by_name(name), |data| data.plugins)
                .await
                .map(Plugins::get_first)
        }

        /// Returns the dependencies of the plugin with the given name.
        /// If no plugin was found an empty optional will be returned.
        pub async fn get_dependencies(&self, name: String) -> Result<Option<Vec<Plugin>>, InexorRgfClientExecutionError> {
            self.client
                .execute_plugins(get_dependencies(name), |data| data.plugins)
                .await
                .map(Plugins::get_first)
                .map(|plugin| plugin.map(|plugin| plugin.dependencies))
        }

        /// Returns the dependents of the plugin with the given name.
        /// If no plugin was found an empty optional will be returned.
        pub async fn get_dependents(&self, name: String) -> Result<Option<Vec<Plugin>>, InexorRgfClientExecutionError> {
            self.client
                .execute_plugins(get_dependents(name), |data| data.plugins)
                .await
                .map(Plugins::get_first)
                .map(|plugin| plugin.map(|plugin| plugin.dependents))
        }

        /// Returns the unsatisfied dependencies of the plugin with the given name.
        /// If no plugin was found an empty optional will be returned.
        pub async fn get_unsatisfied_dependencies(&self, name: String) -> Result<Option<Vec<Plugin>>, InexorRgfClientExecutionError> {
            self.client
                .execute_plugins(get_unsatisfied_dependencies(name), |data| data.plugins)
                .await
                .map(Plugins::get_first)
                .map(|plugin| plugin.map(|plugin| plugin.unsatisfied_dependencies))
        }

        pub async fn start(&self, name: String) -> Result<Plugin, InexorRgfClientExecutionError> {
            self.client.execute_plugins(start(name), |data| data.start).await
        }

        pub async fn stop(&self, name: String) -> Result<Plugin, InexorRgfClientExecutionError> {
            self.client.execute_plugins(stop(name), |data| data.stop).await
        }

        pub async fn restart(&self, name: String) -> Result<Plugin, InexorRgfClientExecutionError> {
            self.client.execute_plugins(restart(name), |data| data.restart).await
        }

        pub async fn uninstall(&self, name: String) -> Result<bool, InexorRgfClientExecutionError> {
            self.client.execute_plugins(uninstall(name), |data| data.uninstall).await
        }

        fn get_first<P: Clone>(plugins: Vec<P>) -> Option<P> {
            plugins.first().cloned()
        }
    }
}

#[cfg(test)]
pub mod test {

    use crate::InexorRgfClient;
    use reactive_graph_runtime_api::Runtime;
    use reactive_graph_runtime_impl::RuntimeBuilder;
    use std::sync::Arc;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_all_plugins() {
        RuntimeBuilder::new()
            .ignore_config_files()
            .disable_all_plugins(true)
            .pick_free_port()
            .init()
            .await
            .post_init()
            .await
            .spawn()
            .await
            .with_runtime(|runtime: Arc<dyn Runtime + Send + Sync>| async move {
                let plugin_container_manager = runtime.get_plugin_container_manager();
                assert_eq!(plugin_container_manager.get_plugins().len(), 0);

                // Client: Connect to self and get all remotes
                let client = InexorRgfClient::new(runtime.address()).expect("Cannot create client");
                let plugins = client.plugins().get_all().await.expect("Failed to get list of plugins");
                assert_eq!(plugins.len(), 0);
            })
            .await
            .stop()
            .await
            .pre_shutdown()
            .await
            .shutdown()
            .await;
    }
}
