use std::sync::LazyLock;

use inexor_rgf_behaviour_api::prelude::*;
use inexor_rgf_graph::RelationInstanceId;
use inexor_rgf_reactive::ReactiveRelation;

pub type RelationBehaviourFunctionsStorage<T> = LazyLock<BehaviourFunctionsReadOnlyView<RelationInstanceId, ReactiveRelation, T>>;
pub type RelationBehaviourFunctions<T> = BehaviourFunctions<RelationInstanceId, ReactiveRelation, T>;
