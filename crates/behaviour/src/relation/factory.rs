use inexor_rgf_behaviour_api::prelude::*;

use crate::model::RelationInstanceId;
use crate::reactive::ReactiveRelation;

pub trait RelationBehaviourFactory: BehaviourFactory<RelationInstanceId, ReactiveRelation> {}

pub type RelationBehaviourFactoryCreator<T> = BehaviourFactoryCreator<RelationInstanceId, ReactiveRelation, T>;
