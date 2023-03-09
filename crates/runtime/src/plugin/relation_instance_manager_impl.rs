use std::sync::Arc;

use indradb::EdgeKey;
use uuid::Uuid;

use crate::api::ComponentManager;
use crate::api::ReactiveRelationInstanceManager;
use crate::api::RelationTypeManager;
use crate::model::BehaviourTypeId;
use crate::model::ComponentTypeId;
use crate::model::ReactiveRelationInstance;
use crate::model::RelationInstance;
use crate::model::RelationTypeId;
use crate::plugins::relation_instance_manager::ReactiveRelationInstanceComponentAddError;
use crate::plugins::relation_instance_manager::RelationInstanceCreationError;
use crate::plugins::RelationInstanceManager;

pub struct RelationInstanceManagerImpl {
    component_manager: Arc<dyn ComponentManager>,
    relation_type_manager: Arc<dyn RelationTypeManager>,
    reactive_relation_instance_manager: Arc<dyn ReactiveRelationInstanceManager>,
}

impl RelationInstanceManagerImpl {
    pub fn new(
        component_manager: Arc<dyn ComponentManager>,
        relation_type_manager: Arc<dyn RelationTypeManager>,
        reactive_relation_instance_manager: Arc<dyn ReactiveRelationInstanceManager>,
    ) -> Self {
        Self {
            component_manager,
            relation_type_manager,
            reactive_relation_instance_manager,
        }
    }
}
impl RelationInstanceManager for RelationInstanceManagerImpl {
    fn has(&self, edge_key: &EdgeKey) -> bool {
        self.reactive_relation_instance_manager.has(edge_key)
    }

    fn get(&self, edge_key: &EdgeKey) -> Option<Arc<ReactiveRelationInstance>> {
        self.reactive_relation_instance_manager.get(edge_key)
    }

    fn get_by_outbound_entity(&self, outbound_entity_id: Uuid) -> Vec<Arc<ReactiveRelationInstance>> {
        self.reactive_relation_instance_manager.get_by_outbound_entity(outbound_entity_id)
    }

    fn get_by_inbound_entity(&self, inbound_entity_id: Uuid) -> Vec<Arc<ReactiveRelationInstance>> {
        self.reactive_relation_instance_manager.get_by_inbound_entity(inbound_entity_id)
    }

    fn get_all(&self) -> Vec<Arc<ReactiveRelationInstance>> {
        self.reactive_relation_instance_manager.get_all()
    }

    fn get_by_type(&self, ty: &RelationTypeId) -> Vec<Arc<ReactiveRelationInstance>> {
        self.reactive_relation_instance_manager.get_by_type(ty)
    }

    fn get_by_namespace(&self, namespace: &str) -> Vec<Arc<ReactiveRelationInstance>> {
        self.reactive_relation_instance_manager.get_by_namespace(namespace)
    }

    fn get_keys(&self) -> Vec<EdgeKey> {
        self.reactive_relation_instance_manager.get_keys()
    }

    fn count(&self) -> usize {
        self.reactive_relation_instance_manager.count()
    }

    fn count_by_type(&self, ty: &RelationTypeId) -> usize {
        self.reactive_relation_instance_manager.count_by_type(ty)
    }

    fn count_by_component(&self, component: &ComponentTypeId) -> usize {
        self.reactive_relation_instance_manager.count_by_component(component)
    }

    fn count_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) -> usize {
        self.reactive_relation_instance_manager.count_by_behaviour(behaviour_ty)
    }

    fn create(&self, relation_instance: RelationInstance) -> Result<Arc<ReactiveRelationInstance>, RelationInstanceCreationError> {
        let relation_ty = relation_instance.relation_type_id();
        let relation_type = self.relation_type_manager.get(&relation_ty);
        // let relation_type = self.relation_type_manager.get_starts_with(&relation_instance.ty);
        match relation_type {
            Some(relation_type) => {
                let edge_key = relation_instance.get_key();
                if self.reactive_relation_instance_manager.has(&edge_key) {
                    if let Some(reactive_relation_instance) = self.reactive_relation_instance_manager.get(&edge_key) {
                        return Ok(reactive_relation_instance);
                    }
                }
                let mut relation_instance = relation_instance;
                // Add properties from relation type if not existing
                for property in relation_type.properties.iter() {
                    if !relation_instance.properties.contains_key(&property.name) {
                        relation_instance.properties.insert(property.name.clone(), property.data_type.default_value());
                    }
                }
                // Add properties from components if not existing
                for component_name in relation_type.components.iter() {
                    if let Some(component) = self.component_manager.get(component_name) {
                        for property in component.properties {
                            if !relation_instance.properties.contains_key(&property.name) {
                                relation_instance.properties.insert(property.name.clone(), property.data_type.default_value());
                            }
                        }
                    }
                }
                let reactive_relation_instance = self.reactive_relation_instance_manager.create_reactive_instance(relation_instance);
                match reactive_relation_instance {
                    Ok(reactive_relation_instance) => Ok(reactive_relation_instance),
                    Err(_) => Err(RelationInstanceCreationError::Failed),
                }
            }
            None => Err(RelationInstanceCreationError::Failed),
        }
    }

    fn add_component(&self, edge_key: &EdgeKey, component: &ComponentTypeId) -> Result<(), ReactiveRelationInstanceComponentAddError> {
        self.reactive_relation_instance_manager
            .add_component(edge_key, component)
            .map_err(|_| ReactiveRelationInstanceComponentAddError::Failed)
    }

    fn remove_component(&self, edge_key: &EdgeKey, component: &ComponentTypeId) {
        self.reactive_relation_instance_manager.remove_component(edge_key, component);
    }

    fn delete(&self, edge_key: &EdgeKey) -> bool {
        self.reactive_relation_instance_manager.delete(edge_key)
    }
}
