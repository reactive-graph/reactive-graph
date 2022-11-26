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

    pub fn get(&self, key: &EdgeKey, ty: &BehaviourTypeId) -> Option<Arc<dyn BehaviourFsm<ReactiveRelationInstance> + Send + Sync>> {
        if let Some(instance_behaviours) = self.0.get(key) {
            if let Some(fsm) = instance_behaviours.value().get(ty) {
                return Some(fsm.value().clone());
            }
        }
        None
    }

    pub fn get_by_behaviour(&self, ty: &BehaviourTypeId) -> Vec<Arc<dyn BehaviourFsm<ReactiveRelationInstance> + Send + Sync>> {
        let mut fsms = vec![];
        for instance_behaviours in self.0.iter() {
            if let Some(fsm) = instance_behaviours.value().get(ty) {
                fsms.push(fsm.value().clone());
            }
        }
        fsms
    }
}

impl Default for RelationBehaviourStorage {
    fn default() -> Self {
        Self::new()
    }
}
