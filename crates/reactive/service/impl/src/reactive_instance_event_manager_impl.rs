use std::ops::Deref;

use async_trait::async_trait;
use dashmap::DashMap;
use serde_json::json;
use springtime_di::component_alias;
use springtime_di::Component;

use reactive_graph_graph::EntityInstance;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_graph::PropertyInstances;
use reactive_graph_graph::PropertyTypeDefinition;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_service_api::ReactiveInstanceEvent;
use reactive_graph_reactive_service_api::ReactiveInstanceEventManager;
use reactive_graph_reactive_service_api::ReactiveInstanceEventTypes;
use reactive_graph_reactive_service_api::REACTIVE_INSTANCE_EVENT_PROPERTY_LABEL;
use reactive_graph_runtime_model::EventProperties::EVENT;
use reactive_graph_runtime_model::ENTITY_TYPE_SYSTEM_EVENT;

#[derive(Component)]
pub struct ReactiveInstanceEventManagerImpl {
    #[component(default = "DashMap::new")]
    event_instances: DashMap<ReactiveInstanceEventTypes, ReactiveEntity>,
}

#[async_trait]
#[component_alias]
impl ReactiveInstanceEventManager for ReactiveInstanceEventManagerImpl {
    fn emit_event(&self, event: ReactiveInstanceEvent) {
        let Some(entity_instance) = self.get_reactive_instance_event_instance((&event).into()) else {
            return;
        };
        match event {
            ReactiveInstanceEvent::EntityInstanceCreated(id)
            | ReactiveInstanceEvent::EntityInstanceDeleted(id)
            | ReactiveInstanceEvent::FlowInstanceCreated(id)
            | ReactiveInstanceEvent::FlowInstanceDeleted(id) => entity_instance.set(EVENT.property_name(), json!(id)),
            ReactiveInstanceEvent::RelationInstanceCreated(relation_instance_id) | ReactiveInstanceEvent::RelationInstanceDeleted(relation_instance_id) => {
                entity_instance.set(EVENT.property_name(), json!(relation_instance_id))
            }
        }
    }

    fn get_reactive_instance_event_instances(&self) -> Vec<ReactiveEntity> {
        self.event_instances.iter().map(|reactive_entity| reactive_entity.value().clone()).collect()
    }

    fn get_reactive_instance_event_instance(&self, event_type: ReactiveInstanceEventTypes) -> Option<ReactiveEntity> {
        self.event_instances.get(&event_type).map(|reactive_entity| reactive_entity.value().clone())
    }
}

impl ReactiveInstanceEventManagerImpl {
    pub(crate) fn create_event_instances(&self) {
        self.event_instances.insert(
            ReactiveInstanceEventTypes::EntityInstanceCreated,
            self.create_event_instance("/io/reactive-graph/event/instance/entity/created"),
        );
        self.event_instances.insert(
            ReactiveInstanceEventTypes::EntityInstanceDeleted,
            self.create_event_instance("/io/reactive-graph/event/instance/entity/deleted"),
        );
        self.event_instances.insert(
            ReactiveInstanceEventTypes::RelationInstanceCreated,
            self.create_event_instance("/io/reactive-graph/event/instance/relation/created"),
        );
        self.event_instances.insert(
            ReactiveInstanceEventTypes::RelationInstanceDeleted,
            self.create_event_instance("/io/reactive-graph/event/instance/relation/deleted"),
        );
        self.event_instances.insert(
            ReactiveInstanceEventTypes::FlowInstanceCreated,
            self.create_event_instance("/io/reactive-graph/event/instance/flow/created"),
        );
        self.event_instances.insert(
            ReactiveInstanceEventTypes::FlowInstanceDeleted,
            self.create_event_instance("/io/reactive-graph/event/instance/flow/deleted"),
        );
    }

    pub(crate) fn create_event_instance<S: Into<String>>(&self, label: S) -> ReactiveEntity {
        EntityInstance::builder()
            .ty(ENTITY_TYPE_SYSTEM_EVENT.deref())
            .properties(
                PropertyInstances::new()
                    .property(REACTIVE_INSTANCE_EVENT_PROPERTY_LABEL, json!(label.into()))
                    .property(EVENT.property_name(), json!(false)),
            )
            .build()
            .into()
    }

    pub(crate) fn delete_event_instances(&self) {}
}

#[async_trait]
impl Lifecycle for ReactiveInstanceEventManagerImpl {
    async fn post_init(&self) {
        self.create_event_instances();
    }

    async fn pre_shutdown(&self) {
        self.delete_event_instances();
    }
}
