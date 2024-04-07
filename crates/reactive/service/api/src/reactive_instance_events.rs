use uuid::Uuid;

use reactive_graph_graph::RelationInstanceId;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum ReactiveInstanceEventTypes {
    EntityInstanceCreated,
    EntityInstanceDeleted,
    RelationInstanceCreated,
    RelationInstanceDeleted,
    FlowInstanceCreated,
    FlowInstanceDeleted,
}

pub enum ReactiveInstanceEvent {
    EntityInstanceCreated(Uuid),
    EntityInstanceDeleted(Uuid),
    RelationInstanceCreated(RelationInstanceId),
    RelationInstanceDeleted(RelationInstanceId),
    FlowInstanceCreated(Uuid),
    FlowInstanceDeleted(Uuid),
}

impl From<&ReactiveInstanceEvent> for ReactiveInstanceEventTypes {
    fn from(event: &ReactiveInstanceEvent) -> Self {
        match event {
            ReactiveInstanceEvent::EntityInstanceCreated(_) => ReactiveInstanceEventTypes::EntityInstanceCreated,
            ReactiveInstanceEvent::EntityInstanceDeleted(_) => ReactiveInstanceEventTypes::EntityInstanceDeleted,
            ReactiveInstanceEvent::RelationInstanceCreated(_) => ReactiveInstanceEventTypes::RelationInstanceCreated,
            ReactiveInstanceEvent::RelationInstanceDeleted(_) => ReactiveInstanceEventTypes::RelationInstanceDeleted,
            ReactiveInstanceEvent::FlowInstanceCreated(_) => ReactiveInstanceEventTypes::FlowInstanceCreated,
            ReactiveInstanceEvent::FlowInstanceDeleted(_) => ReactiveInstanceEventTypes::FlowInstanceDeleted,
        }
    }
}
