#![feature(lazy_cell)]
#![feature(register_tool)]
#![register_tool(tarpaulin)]

pub use behaviour::BehaviourConnect;
pub use behaviour::BehaviourConnectFailed;
pub use behaviour::BehaviourCreationError;
pub use behaviour::BehaviourDisconnect;
pub use behaviour::BehaviourDisconnectFailed;
pub use behaviour::BehaviourInit;
pub use behaviour::BehaviourInitializationFailed;
pub use behaviour::BehaviourInvalid;
pub use behaviour::BehaviourPropertyInvalid;
pub use behaviour::BehaviourReconnectFailed;
pub use behaviour::BehaviourRelationInstanceContainer;
pub use behaviour::BehaviourShutdown;
pub use behaviour::BehaviourShutdownFailed;
pub use behaviour::BehaviourState;
pub use behaviour::BehaviourStorage;
pub use behaviour::BehaviourTransitionError;
pub use behaviour::BehaviourTransitions;
pub use behaviour::BehaviourValidator;
pub use behaviour::EntityPropertyObserverContainerImpl;
pub use behaviour::PropertyObserverContainer;
pub use behaviour::RelationPropertyObserverContainerImpl;
pub use behaviour::RelationPropertyValidator;
pub use behaviour::RelationReactiveInstanceContainerImpl;
pub use entity::EntityBehaviourFactory;
pub use entity::EntityBehaviourStorage;
pub use entity::Expression;
pub use entity::ExpressionResult;
pub use entity::ExpressionValue;
pub use entity::Gate;
pub use entity::Operation;
pub use entity::OperatorPosition;
pub use relation::RelationBehaviourFactory;
pub use relation::RelationBehaviourStorage;

// Reactive Behaviours
pub mod behaviour;

// Entity Behaviours
pub mod entity;

// Relation Behaviours
pub mod relation;

#[cfg(test)]
#[tarpaulin::ignore]
pub mod tests;
