use std::collections::HashMap;
use std::env::consts::DLL_EXTENSION;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::debug;
use log::error;
use log::info;
use log::trace;
use log::warn;
use notify::event::AccessKind::Close;
use notify::event::AccessMode::Write;
use notify::Event;
use notify::EventKind::Access;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Watcher;
use springtime_di::component_alias;
use springtime_di::Component;
use tokio::sync::mpsc;
use uuid::Uuid;
use walkdir::WalkDir;

use reactive_graph_config_api::ConfigManager;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_plugin_api::HotDeployError;
use reactive_graph_plugin_api::PluginState;
use reactive_graph_plugin_api::PLUGIN_NAME_PREFIX;
use reactive_graph_plugin_service_api::PluginContainerManager;
use reactive_graph_plugin_service_api::PluginRepositoryManager;
use reactive_graph_plugin_service_api::PluginResolver;

use crate::plugin_paths::get_install_path;
use crate::plugin_paths::get_stem;

pub type HotDeployWatcher = RwLock<Option<RecommendedWatcher>>;

fn create_hot_deploy_watcher() -> HotDeployWatcher {
    RwLock::new(None)
}

#[derive(Component)]
pub struct PluginRepositoryManagerImpl {
    plugin_container_manager: Arc<dyn PluginContainerManager + Send + Sync>,

    plugin_resolver: Arc<dyn PluginResolver + Send + Sync>,

    config_manager: Arc<dyn ConfigManager + Send + Sync>,

    #[component(default = "create_hot_deploy_watcher")]
    hot_deploy_watcher: HotDeployWatcher,
}

impl PluginRepositoryManagerImpl {
    fn create_and_register_plugin_container(&self, path: PathBuf) -> Option<Uuid> {
        if !is_dll(&path) {
            return None;
        }
        if let Some(stem) = get_stem(&path) {
            return self.plugin_container_manager.create_and_register_plugin_container(stem, path);
        }
        None
    }

