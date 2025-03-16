#[cfg(feature = "json5")]
pub use json5;
pub use serde_json;
pub use springtime_di;
pub use springtime_di::Component;
pub use springtime_di::component_alias;
pub use springtime_di::injectable;
pub use springtime_di::instance_provider::ComponentInstancePtr;
pub use springtime_di::instance_provider::ErrorPtr;
#[cfg(feature = "toml")]
pub use toml;

pub use crate::PluginLoadingError;
pub use PluginActivationError;
pub use PluginDeactivationError;
pub use PluginUnloadingError;
pub use behaviours::entities::entity_behaviour_registry::*;
pub use behaviours::entities::entity_component_behaviour_registry::*;
pub use behaviours::relations::relation_behaviour_registry::*;
pub use behaviours::relations::relation_component_behaviour_registry::*;
pub use error::activation::*;
pub use error::hot_deploy::*;
pub use error::lifecycle::*;
pub use error::loading::*;
pub use graphql::graphql_query_service::*;
pub use graphql::http_body::HttpBody;
pub use graphql::web_resource_manager::*;
pub use graphql::web_resource_provider::*;
pub use instances::entities::entity_instance_manager::*;
pub use instances::flows::flow_instance_manager::*;
pub use instances::relations::relation_instance_manager::*;
pub use plugin::PLUGIN_NAME_PREFIX;
pub use plugin::Plugin;
pub use plugin_context::*;
pub use plugin_declaration::*;
pub use plugin_dependency::*;
pub use plugin_state::PluginDeployError;
pub use plugin_state::PluginDisableError;
pub use plugin_state::PluginRefreshingState;
pub use plugin_state::PluginResolveState;
pub use plugin_state::PluginStartError;
pub use plugin_state::PluginStartingState;
pub use plugin_state::PluginState;
pub use plugin_state::PluginStopError;
pub use plugin_state::PluginStoppingState;
pub use plugin_state::PluginUninstallError;
pub use plugin_state::PluginUninstallingState;
pub use reactive_graph_graph as model;
pub use system::command_manager::*;
pub use system::config_manager::*;
pub use types::components::component_import_export_manager::*;
pub use types::components::component_manager::*;
pub use types::components::component_provider_registry::*;
pub use types::entities::entity_type_import_export_manager::*;
pub use types::entities::entity_type_manager::*;
pub use types::entities::entity_type_provider_registry::*;
pub use types::flows::flow_type_import_export_manager::*;
pub use types::flows::flow_type_manager::*;
pub use types::flows::flow_type_provider_registry::*;
pub use types::relations::relation_type_import_export_manager::*;
pub use types::relations::relation_type_manager::*;
pub use types::relations::relation_type_provider_registry::*;
pub use types::type_system_event_manager::*;

pub use reactive_graph_type_system_api::TypeProvider;
// TODO: pub use reactive_graph_type_system_api::*;

pub mod behaviours;
pub mod instances;
pub mod types;

pub mod system;

pub mod embedded_asset_provider;
pub mod error;
pub mod graphql;
pub mod plugin;
pub mod plugin_context;
pub mod plugin_declaration;
pub mod plugin_dependency;
pub mod plugin_state;

pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");
pub static PLUGIN_API_VERSION: &str = env!("CARGO_PKG_VERSION");

