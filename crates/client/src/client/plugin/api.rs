use crate::client::plugin::mutations::restart::mutations::restart;
use crate::client::plugin::mutations::start::mutations::start;
use crate::client::plugin::mutations::stop::mutations::stop;
use crate::client::plugin::mutations::uninstall::mutations::uninstall;
use crate::client::plugin::queries::get_all::queries::get_all;
use crate::client::plugin::queries::get_by_name::queries::get_by_name;
use crate::client::plugin::queries::get_dependencies::queries::get_dependencies;
use crate::client::plugin::queries::get_dependents::queries::get_dependents;
use crate::client::plugin::queries::get_unsatisfied_dependencies::queries::get_unsatisfied_dependencies;
use crate::client::plugin::queries::search::queries::search;
use crate::client::plugin::variables::search::variables::SearchPluginVariables;
use std::sync::Arc;

use crate::ReactiveGraphClient;
use crate::ReactiveGraphClientExecutionError;

use reactive_graph_plugin_model::Plugin;

pub struct Plugins {
    client: Arc<ReactiveGraphClient>,
}

impl Plugins {
    pub fn new(client: Arc<ReactiveGraphClient>) -> Self {
        Self { client }
    }

    pub async fn get_all(&self) -> Result<Vec<Plugin>, ReactiveGraphClientExecutionError> {
        self.client
            .execute_plugins(get_all(), |data| data.plugins.iter().map(|plugin| plugin.into()).collect())
            .await
    }

    pub async fn search(&self, vars: SearchPluginVariables) -> Result<Vec<Plugin>, ReactiveGraphClientExecutionError> {
        self.client
            .execute_plugins(search(vars), |data| data.plugins.iter().map(|plugin| plugin.into()).collect())
            .await
    }

    /// Returns the plugin with the given name.
    /// If no plugin was found an empty optional will be returned.
    pub async fn get_by_name(&self, name: String) -> Result<Option<Plugin>, ReactiveGraphClientExecutionError> {
        self.client
            .execute_plugins(get_by_name(name), |data| data.plugins.iter().map(|plugin| plugin.into()).collect())
            .await
            .map(Plugins::get_first)
    }

    /// Returns the dependencies of the plugin with the given name.
    /// If no plugin was found an empty optional will be returned.
    pub async fn get_dependencies(&self, name: String) -> Result<Option<Vec<Plugin>>, ReactiveGraphClientExecutionError> {
        self.client
            .execute_plugins(get_dependencies(name), |data| data.plugins)
            .await
            .map(Plugins::get_first)
            .map(|plugin| plugin.map(|plugin| plugin.dependencies))
            .map(|plugins| plugins.map(|plugins| plugins.iter().map(|plugin| plugin.into()).collect()))
    }

    /// Returns the dependents of the plugin with the given name.
    /// If no plugin was found an empty optional will be returned.
    pub async fn get_dependents(&self, name: String) -> Result<Option<Vec<Plugin>>, ReactiveGraphClientExecutionError> {
        self.client
            .execute_plugins(get_dependents(name), |data| data.plugins)
            .await
            .map(Plugins::get_first)
            .map(|plugin| plugin.map(|plugin| plugin.dependents))
            .map(|plugins| plugins.map(|plugins| plugins.iter().map(|plugin| plugin.into()).collect()))
    }

    /// Returns the unsatisfied dependencies of the plugin with the given name.
    /// If no plugin was found an empty optional will be returned.
    pub async fn get_unsatisfied_dependencies(&self, name: String) -> Result<Option<Vec<Plugin>>, ReactiveGraphClientExecutionError> {
        self.client
            .execute_plugins(get_unsatisfied_dependencies(name), |data| data.plugins)
            .await
            .map(Plugins::get_first)
            .map(|plugin| plugin.map(|plugin| plugin.unsatisfied_dependencies))
            .map(|plugins| plugins.map(|plugins| plugins.iter().map(|plugin| plugin.into()).collect()))
    }

    pub async fn start(&self, name: String) -> Result<Plugin, ReactiveGraphClientExecutionError> {
        self.client.execute_plugins(start(name), |data| (&data.start).into()).await
    }

    pub async fn stop(&self, name: String) -> Result<Plugin, ReactiveGraphClientExecutionError> {
        self.client.execute_plugins(stop(name), |data| (&data.stop).into()).await
    }

    pub async fn restart(&self, name: String) -> Result<Plugin, ReactiveGraphClientExecutionError> {
        self.client.execute_plugins(restart(name), |data| (&data.restart).into()).await
    }

    pub async fn uninstall(&self, name: String) -> Result<bool, ReactiveGraphClientExecutionError> {
        self.client.execute_plugins(uninstall(name), |data| data.uninstall).await
    }

    fn get_first<P: Clone>(plugins: Vec<P>) -> Option<P> {
        plugins.first().cloned()
    }
}
