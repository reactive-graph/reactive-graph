use std::collections::HashMap;
use std::ffi::OsStr;
use std::sync::Arc;

use libloading::Library;
use log::{debug, error};

use crate::plugin::proxy::PluginProxy;
use crate::plugin::registrar::PluginRegistrar;
use crate::plugins::{Plugin, PluginDeclaration, PluginError};
use crate::plugins::{INEXOR_RGF_PLUGIN_VERSION, RUSTC_VERSION};

#[derive(Default)]
pub struct PluginRegistry {
    pub plugins: HashMap<String, PluginProxy>,
    pub libraries: Vec<Arc<Library>>,
}

impl PluginRegistry {
    pub fn new() -> PluginRegistry {
        PluginRegistry::default()
    }

    /// Load a plugin library and add all contained functions to the internal
    /// function table.
    ///
    /// # Safety
    ///
    /// A plugin library **must** be implemented using the
    /// [`plugins_core::plugin_declaration!()`] macro. Trying manually implement
    /// a plugin without going through that macro will result in undefined
    /// behaviour.
    pub unsafe fn load<P: AsRef<OsStr>>(&mut self, library_path: P) -> Result<(), PluginError> {
        debug!("Loading library");
        // Load the library into memory
        let library = Library::new(library_path);
        match library {
            Ok(library) => {
                let library = Arc::new(library);
                // Get a pointer to the plugin_declaration symbol.
                let decl = library
                    .get::<*mut PluginDeclaration>(b"plugin_declaration\0")?
                    .read();
                // version checks to prevent accidental ABI incompatibilities
                if decl.rustc_version != RUSTC_VERSION {
                    error!("Version mismatch: rustc");
                    return Err(PluginError::Other {
                        message: String::from("Version mismatch: rustc"),
                    }
                    .into());
                }
                if decl.inexor_rgf_plugin_version != INEXOR_RGF_PLUGIN_VERSION {
                    error!("Version mismatch: inexor_rgf_core_plugins");
                    return Err(PluginError::Other {
                        message: String::from("Version mismatch: inexor_rgf_core_plugins"),
                    }
                    .into());
                }

                let mut registrar = PluginRegistrar::new(Arc::clone(&library));

                (decl.register)(&mut registrar);

                // add all loaded plugins to the plugins map
                self.plugins.extend(registrar.plugins);
                // and make sure PluginRegistry keeps a reference to the library
                self.libraries.push(library);

                Ok(())
            }
            Err(e) => {
                error!("Failed to load dynamic library: {}", e.to_string());
                return Err(PluginError::PluginCreationError);
            }
        }
    }

    pub fn init(&self, name: &str) -> Result<(), PluginError> {
        let plugin_proxy = self.plugins.get(name);
        match plugin_proxy {
            Some(plugin_proxy) => {
                debug!("Initializing plugin {}", name);
                plugin_proxy.init()
            }
            None => {
                error!("Failed to initialize plugin {}: Not found", name);
                return Err(PluginError::InitializationError);
            }
        }
    }

    pub fn post_init(&self, name: &str) -> Result<(), PluginError> {
        let plugin_proxy = self.plugins.get(name);
        match plugin_proxy {
            Some(plugin_proxy) => {
                debug!("Post-Initializing plugin {}", name);
                plugin_proxy.post_init()
            }
            None => {
                error!("Failed to post-initialize plugin {}: Not found", name);
                return Err(PluginError::PostInitializationError);
            }
        }
    }

    pub fn pre_shutdown(&self, name: &str) -> Result<(), PluginError> {
        let plugin_proxy = self.plugins.get(name);
        match plugin_proxy {
            Some(plugin_proxy) => {
                debug!("Pre-Shutdown plugin {}", name);
                plugin_proxy.pre_shutdown()
            }
            None => {
                error!("Failed to pre-shutdown plugin {}: Not found", name);
                return Err(PluginError::PreShutdownError);
            }
        }
    }

    pub fn shutdown(&self, name: &str) -> Result<(), PluginError> {
        let plugin_proxy = self.plugins.get(name);
        match plugin_proxy {
            Some(plugin_proxy) => {
                debug!("Shutting down plugin {}", name);
                plugin_proxy.shutdown()
            }
            None => {
                error!("Failed to shutdown plugin {}: Not found", name);
                return Err(PluginError::ShutdownError);
            }
        }
    }
}
