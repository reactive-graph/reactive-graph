#[cynic::schema_for_derives(file = r#"schema.graphql"#, module = "crate::schema::schema")]
pub mod mapping {
    use typed_builder::TypedBuilder;

    use crate::schema::system::plugin::Plugin;
    use crate::schema::system::plugin::PluginDependencies;
    use crate::schema::system::plugin::PluginDependents;
    use crate::schema::system::plugin::PluginUnsatisfiedDependencies;

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
        pub system: GetAllPluginsSystem,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "System")]
    pub struct GetAllPluginsSystem {
        pub plugins: Vec<Plugin>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "SearchPluginVariables")]
    pub struct SearchPlugins {
        pub system: SearchPluginsSystem,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "System", variables = "SearchPluginVariables")]
    pub struct SearchPluginsSystem {
        #[arguments(name: $name, state: $state, stem: $stem)]
        pub plugins: Vec<Plugin>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "PluginByNameVariables")]
    pub struct GetPluginByName {
        pub system: GetPluginByNameSystem,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "System", variables = "PluginByNameVariables")]
    pub struct GetPluginByNameSystem {
        #[arguments(name: $name)]
        pub plugins: Vec<Plugin>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "PluginByNameVariables")]
    pub struct GetDependencies {
        pub system: GetDependenciesSystem,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "System", variables = "PluginByNameVariables")]
    pub struct GetDependenciesSystem {
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
        pub system: GetDependentsSystem,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "System", variables = "PluginByNameVariables")]
    pub struct GetDependentsSystem {
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
        pub system: GetUnsatisfiedDependenciesSystem,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "System", variables = "PluginByNameVariables")]
    pub struct GetUnsatisfiedDependenciesSystem {
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
        pub system: StartPluginMutationSystem,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "MutationSystem", variables = "PluginByNameVariables")]
    pub struct StartPluginMutationSystem {
        pub plugins: StartPluginMutationPlugins,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "MutationPlugins", variables = "PluginByNameVariables")]
    pub struct StartPluginMutationPlugins {
        #[arguments(name: $name)]
        pub start: Plugin,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "PluginByNameVariables")]
    pub struct StopPlugin {
        pub system: StopPluginMutationSystem,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "MutationSystem", variables = "PluginByNameVariables")]
    pub struct StopPluginMutationSystem {
        pub plugins: StopPluginMutationPlugins,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "MutationPlugins", variables = "PluginByNameVariables")]
    pub struct StopPluginMutationPlugins {
        #[arguments(name: $name)]
        pub stop: Plugin,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "PluginByNameVariables")]
    pub struct RestartPlugin {
        pub system: RestartPluginMutationSystem,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "MutationSystem", variables = "PluginByNameVariables")]
    pub struct RestartPluginMutationSystem {
        pub plugins: RestartPluginMutationPlugins,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "MutationPlugins", variables = "PluginByNameVariables")]
    pub struct RestartPluginMutationPlugins {
        #[arguments(name: $name)]
        pub restart: Plugin,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "PluginByNameVariables")]
    pub struct UninstallPlugin {
        pub system: UninstallPluginMutationSystem,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "MutationSystem", variables = "PluginByNameVariables")]
    pub struct UninstallPluginMutationSystem {
        pub plugins: UninstallPluginMutationPlugins,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "MutationPlugins", variables = "PluginByNameVariables")]
    pub struct UninstallPluginMutationPlugins {
        #[arguments(name: $name)]
        pub uninstall: bool,
    }
}

pub mod queries {
    use crate::client::system::plugin::mapping::GetAllPlugins;
    use crate::client::system::plugin::mapping::GetDependencies;
    use crate::client::system::plugin::mapping::GetDependents;
    use crate::client::system::plugin::mapping::GetPluginByName;
    use crate::client::system::plugin::mapping::GetUnsatisfiedDependencies;
    use crate::client::system::plugin::mapping::PluginByNameVariables;
    use crate::client::system::plugin::mapping::SearchPluginVariables;
    use crate::client::system::plugin::mapping::SearchPlugins;

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
    use crate::client::system::plugin::mapping::PluginByNameVariables;
    use crate::client::system::plugin::mapping::RestartPlugin;
    use crate::client::system::plugin::mapping::StartPlugin;
    use crate::client::system::plugin::mapping::StopPlugin;
    use crate::client::system::plugin::mapping::UninstallPlugin;

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
    use std::sync::Arc;

