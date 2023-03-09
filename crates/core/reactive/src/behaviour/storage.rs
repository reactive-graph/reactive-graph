use std::sync::Arc;

use dashmap::DashMap;

use crate::model::BehaviourTypeId;
use crate::BehaviourFsm;

pub type BehaviourStorage<K, T> = DashMap<K, DashMap<BehaviourTypeId, Arc<dyn BehaviourFsm<T> + Send + Sync>>>;
