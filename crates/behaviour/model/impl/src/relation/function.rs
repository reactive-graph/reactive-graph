use std::sync::LazyLock;

use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_graph::RelationInstanceId;
use reactive_graph_reactive_model_impl::ReactiveRelation;

pub type RelationBehaviourFunctionsStorage<T> = LazyLock<BehaviourFunctionsReadOnlyView<RelationInstanceId, ReactiveRelation, T>>;
pub type RelationBehaviourFunctions<T> = BehaviourFunctions<RelationInstanceId, ReactiveRelation, T>;
