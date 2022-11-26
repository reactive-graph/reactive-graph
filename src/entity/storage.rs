use std::sync::Arc;

use dashmap::DashMap;
use uuid::Uuid;

use crate::model::BehaviourTypeId;
use crate::model::ReactiveEntityInstance;
use crate::BehaviourFsm;
use crate::BehaviourStorage;

pub struct EntityBehaviourStorage(BehaviourStorage<Uuid, ReactiveEntityInstance>);

impl EntityBehaviourStorage {
    pub fn new() -> Self {
        EntityBehaviourStorage(DashMap::new())
    }

    pub fn insert(
        &self,
        key: Uuid,
        ty: BehaviourTypeId,
        behaviour: Arc<dyn BehaviourFsm<ReactiveEntityInstance> + Send + Sync>,
    ) -> Option<Arc<dyn BehaviourFsm<ReactiveEntityInstance> + Send + Sync>> {
        if !self.0.contains_key(&key) {
            self.0.insert(key, DashMap::new());
        }
        if let Some(instance_behaviours) = self.0.get(&key) {
            return instance_behaviours.value().insert(ty, behaviour);
        }
        None
    }

    pub fn remove(&self, key: &Uuid, ty: &BehaviourTypeId) -> Option<(BehaviourTypeId, Arc<dyn BehaviourFsm<ReactiveEntityInstance> + Send + Sync>)> {
        if let Some(instance_behaviours) = self.0.get(key) {
            return instance_behaviours.value().remove(ty);
        }
        None
    }

    pub fn remove_all(&self, key: &Uuid) {
        self.0.remove(key);
    }

    pub fn get(&self, key: &Uuid, ty: &BehaviourTypeId) -> Option<Arc<dyn BehaviourFsm<ReactiveEntityInstance> + Send + Sync>> {
        if let Some(instance_behaviours) = self.0.get(key) {
            if let Some(fsm) = instance_behaviours.value().get(ty) {
                return Some(fsm.value().clone());
            }
        }
        None
    }

    pub fn get_by_behaviour(&self, ty: &BehaviourTypeId) -> Vec<Arc<dyn BehaviourFsm<ReactiveEntityInstance> + Send + Sync>> {
        let mut fsms = vec![];
        for instance_behaviours in self.0.iter() {
            if let Some(fsm) = instance_behaviours.value().get(ty) {
                fsms.push(fsm.value().clone());
            }
        }
        fsms
    }
}

impl Default for EntityBehaviourStorage {
    fn default() -> Self {
        Self::new()
    }
}
