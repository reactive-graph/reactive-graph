use reactive_graph_behaviour_model_api::prelude::*;

use reactive_graph_graph::RelationInstanceId;
use reactive_graph_reactive_model_impl::ReactiveRelation;

pub trait RelationBehaviourFactory: BehaviourFactory<RelationInstanceId, ReactiveRelation> {}

pub type RelationBehaviourFactoryCreator<T> = BehaviourFactoryCreator<RelationInstanceId, ReactiveRelation, T>;
