use std::sync::Arc;

use dashmap::DashMap;

use inexor_rgf_behaviour_api::prelude::*;

pub type BehaviourStorage<ID, T> = DashMap<ID, DashMap<BehaviourTypeId, Arc<dyn BehaviourFsm<ID, T> + Send + Sync>>>;
