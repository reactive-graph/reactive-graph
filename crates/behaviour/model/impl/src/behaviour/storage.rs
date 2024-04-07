use std::sync::Arc;

use dashmap::DashMap;

use reactive_graph_behaviour_model_api::prelude::*;

pub type BehaviourStorage<ID, T> = DashMap<ID, DashMap<BehaviourTypeId, Arc<dyn BehaviourFsm<ID, T> + Send + Sync>>>;
