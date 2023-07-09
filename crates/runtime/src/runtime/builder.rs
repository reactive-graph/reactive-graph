use std::future::Future;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::Arc;

use tokio::time::Duration;

use crate::get_runtime;
use crate::model_runtime::InstanceAddress;
use crate::runtime::Runtime;

pub enum SetConfigLocations {}
pub enum ConfigFilesLoaded {}

pub enum NotRunning {}
pub enum Initialized {}
pub enum Ready {}
pub enum Running {}
pub enum Finished {}
pub enum PreShutdown {}
pub enum Shutdown {}

pub struct RuntimeBuilder<L, R> {
    runtime: Arc<dyn Runtime>,

    typestate: PhantomData<(L, R)>,
}

impl RuntimeBuilder<SetConfigLocations, NotRunning> {
    pub fn new() -> Self {
        Self {
            runtime: get_runtime(),
            typestate: PhantomData,
        }
    }

    /// Sets the location of the instance configuration.
    pub fn instance_config<P: Into<OptionOption<PathBuf>>>(self, location: P) -> RuntimeBuilder<SetConfigLocations, NotRunning> {
        if let Some(location) = location.into().get() {
            self.runtime.get_config_manager().set_instance_config_location(location);
        }
        self
    }

    /// Sets the location of the graphql server configuration.
    pub fn graphql_server_config<P: Into<OptionOption<PathBuf>>>(self, location: P) -> RuntimeBuilder<SetConfigLocations, NotRunning> {
        if let Some(location) = location.into().get() {
            self.runtime.get_config_manager().set_graphql_server_config_location(location);
        }
        self
    }

    /// Sets the location of the plugins configuration.
    pub fn plugins_config<P: Into<OptionOption<PathBuf>>>(self, location: P) -> RuntimeBuilder<SetConfigLocations, NotRunning> {
        if let Some(location) = location.into().get() {
            self.runtime.get_config_manager().set_plugins_config_location(location);
        }
        self
    }

    /// Loads the config files and transits to state `ConfigFilesLoaded`.
    pub async fn load_config_files(self) -> RuntimeBuilder<ConfigFilesLoaded, NotRunning> {
        self.runtime.config().await;
        RuntimeBuilder {
            runtime: self.runtime,
            typestate: PhantomData,
        }
    }

    /// Doesn't load the config files. Transits to state `ConfigFilesLoaded`.
    pub fn ignore_config_files(self) -> RuntimeBuilder<ConfigFilesLoaded, NotRunning> {
        RuntimeBuilder {
            runtime: self.runtime,
            typestate: PhantomData,
        }
    }
}

impl RuntimeBuilder<ConfigFilesLoaded, NotRunning> {
    /// Sets the name of the instance.
    pub fn instance_name<S: Into<OptionOption<String>>>(self, name: S) -> RuntimeBuilder<ConfigFilesLoaded, NotRunning> {
        if let Some(name) = name.into().get() {
            self.runtime.get_config_manager().set_instance_name(&name);
        }
        self
    }

    /// Sets the description of the instance.
    pub fn instance_description<S: Into<OptionOption<String>>>(self, description: S) -> RuntimeBuilder<ConfigFilesLoaded, NotRunning> {
        if let Some(description) = description.into().get() {
            self.runtime.get_config_manager().set_instance_description(&description);
        }
        self
    }

    /// Sets the hostname of the GraphQL server.
    pub fn hostname<S: Into<OptionOption<String>>>(self, hostname: S) -> RuntimeBuilder<ConfigFilesLoaded, NotRunning> {
        if let Some(hostname) = hostname.into().get() {
            self.runtime.get_config_manager().set_graphql_hostname(&hostname);
        }
        self
    }

    /// Sets the port number of the GraphQL server.
    pub fn port<S: Into<OptionOption<u16>>>(self, port: S) -> RuntimeBuilder<ConfigFilesLoaded, NotRunning> {
        if let Some(port) = port.into().get() {
            self.runtime.get_config_manager().set_graphql_port(port);
        }
        self
    }

    /// Picks a free port instead of a fixed port number.
    pub fn pick_free_port(self) -> RuntimeBuilder<ConfigFilesLoaded, NotRunning> {
        if let Some(port) = portpicker::pick_unused_port() {
            self.runtime.get_config_manager().set_graphql_port(port);
        }
        self
    }

