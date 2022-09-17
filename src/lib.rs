pub use component_behaviour_provider::ComponentBehaviourProvider;
pub use component_manager::ComponentManager;
pub use component_provider::ComponentProvider;
pub use entity_behaviour_provider::EntityBehaviourProvider;
pub use entity_instance_manager::EntityInstanceManager;
pub use entity_type_manager::EntityTypeManager;
pub use entity_type_provider::EntityTypeProvider;
pub use flow_instance_manager::FlowInstanceCreationError;
pub use flow_instance_manager::FlowInstanceManager;
pub use flow_instance_provider::FlowInstanceProvider;
pub use http_body::HttpBody;
pub use plugin::Plugin;
pub use plugin::PluginDeclaration;
pub use plugin::PluginError;
pub use plugin::PluginRegistrar;
pub use plugin_context::PluginContext;
pub use relation_behaviour_provider::RelationBehaviourProvider;
pub use relation_instance_manager::RelationInstanceManager;
pub use relation_type_manager::RelationTypeManager;
pub use relation_type_provider::RelationTypeProvider;
pub use web_resource_provider::WebResourceProvider;

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
pub mod http_body;
pub mod plugin;
pub mod plugin_context;
pub mod relation_behaviour_provider;
pub mod relation_instance_manager;
pub mod relation_type_manager;
pub mod relation_type_provider;
pub mod web_resource_provider;

pub static INEXOR_RGF_PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

#[macro_export]
macro_rules! export_plugin {
    ($register:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static plugin_declaration: $crate::PluginDeclaration = $crate::PluginDeclaration {
            rustc_version: $crate::RUSTC_VERSION,
            inexor_rgf_plugin_version: $crate::INEXOR_RGF_PLUGIN_VERSION,
            register: $register,
        };
    };
}
