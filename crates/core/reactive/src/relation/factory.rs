use crate::model::ReactiveRelationInstance;
use crate::BehaviourFactory;

pub trait RelationBehaviourFactory: BehaviourFactory<ReactiveRelationInstance> {}
