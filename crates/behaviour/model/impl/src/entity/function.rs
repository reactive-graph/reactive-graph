use std::sync::LazyLock;

use uuid::Uuid;

use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;

pub type EntityBehaviourFunctionsStorage<T> = LazyLock<BehaviourFunctionsReadOnlyView<Uuid, ReactiveEntity, T>>;
pub type EntityBehaviourFunctions<T> = BehaviourFunctions<Uuid, ReactiveEntity, T>;