    async fn create_hot_deploy_watcher(&self) {
        let plugin_container_manager = self.plugin_container_manager.clone();
        let plugin_resolver = self.plugin_resolver.clone();
        let (tx, mut rx) = mpsc::channel::<notify::Result<Event>>(32);
        tokio::spawn(async move {
            trace!("Hot Deploy Watcher started");
            while let Some(r) = rx.recv().await {
                match r {
                    Ok(event) => {
                        if event.kind != Access(Close(Write)) {
                            continue;
                        }
                        trace!("Hot Deploy Watcher: Detected file system activity: {:?}", event);
                        for path in event.paths.clone() {
                            let Some(stem) = get_stem(&path) else {
                                continue;
                            };
                            if !is_dll(&path) {
                                continue;
                            }
                            if plugin_container_manager.has(&stem) {
                                // If plugin with the same stem is already installed, redeploy and start resolver
                                if let Some(id) = plugin_container_manager.get_id(&stem) {
                                    match plugin_container_manager.redeploy(&id) {
                                        Ok(_) => {
                                            plugin_resolver.resolve_until_idle().await;
                                            // Start dependent plugins
                                            while plugin_container_manager.start_dependent_with_satisfied_dependencies(&id) {
                                                // Resolve until all dependent plugins are started
                                                plugin_resolver.resolve_until_idle().await;
                                            }
                                            plugin_resolver.transition_to_fallback_states().await;
                                        }
                                        Err(e) => {
                                            error!("Failed to redeploy plugin {} {}: {:?}", &stem, &id, e);
                                        }
                                    }
                                }
                            } else {
                                // Deploy new plugins to the installation folder
                                if let Ok(install_path) = deploy_plugin(path) {
                                    // And register a new plugin container and start resolver
                                    if let Some(id) = plugin_container_manager.create_and_register_plugin_container(stem, install_path) {
                                        plugin_resolver.resolve_until_idle().await;
                                        if plugin_container_manager.start(&id).is_ok() {
                                            plugin_resolver.resolve_until_idle().await;
                                            // Start dependent plugins
                                            while plugin_container_manager.start_dependent_with_satisfied_dependencies(&id) {
                                                // Resolve until all dependent plugins are started
                                                plugin_resolver.resolve_until_idle().await;
                                            }
                                        }
                                        plugin_resolver.transition_to_fallback_states().await;
                                    }
                                }
                            }
                        }
                        for path in event.paths {
                            let Some(stem) = get_stem(&path) else {
                                continue;
                            };
                            if !is_dll(&path) {
                                continue;
                            }
                            let Some(id) = plugin_container_manager.get_id(&stem) else {
                                continue;
                            };
                            let name = plugin_container_manager.name(&id).unwrap_or_default().replace(&PLUGIN_NAME_PREFIX, "");
                            let version = plugin_container_manager.version(&id).unwrap_or(String::from("?.?.?"));
                            // Warn about disabled plugins
                            if let Some(state) = plugin_container_manager.get_plugin_state(&id) {
                                if state == PluginState::Disabled {
                                    info!("[DISABLED] {name} {version}");
                                }
                            }
                            // Warn about unsatisfied dependencies
                            for d in plugin_container_manager.get_unsatisfied_dependencies(&id) {
                                trace!(
                                    "Plugin {} {} has unsatisfied dependency: {}:{}",
                                    id,
                                    &name,
                                    d.name.replace(&PLUGIN_NAME_PREFIX, ""),
                                    d.version
                                );
                                match plugin_container_manager.get_plugin_by_dependency(&d) {
                                    Some(dependency_id) => {
                                        let dependency_name = plugin_container_manager
                                            .name(&dependency_id)
                                            .unwrap_or_default()
                                            .replace(&PLUGIN_NAME_PREFIX, "");
                                        let dependency_version = plugin_container_manager.version(&dependency_id).unwrap_or(String::from("?.?.?"));
                                        let dependency_state = plugin_container_manager.get_plugin_state(&dependency_id).unwrap_or(PluginState::Uninstalled);
                                        warn!(
                                            "Plugin {} has unsatisfied dependency: {}:{} - which exists ({} {}) but has state {:?}",
                                            &name,
                                            d.name.replace(&PLUGIN_NAME_PREFIX, ""),
                                            d.version,
                                            dependency_name,
                                            dependency_version,
                                            dependency_state
                                        );
                                    }
                                    None => {
                                        warn!(
                                            "Plugin {} has unsatisfied dependency: {}:{} - which doesn't exist",
                                            &name,
                                            d.name.replace(&PLUGIN_NAME_PREFIX, ""),
                                            d.version
                                        );
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Hot Deploy Watcher: Error: {}", e);
                    }
                }
            }
            trace!("Hot Deploy Watcher: Finished");
        });
        let watcher = notify::recommended_watcher(move |r: notify::Result<Event>| {
            let tx = tx.clone();
            futures::executor::block_on(async {
                match tx.send(r).await {
                    Ok(_) => {}
                    Err(e) => {
                        trace!("SendError {}", e);
                    }
                }
            });
        })
        .ok();
        let mut writer = self.hot_deploy_watcher.write().unwrap();
        *writer = watcher;
    }

    fn destroy_hot_deploy_watcher(&self) {
        let mut writer = self.hot_deploy_watcher.write().unwrap();
        *writer = None;
    }
}

#[async_trait]
#[component_alias]
impl PluginRepositoryManager for PluginRepositoryManagerImpl {
    fn scan_deploy_repository(&self) {
        let plugins_config = self.config_manager.get_plugins_config();
        let Some(hot_deploy_location) = plugins_config.get_hot_deploy_location() else {
            return;
        };
        trace!("Scanning plugin hot deploy folder {hot_deploy_location:?}");
        let Ok(dir) = fs::read_dir(hot_deploy_location) else {
            return;
        };
        for entry in dir.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if !file_type.is_file() {
                    continue;
                }
                let _ = deploy_plugin(entry.path());
            }
        }
    }

    fn remove_duplicates(&self) {
        let plugins_config = self.config_manager.get_plugins_config();
        let Some(install_location) = plugins_config.get_install_location() else {
            return;
        };
        let mut installed_plugins: HashMap<String, (u64, PathBuf)> = HashMap::new();
        let mut plugins_to_remove: Vec<PathBuf> = Vec::new();

        for entry in WalkDir::new(install_location)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
        {
            let p = entry.path();
            if let Some((stem, timestamp)) = p
                .file_stem()
                .and_then(|stem| {
                    stem.to_string_lossy()
                        .rsplit_once('.')
                        .map(|(stem, timestamp)| (String::from(stem), String::from(timestamp)))
                })
                .and_then(|(stem, timestamp)| timestamp.parse::<u64>().ok().map(|timestamp| (stem, timestamp)))
            {
                // let timestamp = timestamp.parse::<u64>();
                match installed_plugins.get_mut(&stem) {
                    Some(entry) => {
                        // (timestamp2, p2)
                        if entry.0 < timestamp {
                            plugins_to_remove.push(entry.1.clone());
                            entry.0 = timestamp;
                            entry.1 = PathBuf::from(p);
                            // filenames.insert(stem, (timestamp, PathBuf::from(p)));
                        } else {
                            plugins_to_remove.push(PathBuf::from(p));
                        }
                    }
                    None => {
                        installed_plugins.insert(stem, (timestamp, PathBuf::from(p)));
                    }
                }
            }
        }
        for plugin_to_remove in plugins_to_remove {
            if fs::remove_file(&plugin_to_remove).is_ok() {
                trace!("Removed duplicate plugin: {}", plugin_to_remove.display());
            }
        }
    }

    fn scan_plugin_repository(&self) {
        let plugins_config = self.config_manager.get_plugins_config();
        let Some(install_location) = plugins_config.get_install_location() else {
            return;
        };
        trace!("Scanning plugin installation folder {install_location:?}");
        let Ok(dir) = fs::read_dir(install_location) else {
            return;
        };
        for entry in dir.flatten() {
            if entry.file_type().map(|f| f.is_file()).unwrap_or(false) {
                self.create_and_register_plugin_container(entry.path());
            }
        }
    }

    fn watch_hot_deploy(&self) {
        let Some(hot_deploy_location) = self.config_manager.get_plugins_config().get_hot_deploy_location() else {
            return;
        };
        let mut writer = self.hot_deploy_watcher.write().unwrap();
        if let Some(recommended_watcher) = writer.as_mut() {
            // Add a path to be watched. All files and directories at that path and
            // below will be monitored for changes.
            match recommended_watcher.watch(&hot_deploy_location, RecursiveMode::NonRecursive) {
                Ok(_) => {
                    trace!("Watching hot deploy folder {hot_deploy_location:?}");
                }
                Err(e) => {
                    error!("Failed to watch hot deploy folder {hot_deploy_location:?}: {}", e);
                }
            }
        }
    }

    fn unwatch_hot_deploy(&self) {
        let Some(hot_deploy_location) = self.config_manager.get_plugins_config().get_hot_deploy_location() else {
            return;
        };
        let mut writer = self.hot_deploy_watcher.write().unwrap();
        if let Some(recommended_watcher) = writer.as_mut() {
            let _ = recommended_watcher.unwatch(&hot_deploy_location);
        }
    }
}

#[async_trait]
impl Lifecycle for PluginRepositoryManagerImpl {
    async fn init(&self) {
        // Initially, the deploy folder will be scanned. Detected plugins will be copied to the
        // install folder before the install folder will be scanned. Eventually existing plugins
        // will be overwritten by the version in the deploy folder.
        self.scan_deploy_repository();

        self.remove_duplicates();

        // Initially, scans the plugin installation folder and creates and registers plugin
        // containers for each plugin.
        self.scan_plugin_repository();

        // Create a deploy watcher.
        self.create_hot_deploy_watcher().await;
    }

    async fn post_init(&self) {
        // Initiates watching the hot deployment folder.
        self.watch_hot_deploy();
    }

    async fn pre_shutdown(&self) {
        self.unwatch_hot_deploy();
    }

    async fn shutdown(&self) {
        self.destroy_hot_deploy_watcher();
    }
}

fn is_dll(path: &Path) -> bool {
    if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
        return extension == DLL_EXTENSION;
    }
    false
}

fn deploy_plugin(deploy_path: PathBuf) -> Result<PathBuf, HotDeployError> {
    debug!("Detected new plugin {}", deploy_path.display());
    if !is_dll(&deploy_path) {
        return Err(HotDeployError::NoDynamicLinkLibrary);
    }
    let Some(install_path) = get_install_path(&deploy_path) else {
        return Err(HotDeployError::InvalidInstallPath);
    };
    match fs::rename(&deploy_path, &install_path) {
        Ok(_) => {
            debug!("Moved plugin from {} to {}", deploy_path.display(), install_path.display());
            Ok(install_path)
        }
        Err(_) => {
            error!("Failed to moved plugin from {} to {}", deploy_path.display(), install_path.display());
            Err(HotDeployError::MoveError)
        }
    }
}