    /// Sets if the GraphQL server shall use https or http.
    pub fn secure<S: Into<OptionOption<bool>>>(self, secure: S) -> RuntimeBuilder<ConfigFilesLoaded, NotRunning> {
        if let Some(secure) = secure.into().get() {
            self.runtime.get_config_manager().set_graphql_secure(secure);
        }
        self
    }

    /// Sets the hostname, port and secure of the GraphQL server.
    pub fn address(self, address: &InstanceAddress) -> RuntimeBuilder<ConfigFilesLoaded, NotRunning> {
        let config_manager = self.runtime.get_config_manager();
        config_manager.set_graphql_hostname(&address.hostname);
        config_manager.set_graphql_port(address.port);
        config_manager.set_graphql_secure(address.secure);
        self
    }

    /// Sets timeout of the shutdown of the GraphQL server.
    pub fn shutdown_timeout<S: Into<OptionOption<u64>>>(self, shutdown_timeout: S) -> RuntimeBuilder<ConfigFilesLoaded, NotRunning> {
        if let Some(shutdown_timeout) = shutdown_timeout.into().get() {
            self.runtime.get_config_manager().set_graphql_shutdown_timeout(shutdown_timeout);
        }
        self
    }

    /// Sets the number of workers of the GraphQL server.
    pub fn workers<S: Into<OptionOption<usize>>>(self, workers: S) -> RuntimeBuilder<ConfigFilesLoaded, NotRunning> {
        if let Some(workers) = workers.into().get() {
            self.runtime.get_config_manager().set_graphql_workers(workers);
        }
        self
    }

    /// Sets context path of a web resource provider which shall be used as default context path.
    pub fn default_context_path<S: Into<OptionOption<String>>>(self, default_context_path: S) -> RuntimeBuilder<ConfigFilesLoaded, NotRunning> {
        if let Some(default_context_path) = default_context_path.into().get() {
            self.runtime.get_config_manager().set_graphql_default_context_path(default_context_path);
        }
        self
    }

    /// Disables all plugins.
    pub fn disable_all_plugins<S: Into<OptionOption<bool>>>(self, disabled: S) -> RuntimeBuilder<ConfigFilesLoaded, NotRunning> {
        if let Some(disabled) = disabled.into().get() {
            self.runtime.get_config_manager().set_disable_all_plugins(disabled);
        }
        self
    }

    /// Sets which plugins will be disabled.
    pub fn disabled_plugins<S: Into<OptionOption<Vec<String>>>>(self, disabled_plugins: S) -> RuntimeBuilder<ConfigFilesLoaded, NotRunning> {
        if let Some(disabled_plugins) = disabled_plugins.into().get() {
            self.runtime.get_config_manager().set_disabled_plugins(disabled_plugins);
        }
        self
    }

    /// Disables hot deployment of plugins.
    pub fn disable_hot_deploy<S: Into<OptionOption<bool>>>(self, disabled: S) -> RuntimeBuilder<ConfigFilesLoaded, NotRunning> {
        if let Some(disabled) = disabled.into().get() {
            self.runtime.get_config_manager().set_disable_hot_deploy(disabled);
        }
        self
    }

    /// Sets the directory where plugins can be hot deployed.
    pub fn hot_deploy_location<S: Into<OptionOption<String>>>(self, hot_deploy_location: S) -> RuntimeBuilder<ConfigFilesLoaded, NotRunning> {
        if let Some(hot_deploy_location) = hot_deploy_location.into().get() {
            self.runtime.get_config_manager().set_hot_deploy_location(Some(hot_deploy_location))
        }
        self
    }

    /// Sets the directory where plugins are located.
    /// During hot deployment new plugins will be moved into this directory.
    pub fn install_location<S: Into<OptionOption<String>>>(self, install_location: S) -> RuntimeBuilder<ConfigFilesLoaded, NotRunning> {
        if let Some(install_location) = install_location.into().get() {
            self.runtime.get_config_manager().set_install_location(Some(install_location))
        }
        self
    }

    pub fn get(self) -> Arc<dyn Runtime> {
        self.runtime
    }

