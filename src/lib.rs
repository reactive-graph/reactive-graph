pub use component_provider::ComponentProvider;
pub use entity_behaviour_provider::EntityBehaviourProvider;
pub use entity_type_provider::EntityTypeProvider;
pub use flow_provider::FlowProvider;
pub use plugin::Plugin;
pub use plugin::PluginDeclaration;
pub use plugin::PluginError;
pub use plugin::PluginRegistrar;
pub use relation_behaviour_provider::RelationBehaviourProvider;
pub use relation_type_provider::RelationTypeProvider;
pub use web_resource_provider::WebResourceProvider;

pub static INEXOR_RGF_PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

pub mod component_provider;
pub mod entity_behaviour_provider;
pub mod entity_type_provider;
pub mod flow_provider;
pub mod plugin;
pub mod relation_behaviour_provider;
pub mod relation_type_provider;
pub mod web_resource_provider;

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
