use std::sync::Arc;

use dashmap::DashMap;
use indradb::EdgeKey;

use crate::model::BehaviourTypeId;
use crate::model::ReactiveRelationInstance;
use crate::BehaviourFsm;
use crate::BehaviourStorage;

pub struct RelationBehaviourStorage(BehaviourStorage<EdgeKey, ReactiveRelationInstance>);

impl RelationBehaviourStorage {
    pub fn new() -> Self {
        RelationBehaviourStorage(DashMap::new())
    }

    pub fn insert(
        &self,
        key: EdgeKey,
        ty: BehaviourTypeId,
        behaviour: Arc<dyn BehaviourFsm<ReactiveRelationInstance> + Send + Sync>,
    ) -> Option<Arc<dyn BehaviourFsm<ReactiveRelationInstance> + Send + Sync>> {
        if !self.0.contains_key(&key) {
            self.0.insert(key.clone(), DashMap::new());
        }
        if let Some(instance_behaviours) = self.0.get(&key) {
            return instance_behaviours.value().insert(ty, behaviour);
        }
        None
    }

    pub fn remove(&self, key: &EdgeKey, ty: &BehaviourTypeId) -> Option<(BehaviourTypeId, Arc<dyn BehaviourFsm<ReactiveRelationInstance> + Send + Sync>)> {
        if let Some(instance_behaviours) = self.0.get(key) {
            return instance_behaviours.value().remove(ty);
        }
        None
    }

    pub fn remove_all(&self, key: &EdgeKey) {
        self.0.remove(key);
    }
}

impl Default for RelationBehaviourStorage {
    fn default() -> Self {
        Self::new()
    }
}