#[macro_export]
macro_rules! export_plugin {
    () => {
        $crate::export_plugin_constants!();
        $crate::construct_plugin!();
        $crate::register_plugin!();
        $crate::plugin_dependencies!();
        $crate::export_plugin_declaration!();
    };
    ({
        "plugin": {
            "name": $name: expr,
            "description": $description: expr,
            "version": $version: expr $(,)?
        } $(,)?
    }) => {
        $crate::export_plugin_constants!(
            "plugin": {
                "name": $name,
                "description": $description,
                "version": $version,
            },
        );
        $crate::construct_plugin!();
        $crate::register_plugin!();
        $crate::plugin_dependencies!();
        $crate::export_plugin_declaration!();
    };
    ({
        "dependencies": [
            $({
                "name": $plugin_name: expr,
                "version": $version_range: expr $(,)?
            } $(,)?)*
        ]
    }) => {
        $crate::export_plugin_constants!();
        $crate::construct_plugin!();
        $crate::register_plugin!();
        $crate::plugin_dependencies!(
            "dependencies": [
                $({
                    "name": $plugin_name,
                    "version": $version_range,
                },)*
            ],
        );
        $crate::export_plugin_declaration!();
    };
    ({
        "plugin": {
            "name": $name: expr,
            "description": $description: expr,
            "version": $version: expr $(,)?
        },
        "dependencies": [
            $({
                "name": $plugin_name: expr,
                "version": $version_range: expr $(,)?
            } $(,)?)*
        ]
        $(,)?
    }) => {
        $crate::export_plugin_constants!(
            "plugin": {
                "name": $name,
                "description": $description,
                "version": $version,
            },
        );
        $crate::construct_plugin!();
        $crate::register_plugin!();
        $crate::plugin_dependencies!(
            "dependencies": [
                $({
                    "name": $plugin_name,
                    "version": $version_range,
                },)*
            ],
        );
        $crate::export_plugin_declaration!();
    };
}
#[macro_export]
macro_rules! export_plugin_constants {
    () => {
        /// The name of the plugin (CARGO_PKG_NAME).
        pub static PLUGIN_NAME: &str = env!("CARGO_PKG_NAME");

        /// The description of the plugin (CARGO_PKG_DESCRIPTION).
        pub static PLUGIN_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

        /// The version of the plugin (CARGO_PKG_VERSION).
        pub static PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
    };
    (
        "plugin": {
            "name": $name: expr,
            "description": $description: expr,
            "version": $version: expr $(,)?
        } $(,)?
    ) => {
        /// The name of the plugin (CARGO_PKG_NAME).
        pub static PLUGIN_NAME: &str = $name;

        /// The description of the plugin (CARGO_PKG_DESCRIPTION).
        pub static PLUGIN_DESCRIPTION: &str = $description;

        /// The version of the plugin (CARGO_PKG_VERSION).
        pub static PLUGIN_VERSION: &str = $version;
    };
}

