#![cfg_attr(unboxed_closures, feature(unboxed_closures))]
#![cfg_attr(fn_traits, feature(fn_traits))]

pub use error::entity::*;
pub use error::flow::*;
pub use error::relation::*;
pub use event_channels::*;
pub use flow_instance_provider::*;
pub use property::*;
pub use reactive_entity_manager::*;
pub use reactive_flow_manager::*;
#[cfg(feature = "derive")]
pub use reactive_graph_reactive_derive::ReactiveEntity;
#[cfg(feature = "derive")]
pub use reactive_graph_reactive_derive::reactive_entity;
pub use reactive_instance_event_manager::*;
pub use reactive_instance_event_subscriber::*;
pub use reactive_instance_events::*;
pub use reactive_relation_manager::*;
pub use reactive_system::*;

pub mod error;
pub mod flow_instance_provider;
pub mod property;
pub mod reactive_entity_manager;
pub mod reactive_flow_manager;
pub mod reactive_instance_event_manager;

pub mod event_channels;
pub mod reactive_instance_event_subscriber;
pub mod reactive_instance_events;
pub mod reactive_relation_manager;
pub mod reactive_system;

pub mod prelude {
    pub use crate::error::entity::*;
    pub use crate::error::flow::*;
    pub use crate::error::relation::*;
    pub use crate::flow_instance_provider::*;
    pub use crate::property::*;
    pub use crate::reactive_entity_manager::*;
    pub use crate::reactive_flow_manager::*;
    pub use crate::reactive_relation_manager::*;
}
