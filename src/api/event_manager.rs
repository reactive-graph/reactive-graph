use std::sync::Arc;

use async_trait::async_trait;
use indradb::EdgeKey;
use uuid::Uuid;

use crate::api::Lifecycle;
use crate::model::ReactiveEntityInstance;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum SystemEventTypes {
    ComponentCreated,
    ComponentDeleted,
    EntityTypeCreated,
    EntityTypeDeleted,
    RelationTypeCreated,
    RelationTypeDeleted,
    EntityInstanceCreated,
    EntityInstanceDeleted,
    RelationInstanceCreated,
    RelationInstanceDeleted,
    FlowCreated,
    FlowDeleted,
}

pub enum SystemEvent {
    ComponentCreated(String),
    ComponentDeleted(String),
    EntityTypeCreated(String),
    EntityTypeDeleted(String),
    RelationTypeCreated(String),
    RelationTypeDeleted(String),
    EntityInstanceCreated(Uuid),
    EntityInstanceDeleted(Uuid),
    RelationInstanceCreated(EdgeKey),
    RelationInstanceDeleted(EdgeKey),
    FlowCreated(Uuid),
    FlowDeleted(Uuid),
}

#[async_trait]
pub trait SystemEventManager: Send + Sync + Lifecycle {
    fn emit_event(&self, event: SystemEvent);

    fn get_system_event_instances(&self) -> Vec<Arc<ReactiveEntityInstance>>;
}