    pub async fn init(self) -> RuntimeBuilder<ConfigFilesLoaded, Initialized> {
        self.runtime.init().await;
        RuntimeBuilder {
            runtime: self.runtime,
            typestate: PhantomData,
        }
    }

    pub async fn block_on(self) -> RuntimeBuilder<ConfigFilesLoaded, Shutdown> {
        {
            self.runtime.init().await;
            self.runtime.post_init().await;
            self.runtime.run().await;
            self.runtime.pre_shutdown().await;
            self.runtime.shutdown().await;
        }
        RuntimeBuilder {
            runtime: self.runtime,
            typestate: PhantomData,
        }
    }
}

impl RuntimeBuilder<ConfigFilesLoaded, Initialized> {
    pub async fn post_init(self) -> RuntimeBuilder<ConfigFilesLoaded, Ready> {
        self.runtime.post_init().await;
        RuntimeBuilder {
            runtime: self.runtime,
            typestate: PhantomData,
        }
    }
}

impl RuntimeBuilder<ConfigFilesLoaded, Ready> {
    pub fn get(self) -> Arc<dyn Runtime> {
        self.runtime
    }

    pub async fn with_runtime<F, C>(self, f: C) -> RuntimeBuilder<ConfigFilesLoaded, Ready>
    where
        F: Future<Output = ()>,
        C: FnOnce(Arc<dyn Runtime>) -> F,
    {
        let runtime = self.runtime.clone();
        f(runtime).await;
        self
    }

    /// Starts the GraphQL server (non-blocking).
    pub async fn spawn(self) -> RuntimeBuilder<ConfigFilesLoaded, Running> {
        let runtime_inner = self.runtime.clone();
        tokio::task::spawn(async move {
            runtime_inner.run().await;
        });
        let _ = self.runtime.wait_for_started(Duration::from_secs(5)).await;
        RuntimeBuilder {
            runtime: self.runtime,
            typestate: PhantomData,
        }
    }

    /// Starts the GraphQL server (blocking).
    pub async fn spawn_blocking(self) -> RuntimeBuilder<ConfigFilesLoaded, Finished> {
        self.runtime.run().await;
        RuntimeBuilder {
            runtime: self.runtime,
            typestate: PhantomData,
        }
    }

    /// Runs starts the GraphQL server. Stops the GraphQL server after the given duration.
    pub async fn run_for(self, duration: Duration) -> RuntimeBuilder<ConfigFilesLoaded, Finished> {
        let inner_runtime = self.runtime.clone();
        tokio::spawn(async move {
            tokio::time::sleep(duration).await;
            inner_runtime.get_shutdown_manager().do_shutdown();
        });
        self.runtime.run().await;
        RuntimeBuilder {
            runtime: self.runtime,
            typestate: PhantomData,
        }
    }

    /// Do not start the GraphQL server but shutdown the runtime.
    pub async fn do_not_run(self) -> RuntimeBuilder<ConfigFilesLoaded, Finished> {
        RuntimeBuilder {
            runtime: self.runtime,
            typestate: PhantomData,
        }
    }
}

impl RuntimeBuilder<ConfigFilesLoaded, Running> {
    pub fn get(self) -> Arc<dyn Runtime> {
        self.runtime
    }

    pub async fn with_runtime<F, C>(self, f: C) -> RuntimeBuilder<ConfigFilesLoaded, Running>
    where
        F: Future<Output = ()>,
        C: FnOnce(Arc<dyn Runtime>) -> F,
    {
        let runtime = self.runtime.clone();
        f(runtime).await;
        self
    }

    /// Stops the runtime. Waits for the GraphQL server has been stopped.
    pub async fn stop(self) -> RuntimeBuilder<ConfigFilesLoaded, Finished> {
        self.runtime.stop();
        self.runtime.wait_for_stopped().await;
        RuntimeBuilder {
            runtime: self.runtime,
            typestate: PhantomData,
        }
    }

    /// Stops the runtime. Waits for the GraphQL server has been stopped or the given timeout has
    /// been reached.
    pub async fn stop_with_timeout(self, timeout_duration: Duration) -> RuntimeBuilder<ConfigFilesLoaded, Finished> {
        self.runtime.stop();
        let _ = self.runtime.wait_for_stopped_with_timeout(timeout_duration).await;
        RuntimeBuilder {
            runtime: self.runtime,
            typestate: PhantomData,
        }
    }