#[macro_export]
macro_rules! construct_plugin {
    () => {
        /// Static plugin context.
        ///
        /// This plugin context is created by construct_plugin before constructing
        /// the dependency injection container.
        pub static PLUGIN_CONTEXT: std::sync::OnceLock<std::sync::Arc<dyn $crate::PluginContext + Send + Sync>> = std::sync::OnceLock::new();

        /// Returns the static plugin context if called after construct_plugin or an empty option if called before construct_plugin.
        ///
        /// ```
        /// pub struct MyPluginImpl {
        ///   #[component(default = "inject_plugin_context_checked")]
        ///   context: Option<std::sync::Arc<dyn reactive_graph_plugin_api::PluginContext + Send + Sync>>,
        /// }
        /// ```
        pub fn inject_plugin_context_checked() -> Option<std::sync::Arc<dyn $crate::PluginContext + Send + Sync>> {
            PLUGIN_CONTEXT.get().cloned()
        }

        /// Returns the static plugin context.
        ///
        /// ```
        /// pub struct MyPluginImpl {
        ///   #[component(default = "inject_plugin_context")]
        ///   context: std::sync::Arc<dyn reactive_graph_plugin_api::PluginContext + Send + Sync>,
        /// }
        /// ```
        ///
        /// # Panics
        ///
        /// Panics if the inject_plugin_context was called before construct_plugin!
        ///
        pub fn inject_plugin_context() -> std::sync::Arc<dyn $crate::PluginContext + Send + Sync> {
            let Some(context) = PLUGIN_CONTEXT.get().cloned() else {
                panic!("The plugin context is uninitialized!");
            };
            context
        }

        pub fn component_provider_registry() -> std::sync::Arc<dyn reactive_graph_plugin_api::ComponentProviderRegistry + Send + Sync> {
            let Some(context) = PLUGIN_CONTEXT.get().cloned() else {
                panic!("The plugin context is uninitialized!");
            };
            context.get_component_provider_registry()
        }

        pub fn entity_types_provider_registry() -> std::sync::Arc<dyn reactive_graph_plugin_api::EntityTypeProviderRegistry + Send + Sync> {
            let Some(context) = PLUGIN_CONTEXT.get().cloned() else {
                panic!("The plugin context is uninitialized!");
            };
            context.get_entity_type_provider_registry()
        }

        pub fn relation_types_provider_registry() -> std::sync::Arc<dyn reactive_graph_plugin_api::RelationTypeProviderRegistry + Send + Sync> {
            let Some(context) = PLUGIN_CONTEXT.get().cloned() else {
                panic!("The plugin context is uninitialized!");
            };
            context.get_relation_type_provider_registry()
        }

        pub fn flow_types_provider_registry() -> std::sync::Arc<dyn reactive_graph_plugin_api::FlowTypeProviderRegistry + Send + Sync> {
            let Some(context) = PLUGIN_CONTEXT.get().cloned() else {
                panic!("The plugin context is uninitialized!");
            };
            context.get_flow_type_provider_registry()
        }

        pub fn entity_behaviour_registry() -> std::sync::Arc<dyn reactive_graph_plugin_api::EntityBehaviourRegistry + Send + Sync> {
            let Some(context) = PLUGIN_CONTEXT.get().cloned() else {
                panic!("The plugin context is uninitialized!");
            };
            context.get_entity_behaviour_registry()
        }

        pub fn entity_component_behaviour_registry() -> std::sync::Arc<dyn reactive_graph_plugin_api::EntityComponentBehaviourRegistry + Send + Sync> {
            let Some(context) = PLUGIN_CONTEXT.get().cloned() else {
                panic!("The plugin context is uninitialized!");
            };
            context.get_entity_component_behaviour_registry()
        }

        pub fn relation_behaviour_registry() -> std::sync::Arc<dyn reactive_graph_plugin_api::RelationBehaviourRegistry + Send + Sync> {
            let Some(context) = PLUGIN_CONTEXT.get().cloned() else {
                panic!("The plugin context is uninitialized!");
            };
            context.get_relation_behaviour_registry()
        }

        pub fn relation_component_behaviour_registry() -> std::sync::Arc<dyn reactive_graph_plugin_api::RelationComponentBehaviourRegistry + Send + Sync> {
            let Some(context) = PLUGIN_CONTEXT.get().cloned() else {
                panic!("The plugin context is uninitialized!");
            };
            context.get_relation_component_behaviour_registry()
        }

        pub fn component_manager() -> std::sync::Arc<dyn reactive_graph_plugin_api::ComponentManager + Send + Sync> {
            let Some(context) = PLUGIN_CONTEXT.get().cloned() else {
                panic!("The plugin context is uninitialized!");
            };
            context.get_component_manager()
        }

        pub fn entity_type_manager() -> std::sync::Arc<dyn reactive_graph_plugin_api::EntityTypeManager + Send + Sync> {
            let Some(context) = PLUGIN_CONTEXT.get().cloned() else {
                panic!("The plugin context is uninitialized!");
            };
            context.get_entity_type_manager()
        }

        pub fn relation_type_manager() -> std::sync::Arc<dyn reactive_graph_plugin_api::RelationTypeManager + Send + Sync> {
            let Some(context) = PLUGIN_CONTEXT.get().cloned() else {
                panic!("The plugin context is uninitialized!");
            };
            context.get_relation_type_manager()
        }

        pub fn flow_type_manager() -> std::sync::Arc<dyn reactive_graph_plugin_api::FlowTypeManager + Send + Sync> {
            let Some(context) = PLUGIN_CONTEXT.get().cloned() else {
                panic!("The plugin context is uninitialized!");
            };
            context.get_flow_type_manager()
        }

        pub fn web_resource_manager() -> std::sync::Arc<dyn reactive_graph_plugin_api::WebResourceManager + Send + Sync> {
            let Some(context) = PLUGIN_CONTEXT.get().cloned() else {
                panic!("The plugin context is uninitialized!");
            };
            context.get_web_resource_manager()
        }

        pub fn config_manager() -> std::sync::Arc<dyn reactive_graph_plugin_api::ConfigManager + Send + Sync> {
            let Some(context) = PLUGIN_CONTEXT.get().cloned() else {
                panic!("The plugin context is uninitialized!");
            };
            context.get_config_manager()
        }

        pub fn entity_instance_manager() -> std::sync::Arc<dyn reactive_graph_plugin_api::EntityInstanceManager + Send + Sync> {
            let Some(context) = PLUGIN_CONTEXT.get().cloned() else {
                panic!("The plugin context is uninitialized!");
            };
            context.get_entity_instance_manager()
        }

        pub fn relation_instance_manager() -> std::sync::Arc<dyn reactive_graph_plugin_api::RelationInstanceManager + Send + Sync> {
            let Some(context) = PLUGIN_CONTEXT.get().cloned() else {
                panic!("The plugin context is uninitialized!");
            };
            context.get_relation_instance_manager()
        }

        /// Constructs the plugin and initializes the plugin context.
        ///
        /// This method guarantees that the plugin context is set before initializing the
        /// dependency injection framework. After `PLUGIN_CONTEXT.set` has been called, it's
        /// guaranteed that PLUGIN_CONTEXT has a value. See also the documentation of OnceLock::set.
        ///
        /// Returns an error if the plugin construction failed, by either:
        /// - The dependency injection container failed to construct
        /// - The dependency injection container doesn't contain a component of type Plugin
        /// - A component has unsatisfied dependencies.
        pub fn construct_plugin(
            context: std::sync::Arc<dyn $crate::PluginContext + Send + Sync>,
        ) -> Result<std::sync::Arc<dyn $crate::Plugin + Send + Sync>, $crate::PluginLoadingError> {
            let _ = PLUGIN_CONTEXT.set(context);
            // After set has been called, it's guaranteed that PLUGIN_CONTEXT has a value.
            $crate::springtime_di::factory::ComponentFactoryBuilder::new()
                // Propagate errors from the ComponentFactoryBuilder
                .map_err($crate::PluginLoadingError::ComponentDefinitionRegistryError)
                .map(|component_factory| component_factory.build())
                .and_then(|mut component_factory| {
                    // Get the
                    $crate::springtime_di::instance_provider::TypedComponentInstanceProvider::primary_instance_typed::<dyn $crate::Plugin + Send + Sync>(
                        &mut component_factory,
                    )
                    .map_err($crate::PluginLoadingError::ComponentInstanceProviderError)
                })
        }
    };
}

#[macro_export]
macro_rules! register_plugin {
    () => {
        /// The `register` method is
        #[allow(improper_ctypes_definitions)]
        extern "C" fn register(registrar: &mut dyn $crate::PluginRegistrar) -> Result<(), $crate::PluginLoadingError> {
            if let Err(error) = log4rs::init_file("config/logging.toml", Default::default()) {
                println!("Failed to configure logger in {}: {}", PLUGIN_NAME, error);
            }
            match construct_plugin(registrar.context()) {
                Ok(plugin) => {
                    registrar.register_plugin(Box::new(plugin));
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
    };
}

#[macro_export]
macro_rules! get_context {
    ($context: expr, $err: expr) => {
        $context.clone().ok_or($err)?
    };
}

pub mod prelude;

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
pub mod tests;
