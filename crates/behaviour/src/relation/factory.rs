use crate::BehaviourFactory;
use crate::model::RelationInstanceId;
use crate::reactive::ReactiveRelation;

pub trait RelationBehaviourFactory: BehaviourFactory<RelationInstanceId, ReactiveRelation> {}
