#![feature(register_tool)]
#![register_tool(tarpaulin)]

pub use behaviour::BehaviourConnect;
pub use behaviour::BehaviourConnectFailed;
pub use behaviour::BehaviourCreationError;
pub use behaviour::BehaviourDisconnect;
pub use behaviour::BehaviourDisconnectFailed;
pub use behaviour::BehaviourFactory;
pub use behaviour::BehaviourFsm;
pub use behaviour::BehaviourInit;
pub use behaviour::BehaviourInitializationFailed;
pub use behaviour::BehaviourInvalid;
pub use behaviour::BehaviourPropertyInvalid;
pub use behaviour::BehaviourPropertyValidator;
pub use behaviour::BehaviourReactiveInstanceContainer;
pub use behaviour::BehaviourReconnectFailed;
pub use behaviour::BehaviourRelationInstanceContainer;
pub use behaviour::BehaviourShutdown;
pub use behaviour::BehaviourShutdownFailed;
pub use behaviour::BehaviourState;
pub use behaviour::BehaviourStorage;
pub use behaviour::BehaviourTransitionError;
pub use behaviour::BehaviourTransitions;
pub use behaviour::BehaviourTypeContainer;
pub use behaviour::BehaviourValidator;
pub use behaviour::EntityPropertyObserverContainerImpl;
pub use behaviour::PropertyObserverContainer;
pub use behaviour::RelationBehaviourReactiveInstanceContainerImpl;
pub use behaviour::RelationPropertyObserverContainerImpl;
pub use behaviour::RelationPropertyValidator;
pub use entity::EntityBehaviourFactory;
pub use entity::EntityBehaviourStorage;
pub use entity::Expression;
pub use entity::ExpressionResult;
pub use entity::ExpressionValue;
pub use entity::Gate;
pub use entity::Operation;
pub use entity::OperatorPosition;
pub use property::*;
pub use relation::RelationBehaviourFactory;
pub use relation::RelationBehaviourStorage;

use inexor_rgf_core_model as model;

pub mod behaviour;
pub mod entity;
pub mod property;
pub mod relation;

#[cfg(test)]
#[tarpaulin::ignore]
pub mod tests;
