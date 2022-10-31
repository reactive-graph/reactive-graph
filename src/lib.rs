#![feature(register_tool)]
#![register_tool(tarpaulin)]

pub use component_behaviour_provider::ComponentBehaviourProvider;
pub use component_behaviour_provider::ComponentBehaviourProviderError;
pub use component_manager::ComponentCreationError;
pub use component_manager::ComponentImportError;
pub use component_manager::ComponentManager;
pub use component_manager::ComponentManagerError;
pub use component_provider::ComponentProvider;
pub use component_provider::ComponentProviderError;
pub use entity_behaviour_provider::EntityBehaviourProvider;
pub use entity_behaviour_provider::EntityBehaviourProviderError;
pub use entity_instance_manager::EntityInstanceManager;
pub use entity_instance_manager::EntityInstanceManagerError;
pub use entity_type_manager::EntityTypeCreationError;
pub use entity_type_manager::EntityTypeImportError;
pub use entity_type_manager::EntityTypeManager;
pub use entity_type_manager::EntityTypeManagerError;
pub use entity_type_provider::EntityTypeProvider;
pub use entity_type_provider::EntityTypeProviderError;
pub use flow_instance_manager::FlowInstanceCreationError;
pub use flow_instance_manager::FlowInstanceManager;
pub use flow_instance_manager::FlowInstanceManagerError;
pub use flow_instance_provider::FlowInstanceProvider;
pub use flow_instance_provider::FlowInstanceProviderError;
pub use flow_type_manager::FlowTypeCreationError;
pub use flow_type_manager::FlowTypeManager;
pub use flow_type_manager::FlowTypeManagerError;
pub use flow_type_provider::FlowTypeProvider;
pub use flow_type_provider::FlowTypeProviderError;
pub use graphql_query_service::GraphQLQueryService;
pub use http_body::HttpBody;
pub use plugin::Plugin;
pub use plugin::PluginActivationError;
pub use plugin::PluginDeactivationError;
pub use plugin::PluginLoadingError;
pub use plugin::PluginUnloadingError;
pub use plugin_context::PluginContext;
pub use plugin_context::PluginContextDeinitializationError;
pub use plugin_context::PluginContextInitializationError;
pub use plugin_declaration::PluginDeclaration;
pub use plugin_declaration::PluginRegistrar;
pub use plugin_dependency::PluginDependency;
pub use plugin_state::PluginState;
pub use relation_behaviour_provider::RelationBehaviourProvider;
pub use relation_behaviour_provider::RelationBehaviourProviderError;
pub use relation_instance_manager::RelationInstanceManager;
pub use relation_instance_manager::RelationInstanceManagerError;
pub use relation_type_manager::RelationTypeCreationError;
pub use relation_type_manager::RelationTypeImportError;
pub use relation_type_manager::RelationTypeManager;
pub use relation_type_manager::RelationTypeManagerError;
pub use relation_type_provider::RelationTypeProvider;
pub use relation_type_provider::RelationTypeProviderError;
pub use system_event_manager::SystemEventManager;
pub use system_events::*;
pub use web_resource_provider::WebResourceProvider;
pub use web_resource_provider::WebResourceProviderError;

use inexor_rgf_core_model as model;

pub mod component_behaviour_provider;
pub mod component_manager;
pub mod component_provider;
pub mod entity_behaviour_provider;
pub mod entity_instance_manager;
pub mod entity_type_manager;
pub mod entity_type_provider;
pub mod flow_instance_manager;
pub mod flow_instance_provider;
pub mod flow_type_manager;
pub mod flow_type_provider;
pub mod graphql_query_service;
pub mod http_body;
pub mod plugin;
pub mod plugin_context;
pub mod plugin_declaration;
pub mod plugin_dependency;
pub mod plugin_state;
pub mod relation_behaviour_provider;
pub mod relation_instance_manager;
pub mod relation_type_manager;
pub mod relation_type_provider;
pub mod system_event_manager;
pub mod system_events;
pub mod web_resource_provider;

pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");
pub static PLUGIN_API_VERSION: &str = env!("CARGO_PKG_VERSION");

#[macro_export]
macro_rules! export_plugin {
    ($register:expr, $get_dependencies:expr, $name:expr, $description:expr, $version:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static plugin_declaration: $crate::PluginDeclaration = $crate::PluginDeclaration {
            rustc_version: $crate::RUSTC_VERSION,
            plugin_api_version: $crate::PLUGIN_API_VERSION,
            name: $name,
            description: $description,
            version: $version,
            register: $register,
            get_dependencies: $get_dependencies,
        };
    };
}

#[macro_export]
macro_rules! embedded_asset_provider_impl {
    ($asset: ident, $ty: ident) => {{
        let mut entries = Vec::new();
        for file in $asset::iter() {
            let filename = file.as_ref();
            debug!("Loading resource {}", filename);
            match $asset::get(filename) {
                Some(asset) => match std::str::from_utf8(asset.data.as_ref()) {
                    Ok(json_str) => match serde_json::from_str(json_str) {
                        Ok(parsed_entry) => {
                            let entry: $ty = parsed_entry;
                            entries.push(entry);
                        }
                        Err(e) => error!("Error in parsing JSON file {}: {}", filename, e),
                    },
                    Err(e) => error!("Error in decoding file to UTF-8 {}: {}", filename, e),
                },
                None => {}
            }
        }
        entries
    }};
}

#[cfg(test)]
#[tarpaulin::ignore]
pub mod tests;
