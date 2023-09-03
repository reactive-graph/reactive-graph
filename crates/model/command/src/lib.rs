use inexor_rgf_core_model as model;
use inexor_rgf_model_runtime as model_runtime;
use inexor_rgf_reactive as reactive;

pub use crate::component::CommandProperties;
pub use crate::component::COMPONENT_COMMAND;
pub use crate::component::COMPONENT_NAME_COMMAND;
pub use crate::entity::arg::CommandArg;
pub use crate::entity::arg::CommandArgs;
pub use crate::entity::command::Command;

pub mod builder;
pub mod component;
pub mod entity;
pub mod error;

pub const NAMESPACE_COMMAND: &str = "command";
