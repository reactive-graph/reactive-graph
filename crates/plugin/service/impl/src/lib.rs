#![feature(path_file_prefix)]

pub use container::*;
pub use context::*;
pub use plugin_container_manager_impl::*;
pub use plugin_context_factory_impl::*;
pub use plugin_repository_manager_impl::*;
pub use plugin_resolver_impl::*;
pub use plugin_system_impl::*;
pub use proxy::*;
pub(crate) use registrar::*;

pub mod container;
pub mod context;
pub mod plugin_container_manager_impl;
pub mod plugin_context_factory_impl;
pub mod plugin_paths;
pub mod plugin_repository_manager_impl;
pub mod plugin_resolver_impl;
pub mod plugin_system_impl;
pub mod proxy;
pub mod registrar;
