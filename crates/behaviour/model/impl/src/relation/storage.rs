use std::sync::Arc;

use dashmap::DashMap;

use reactive_graph_behaviour_model_api::prelude::*;

use crate::BehaviourStorage;
use reactive_graph_graph::RelationInstanceId;
use reactive_graph_reactive_model_impl::ReactiveRelation;

pub struct RelationBehaviourStorage(BehaviourStorage<RelationInstanceId, ReactiveRelation>);

impl RelationBehaviourStorage {
    pub fn new() -> Self {
        RelationBehaviourStorage(DashMap::new())
    }

    pub fn insert(
        &self,
        key: RelationInstanceId,
        ty: BehaviourTypeId,
        behaviour: Arc<dyn BehaviourFsm<RelationInstanceId, ReactiveRelation> + Send + Sync>,
    ) -> Option<Arc<dyn BehaviourFsm<RelationInstanceId, ReactiveRelation> + Send + Sync>> {
        if !self.0.contains_key(&key) {
            self.0.insert(key.clone(), DashMap::new());
        }
        if let Some(instance_behaviours) = self.0.get(&key) {
            return instance_behaviours.value().insert(ty, behaviour);
        }
        None
    }

    pub fn remove(
        &self,
        key: &RelationInstanceId,
        ty: &BehaviourTypeId,
    ) -> Option<(BehaviourTypeId, Arc<dyn BehaviourFsm<RelationInstanceId, ReactiveRelation> + Send + Sync>)> {
        if let Some(instance_behaviours) = self.0.get(key) {
            return instance_behaviours.value().remove(ty);
        }
        None
    }

    /// Removes all behaviours of the given behaviour type.
    pub fn remove_by_behaviour(&self, ty: &BehaviourTypeId) {
        for instance_behaviours in self.0.iter_mut() {
            instance_behaviours.value().remove(ty);
        }
    }

    pub fn remove_all(&self, key: &RelationInstanceId) {
        self.0.remove(key);
    }

    pub fn has(&self, key: &RelationInstanceId, ty: &BehaviourTypeId) -> bool {
        if let Some(instance_behaviours) = self.0.get(key) {
            return instance_behaviours.value().get(ty).is_some();
        }
        false
    }

    pub fn get(&self, key: &RelationInstanceId, ty: &BehaviourTypeId) -> Option<Arc<dyn BehaviourFsm<RelationInstanceId, ReactiveRelation> + Send + Sync>> {
        if let Some(instance_behaviours) = self.0.get(key) {
            if let Some(fsm) = instance_behaviours.value().get(ty) {
                return Some(fsm.value().clone());
            }
        }
        None
    }

    pub fn get_by_behaviour(&self, ty: &BehaviourTypeId) -> Vec<Arc<dyn BehaviourFsm<RelationInstanceId, ReactiveRelation> + Send + Sync>> {
        let mut fsms = vec![];
        for instance_behaviours in self.0.iter() {
            if let Some(fsm) = instance_behaviours.value().get(ty) {
                fsms.push(fsm.value().clone());
            }
        }
        fsms
    }

    pub fn get_behaviours_by_instance(&self, key: &RelationInstanceId) -> Vec<BehaviourTypeId> {
        if let Some(instance_behaviours) = self.0.get(key) {
            return instance_behaviours.value().iter().map(|b| b.key().clone()).collect();
        }
        Vec::new()
    }

    pub fn get_instances_by_behaviour(&self, ty: &BehaviourTypeId) -> Vec<ReactiveRelation> {
        let mut instances = vec![];
        for instance_behaviours in self.0.iter() {
            if let Some(inner) = instance_behaviours.value().get(ty) {
                instances.push(inner.value().get_reactive_instance().clone());
            }
        }
        instances
    }
}

impl Default for RelationBehaviourStorage {
    fn default() -> Self {
        Self::new()
    }
}