    use crate::client::system::plugin::mapping::SearchPluginVariables;
    use crate::client::system::plugin::operations::restart;
    use crate::client::system::plugin::operations::start;
    use crate::client::system::plugin::operations::stop;
    use crate::client::system::plugin::operations::uninstall;
    use crate::client::system::plugin::queries::get_all;
    use crate::client::system::plugin::queries::get_by_name;
    use crate::client::system::plugin::queries::get_dependencies;
    use crate::client::system::plugin::queries::get_dependents;
    use crate::client::system::plugin::queries::search;
    use crate::schema::system::plugin::Plugin;
    use crate::system::plugin::queries::get_unsatisfied_dependencies;
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
            self.client.run_graphql(get_all(), |data| data.system.plugins).await
        }

        pub async fn search(&self, vars: SearchPluginVariables) -> Result<Vec<Plugin>, InexorRgfClientExecutionError> {
            self.client.run_graphql(search(vars), |data| data.system.plugins).await
        }

        /// Returns the plugin with the given name.
        /// If no plugin was found an empty optional will be returned.
        pub async fn get_by_name(&self, name: String) -> Result<Option<Plugin>, InexorRgfClientExecutionError> {
            self.client
                .run_graphql(get_by_name(name), |data| data.system.plugins)
                .await
                .map(Plugins::get_first)
        }

        /// Returns the dependencies of the plugin with the given name.
        /// If no plugin was found an empty optional will be returned.
        pub async fn get_dependencies(&self, name: String) -> Result<Option<Vec<Plugin>>, InexorRgfClientExecutionError> {
            self.client
                .run_graphql(get_dependencies(name), |data| data.system.plugins)
                .await
                .map(Plugins::get_first)
                .map(|plugin| plugin.map(|plugin| plugin.dependencies))
        }

        /// Returns the dependents of the plugin with the given name.
        /// If no plugin was found an empty optional will be returned.
        pub async fn get_dependents(&self, name: String) -> Result<Option<Vec<Plugin>>, InexorRgfClientExecutionError> {
            self.client
                .run_graphql(get_dependents(name), |data| data.system.plugins)
                .await
                .map(Plugins::get_first)
                .map(|plugin| plugin.map(|plugin| plugin.dependents))
        }

        /// Returns the unsatisfied dependencies of the plugin with the given name.
        /// If no plugin was found an empty optional will be returned.
        pub async fn get_unsatisfied_dependencies(&self, name: String) -> Result<Option<Vec<Plugin>>, InexorRgfClientExecutionError> {
            self.client
                .run_graphql(get_unsatisfied_dependencies(name), |data| data.system.plugins)
                .await
                .map(Plugins::get_first)
                .map(|plugin| plugin.map(|plugin| plugin.unsatisfied_dependencies))
        }

        pub async fn start(&self, name: String) -> Result<Plugin, InexorRgfClientExecutionError> {
            self.client.run_graphql(start(name), |data| data.system.plugins.start).await
        }

        pub async fn stop(&self, name: String) -> Result<Plugin, InexorRgfClientExecutionError> {
            self.client.run_graphql(stop(name), |data| data.system.plugins.stop).await
        }

        pub async fn restart(&self, name: String) -> Result<Plugin, InexorRgfClientExecutionError> {
            self.client.run_graphql(restart(name), |data| data.system.plugins.restart).await
        }

        pub async fn uninstall(&self, name: String) -> Result<bool, InexorRgfClientExecutionError> {
            self.client.run_graphql(uninstall(name), |data| data.system.plugins.uninstall).await
        }

        fn get_first<P: Clone>(plugins: Vec<P>) -> Option<P> {
            plugins.first().cloned()
        }
    }
}

#[cfg(test)]
pub mod test {

    use crate::InexorRgfClient;
    use inexor_rgf_rt::runtime::Runtime;
    use inexor_rgf_rt::runtime::RuntimeBuilder;
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
            .with_runtime(|runtime: Arc<dyn Runtime>| async move {
                let plugin_container_manager = runtime.get_plugin_container_manager();
                assert_eq!(plugin_container_manager.get_plugins().len(), 0);

                // Client: Connect to self and get all remotes
                let client = InexorRgfClient::new(runtime.address()).expect("Cannot create client");
                let plugins = client.system().plugins().get_all().await.expect("Failed to get list of plugins");
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
