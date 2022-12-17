use std::env::consts::DLL_EXTENSION;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::RwLock;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use async_trait::async_trait;
use log::debug;
use log::error;
use log::trace;
use notify::event::AccessKind::Close;
use notify::event::AccessMode::Write;
use notify::Event;
use notify::EventKind::Access;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Watcher;
use uuid::Uuid;

use crate::api::Lifecycle;
use crate::api::PluginContainerManager;
use crate::api::PluginRepositoryManager;
use crate::api::PluginResolver;
use crate::di::*;

#[derive(Debug)]
pub enum HotDeployError {
    NoDynamicLinkLibrary,
    InvalidInstallPath,
    MoveError,
}

#[wrapper]
pub struct HotDeployWatcher(RwLock<Option<RecommendedWatcher>>);

#[provides]
fn create_hot_deploy_watcher() -> HotDeployWatcher {
    HotDeployWatcher(RwLock::new(None))
}

#[component]
pub struct PluginRepositoryManagerImpl {
    plugin_container_manager: Wrc<dyn PluginContainerManager>,

    plugin_resolver: Wrc<dyn PluginResolver>,

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

    fn create_hot_deploy_watcher(&self) {
        let plugin_container_manager = self.plugin_container_manager.clone();
        let plugin_resolver = self.plugin_resolver.clone();
        let watcher = notify::recommended_watcher(move |r: notify::Result<Event>| match r {
            Ok(event) => {
                if event.kind != Access(Close(Write)) {
                    return;
                }
                trace!("Hot deploy watcher detected file system activity: {:?}", event);
                for path in event.paths {
                    let Some(stem) = get_stem(&path) else {
                        continue;
                    };
                    if plugin_container_manager.has(&stem) {
                        // If plugin with the same stem is already installed, redeploy and start resolver
                        if let Some(id) = plugin_container_manager.get_id(&stem) {
                            match plugin_container_manager.redeploy(&id) {
                                Ok(_) => {
                                    plugin_resolver.resolve_until_idle();
                                    // Start dependent plugins
                                    while plugin_container_manager.start_dependent_with_satisfied_dependencies(&id) {
                                        // Resolve until all dependent plugins are started
                                        plugin_resolver.resolve_until_idle();
                                    }
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
                                plugin_resolver.resolve_until_idle();
                                if plugin_container_manager.start(&id).is_ok() {
                                    plugin_resolver.resolve_until_idle();
                                    // Start dependent plugins
                                    while plugin_container_manager.start_dependent_with_satisfied_dependencies(&id) {
                                        // Resolve until all dependent plugins are started
                                        plugin_resolver.resolve_until_idle();
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {}
        })
        .ok();
        let mut writer = self.hot_deploy_watcher.0.write().unwrap();
        *writer = watcher;
    }

    fn destroy_hot_deploy_watcher(&self) {
        let mut writer = self.hot_deploy_watcher.0.write().unwrap();
        *writer = None;
    }
}

#[async_trait]
#[provides]
impl PluginRepositoryManager for PluginRepositoryManagerImpl {
    fn scan_deploy_repository(&self) {
        trace!("Scanning folder plugins/deploy");
        if let Ok(dir) = fs::read_dir("./plugins/deploy") {
            for entry in dir.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if !file_type.is_file() {
                        continue;
                    }
                    let _ = deploy_plugin(entry.path());
                }
            }
        }
    }

    fn scan_plugin_repository(&self) {
        trace!("Scanning folder plugins/installed");
        if let Ok(dir) = fs::read_dir("./plugins/installed") {
            for entry in dir.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        self.create_and_register_plugin_container(entry.path());
                    }
                }
            }
        }
    }

    fn watch_hot_deploy(&self) {
        let mut writer = self.hot_deploy_watcher.0.write().unwrap();
        if let Some(recommended_watcher) = writer.as_mut() {
            // Add a path to be watched. All files and directories at that path and
            // below will be monitored for changes.
            let deploy_folder = Path::new("./plugins/deploy");
            let c_deploy_folder = fs::canonicalize(deploy_folder).map(|p| p.display().to_string()).unwrap_or_default();
            match recommended_watcher.watch(deploy_folder, RecursiveMode::NonRecursive) {
                Ok(_) => {
                    trace!("Watching folder {}", c_deploy_folder);
                }
                Err(e) => {
                    error!("Failed to watch folder {}: {}", c_deploy_folder, e);
                }
            }
        }
    }

    fn unwatch_hot_deploy(&self) {
        let mut writer = self.hot_deploy_watcher.0.write().unwrap();
        if let Some(recommended_watcher) = writer.as_mut() {
            let deploy_folder = Path::new("./plugins/deploy");
            let _ = recommended_watcher.unwatch(deploy_folder);
        }
    }
}

impl Lifecycle for PluginRepositoryManagerImpl {
    fn init(&self) {
        // Initially, the deploy folder will be scanned. Detected plugins will be copied to the
        // install folder before the install folder will be scanned. Eventually existing plugins
        // will be overwritten by the version in the deploy folder.
        self.scan_deploy_repository();

        // Initially, scans the folder plugins/installed and creates and registers plugin
        // containers for each plugin.
        self.scan_plugin_repository();

        // Create a deploy watcher.
        self.create_hot_deploy_watcher();
    }

    fn post_init(&self) {
        // Initiates watching the folder plugins/deploy.
        self.watch_hot_deploy();
    }

    fn pre_shutdown(&self) {
        self.unwatch_hot_deploy();
    }

    fn shutdown(&self) {
        self.destroy_hot_deploy_watcher();
    }
}

fn get_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

pub fn get_deploy_path(path: &Path) -> Option<PathBuf> {
    path.file_prefix().and_then(|file_prefix| {
        path.parent()
            .and_then(|path| path.parent())
            .map(|path| path.join("deploy").join(file_prefix).with_extension(DLL_EXTENSION))
    })
}

pub fn get_install_path(path: &Path) -> Option<PathBuf> {
    path.file_prefix().and_then(|file_prefix| {
        path.parent().and_then(|path| path.parent()).map(|path| {
            path.join("installed")
                .join(file_prefix)
                .with_extension(format!("{}.{}", get_timestamp(), DLL_EXTENSION))
        })
    })
}

fn get_stem(path: &Path) -> Option<String> {
    path.file_prefix().and_then(|stem| Some(stem.to_str()?.to_string()))
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
