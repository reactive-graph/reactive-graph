pub use client::system;
pub use client::system::command::api::Command;
pub use client::system::instance::api::Instance;
pub use client::system::plugin::api::Plugins;
pub use client::system::remotes::api::Remotes;
pub use client::types;
pub use client::types::components::api::Components;
pub use client::InexorRgfClient;
pub use client::InexorRgfClientError;
pub use client::InexorRgfClientExecutionError;

pub use schema::system::instance::*;
pub use schema::system::plugin::*;
pub use schema::types::component::*;
pub use schema::types::data_type::*;
pub use schema::types::extension::*;
pub use schema::types::mutability::*;
pub use schema::types::property_instance::*;
pub use schema::types::property_type::*;
pub use schema::types::socket_type::*;

use inexor_rgf_config as config;
use inexor_rgf_graph as model;
use inexor_rgf_model_runtime as model_runtime;

pub mod client;
pub mod schema;
