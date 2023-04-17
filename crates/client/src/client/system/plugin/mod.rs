#[cynic::schema_for_derives(file = r#"schema.graphql"#, module = "crate::schema::schema")]
pub mod mapping {
    use typed_builder::TypedBuilder;

    use crate::schema::plugin::Plugin;
    use crate::schema::plugin::PluginDependencies;
    use crate::schema::plugin::PluginDependents;

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
}

pub mod queries {
    use crate::system::plugin::mapping::GetAllPlugins;
    use crate::system::plugin::mapping::GetDependencies;
    use crate::system::plugin::mapping::GetDependents;
    use crate::system::plugin::mapping::GetPluginByName;
    use crate::system::plugin::mapping::PluginByNameVariables;
    use crate::system::plugin::mapping::SearchPluginVariables;
    use crate::system::plugin::mapping::SearchPlugins;

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
}

pub mod operations {
    use crate::system::plugin::mapping::PluginByNameVariables;
    use crate::system::plugin::mapping::RestartPlugin;
    use crate::system::plugin::mapping::StartPlugin;
    use crate::system::plugin::mapping::StopPlugin;

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
}

pub mod api {
    use std::sync::Arc;

    use cynic::http::ReqwestExt;
    use cynic::Operation;

    use crate::schema::plugin::Plugin;
    use crate::system::plugin::mapping::SearchPluginVariables;
    use crate::system::plugin::operations::restart;
    use crate::system::plugin::operations::start;
    use crate::system::plugin::operations::stop;
    use crate::system::plugin::queries::get_all;
    use crate::system::plugin::queries::get_by_name;
    use crate::system::plugin::queries::get_dependencies;
    use crate::system::plugin::queries::get_dependents;
    use crate::system::plugin::queries::search;
    use crate::InexorRgfClient;
    use crate::InexorRgfClientExecutionError;

    pub struct Plugins {
        client: Arc<InexorRgfClient>,
    }

    impl Plugins {
        pub fn new(client: Arc<InexorRgfClient>) -> Self {
            Self { client }
        }

        pub async fn plugin_operation<ResponseData, Vars, ResponseType>(
            &self,
            operation: Operation<ResponseData, Vars>,
            extractor: impl FnOnce(ResponseData) -> ResponseType,
        ) -> Result<Option<ResponseType>, InexorRgfClientExecutionError>
        where
            Vars: serde::Serialize,
            ResponseData: serde::de::DeserializeOwned + 'static,
        {
            let value = self
                .client
                .client
                .post(self.client.url())
                .run_graphql(operation)
                .await?
                // .map_err(InexorRgfClientExecutionError::from)?
                .data
                .map(extractor);
            Ok(value)
        }

        pub async fn get_all(&self) -> Result<Option<Vec<Plugin>>, InexorRgfClientExecutionError> {
            return self.plugin_operation(get_all(), |data| data.system.plugins).await;
        }

        pub async fn search(&self, vars: SearchPluginVariables) -> Result<Option<Vec<Plugin>>, InexorRgfClientExecutionError> {
            return self.plugin_operation(search(vars), |data| data.system.plugins).await;
        }

        pub async fn get_by_name(&self, name: String) -> Result<Option<Plugin>, InexorRgfClientExecutionError> {
            self.plugin_operation(get_by_name(name), |data| data.system.plugins)
                .await
                .map(Plugins::get_first)
        }

        pub async fn get_dependencies(&self, name: String) -> Result<Option<Vec<Plugin>>, InexorRgfClientExecutionError> {
            return self
                .plugin_operation(get_dependencies(name), |data| data.system.plugins)
                .await
                .map(Plugins::get_first)
                .map(|plugin| plugin.map(|plugin| plugin.dependencies));
        }

        pub async fn get_dependents(&self, name: String) -> Result<Option<Vec<Plugin>>, InexorRgfClientExecutionError> {
            return self
                .plugin_operation(get_dependents(name), |data| data.system.plugins)
                .await
                .map(Plugins::get_first)
                .map(|plugin| plugin.map(|plugin| plugin.dependents));
        }

        pub async fn start(&self, name: String) -> Result<Option<Plugin>, InexorRgfClientExecutionError> {
            return self.plugin_operation(start(name), |data| data.system.plugins.start).await;
        }

        pub async fn stop(&self, name: String) -> Result<Option<Plugin>, InexorRgfClientExecutionError> {
            return self.plugin_operation(stop(name), |data| data.system.plugins.stop).await;
        }

        pub async fn restart(&self, name: String) -> Result<Option<Plugin>, InexorRgfClientExecutionError> {
            return self.plugin_operation(restart(name), |data| data.system.plugins.restart).await;
        }

        fn get_first<P: Clone>(plugins: Option<Vec<P>>) -> Option<P> {
            plugins.and_then(|plugins| plugins.first().cloned())
        }
    }
}
