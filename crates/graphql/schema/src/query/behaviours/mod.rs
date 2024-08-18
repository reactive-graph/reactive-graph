pub use behaviour::*;
pub use behaviours::*;
pub use component_behaviour::*;
pub use entity_behaviour::*;
pub use relation_behaviour::*;

pub mod behaviour;
#[allow(clippy::module_inception)]
pub mod behaviours;
pub mod component_behaviour;
pub mod entity_behaviour;
pub mod relation_behaviour;
