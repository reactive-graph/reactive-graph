pub mod plugin {
    pub use std::sync::Arc;

    pub use async_trait::async_trait;

    pub use inexor_rgf_graph::Components;
    pub use inexor_rgf_graph::EntityTypes;
    pub use inexor_rgf_graph::FlowTypes;
    pub use inexor_rgf_graph::RelationTypes;

    pub use crate::component_alias;
    pub use crate::export_plugin;
    pub use crate::injectable;
    pub use crate::Component;
    pub use crate::Plugin;
    pub use crate::PluginActivationError;
    pub use crate::PluginContext;
    pub use crate::PluginDeactivationError;
    pub use crate::TypeProvider;
    pub use crate::WebResourceProvider;
}

pub mod providers {
    pub use inexor_rgf_graph::Components;
    pub use inexor_rgf_graph::EntityTypes;
    pub use inexor_rgf_graph::FlowTypes;
    pub use inexor_rgf_graph::RelationTypes;

    pub use crate::Component;
    pub use crate::ComponentProviderRegistry;
    pub use crate::EntityTypeProviderRegistry;
    pub use crate::FlowTypeProviderRegistry;
    pub use crate::RelationTypeProviderRegistry;
    pub use crate::TypeProvider;
}