    /// Waits for the GraphQL server has been stopped.
    pub async fn wait_for_stopped(self) -> RuntimeBuilder<ConfigFilesLoaded, Finished> {
        self.runtime.wait_for_stopped().await;
        RuntimeBuilder {
            runtime: self.runtime,
            typestate: PhantomData,
        }
    }

    /// Waits for the GraphQL server has been stopped.
    pub async fn wait_for_stopped_with_timeout(self, timeout_duration: Duration) -> RuntimeBuilder<ConfigFilesLoaded, Finished> {
        let _ = self.runtime.wait_for_stopped_with_timeout(timeout_duration).await;
        RuntimeBuilder {
            runtime: self.runtime,
            typestate: PhantomData,
        }
    }
}

impl RuntimeBuilder<ConfigFilesLoaded, Finished> {
    pub fn get(self) -> Arc<dyn Runtime> {
        self.runtime
    }

    pub async fn with_runtime<F, C>(self, f: C) -> RuntimeBuilder<ConfigFilesLoaded, Finished>
    where
        F: Future<Output = ()>,
        C: FnOnce(Arc<dyn Runtime>) -> F,
    {
        let runtime = self.runtime.clone();
        f(runtime).await;
        self
    }

    pub async fn pre_shutdown(self) -> RuntimeBuilder<ConfigFilesLoaded, PreShutdown> {
        self.runtime.pre_shutdown().await;
        RuntimeBuilder {
            runtime: self.runtime,
            typestate: PhantomData,
        }
    }
}

impl RuntimeBuilder<ConfigFilesLoaded, PreShutdown> {
    pub async fn shutdown(self) -> RuntimeBuilder<ConfigFilesLoaded, Shutdown> {
        self.runtime.shutdown().await;
        RuntimeBuilder {
            runtime: self.runtime,
            typestate: PhantomData,
        }
    }
}

impl RuntimeBuilder<ConfigFilesLoaded, Shutdown> {
    pub async fn wait_for(self, duration: Duration) -> RuntimeBuilder<ConfigFilesLoaded, Shutdown> {
        tokio::time::sleep(duration).await;
        self
    }
}

impl Default for RuntimeBuilder<SetConfigLocations, NotRunning> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct OptionOption<T>(Option<T>);

impl<T> OptionOption<T> {
    pub fn get(self) -> Option<T> {
        self.0
    }
}

impl<T> From<T> for OptionOption<T> {
    fn from(value: T) -> Self {
        Self(Some(value))
    }
}

impl<T> From<Option<T>> for OptionOption<T> {
    fn from(value: Option<T>) -> Self {
        Self(value)
    }
}

impl From<&str> for OptionOption<String> {
    fn from(value: &str) -> Self {
        Self(Some(value.into()))
    }
}

impl From<String> for OptionOption<PathBuf> {
    fn from(value: String) -> Self {
        Self(Some(value.into()))
    }
}

impl From<Option<String>> for OptionOption<PathBuf> {
    fn from(value: Option<String>) -> Self {
        Self(value.map(|v| v.into()))
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn build_builder_get() {
    let runtime = RuntimeBuilder::new()
        .ignore_config_files()
        .instance_name("Test Runtime Builder Get")
        .pick_free_port()
        .disable_all_plugins(true)
        .get();
    let inner_runtime = runtime.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(2000)).await;
        inner_runtime.get_shutdown_manager().do_shutdown();
    });
    {
        runtime.init().await;
        runtime.post_init().await;
        runtime.run().await;
        runtime.pre_shutdown().await;
        runtime.shutdown().await;
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn run_it() {
    RuntimeBuilder::new()
        .load_config_files()
        .await
        .instance_name("Test runtime builder with timeout")
        .disable_all_plugins(true)
        .init()
        .await
        .post_init()
        .await
        .run_for(Duration::from_millis(300))
        .await
        .pre_shutdown()
        .await
        .shutdown()
        .await
        .wait_for(Duration::from_millis(300))
        .await;
}
