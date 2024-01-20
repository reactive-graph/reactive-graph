use inexor_rgf_behaviour_model_api::prelude::*;

use inexor_rgf_graph::RelationInstanceId;
use inexor_rgf_reactive_model_impl::ReactiveRelation;

pub trait RelationBehaviourFactory: BehaviourFactory<RelationInstanceId, ReactiveRelation> {}

pub type RelationBehaviourFactoryCreator<T> = BehaviourFactoryCreator<RelationInstanceId, ReactiveRelation, T>;
