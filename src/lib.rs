use std::sync::Arc;

use inexor_ecs_model as model;
use model::{Component, EntityType, RelationType};
use inexor_ecs_model::{ReactiveEntityInstance, ReactiveRelationInstance, Flow};
use uuid::Uuid;
use indradb::EdgeKey;

#[derive(Debug)]
pub struct SubsystemError;

pub trait Subsystem: Send + Sync {
    fn init(&self);

    fn post_init(&self);

    fn shutdown(&self);

    fn get_component_provider(&self) -> Result<Arc<dyn ComponentProvider>, SubsystemError>;

    fn get_entity_type_provider(&self) -> Result<Arc<dyn EntityTypeProvider>, SubsystemError>;

    fn get_relation_type_provider(&self) -> Result<Arc<dyn RelationTypeProvider>, SubsystemError>;

    fn get_entity_behaviour_provider(&self) -> Result<Arc<dyn EntityBehaviourProvider>, SubsystemError>;

    fn get_relation_behaviour_provider(&self) -> Result<Arc<dyn RelationBehaviourProvider>, SubsystemError>;

    fn get_flow_provider(&self) -> Result<Arc<dyn FlowProvider>, SubsystemError>;
}

pub trait ComponentProvider: Send + Sync {
    fn get_components(&self) -> Vec<Component>;
}

pub trait EntityTypeProvider: Send + Sync {
    fn get_entity_types(&self) -> Vec<EntityType>;
}

pub trait RelationTypeProvider: Send + Sync {
    fn get_relation_types(&self) -> Vec<RelationType>;
}

pub trait EntityBehaviourProvider: Send + Sync {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_behaviours_by_id(&self, id: Uuid);
}

pub trait RelationBehaviourProvider: Send + Sync {
    fn add_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_behaviours_by_key(&self, edge_key: EdgeKey);
}

pub trait FlowProvider: Send + Sync {
    fn get_flows(&self) -> Vec<Flow>;
}
