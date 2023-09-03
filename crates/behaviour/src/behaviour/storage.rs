use std::sync::Arc;

use dashmap::DashMap;

use crate::reactive::BehaviourTypeId;
use crate::BehaviourFsm;

pub type BehaviourStorage<ID, T> = DashMap<ID, DashMap<BehaviourTypeId, Arc<dyn BehaviourFsm<ID, T> + Send + Sync>>>;
