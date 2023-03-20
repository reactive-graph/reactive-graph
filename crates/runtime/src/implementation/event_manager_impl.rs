use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use serde_json::json;

use crate::api::Lifecycle;
use crate::api::SystemEventManager;
use crate::api::SYSTEM_EVENT_PROPERTY_LABEL;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::di::*;
use crate::model::ComponentTypeId;
use crate::model::ExtensionTypeId;
use crate::model::PropertyInstanceSetter;
use crate::model::PropertyTypeDefinition;
use crate::model::ReactiveEntityInstance;
use crate::model::TypeDefinition;
use crate::model::TypeDefinitionComponent;
use crate::model::TypeDefinitionExtension;
use crate::model::TypeDefinitionGetter;
use crate::model::TypeDefinitionProperty;
use crate::model_runtime::EventProperties::EVENT;
use crate::model_runtime::ENTITY_TYPE_SYSTEM_EVENT;
use crate::plugins::SystemEvent;
use crate::plugins::SystemEventTypes;

#[wrapper]
pub struct SystemEventInstanceStorage(RwLock<HashMap<SystemEventTypes, Arc<ReactiveEntityInstance>>>);

#[provides]
fn create_shutdown_state() -> SystemEventInstanceStorage {
    SystemEventInstanceStorage(RwLock::new(HashMap::new()))
}

#[component]
pub struct SystemEventManagerImpl {
    system_event_instances: SystemEventInstanceStorage,
}

#[async_trait]
#[provides]
impl SystemEventManager for SystemEventManagerImpl {
    fn emit_event(&self, event: SystemEvent) {
        let Some(entity_instance) = self.get_system_event_instance((&event).into()) else {
            return;
        };
        match event {
            SystemEvent::ComponentCreated(ty) | SystemEvent::ComponentDeleted(ty) => {
                self.propagate_type_definition_event(entity_instance, ty.type_definition());
            }
            SystemEvent::EntityTypeCreated(ty) | SystemEvent::EntityTypeDeleted(ty) => {
                self.propagate_type_definition_event(entity_instance, ty.type_definition());
            }
            SystemEvent::RelationTypeCreated(ty) | SystemEvent::RelationTypeDeleted(ty) => {
                self.propagate_type_definition_event(entity_instance, ty.type_definition());
            }
            SystemEvent::FlowTypeCreated(ty) | SystemEvent::FlowTypeUpdated(ty) | SystemEvent::FlowTypeDeleted(ty) => {
                self.propagate_type_definition_event(entity_instance, ty.type_definition());
            }
            SystemEvent::EntityTypeComponentAdded(ty, component_ty) | SystemEvent::EntityTypeComponentRemoved(ty, component_ty) => {
                self.propagate_type_definition_component_event(entity_instance, ty.type_definition(), &component_ty);
            }
            SystemEvent::RelationTypeComponentAdded(ty, component_ty) | SystemEvent::RelationTypeComponentRemoved(ty, component_ty) => {
                self.propagate_type_definition_component_event(entity_instance, ty.type_definition(), &component_ty);
            }
            SystemEvent::ComponentPropertyAdded(ty, property_name) | SystemEvent::ComponentPropertyRemoved(ty, property_name) => {
                self.propagate_type_definition_property_event(entity_instance, ty.type_definition(), property_name);
            }
            SystemEvent::EntityTypePropertyAdded(ty, property_name) | SystemEvent::EntityTypePropertyRemoved(ty, property_name) => {
                self.propagate_type_definition_property_event(entity_instance, ty.type_definition(), property_name);
            }
            SystemEvent::RelationTypePropertyAdded(ty, property_name) | SystemEvent::RelationTypePropertyRemoved(ty, property_name) => {
                self.propagate_type_definition_property_event(entity_instance, ty.type_definition(), property_name);
            }
            SystemEvent::ComponentExtensionAdded(ty, extension_ty) | SystemEvent::ComponentExtensionRemoved(ty, extension_ty) => {
                self.propagate_type_definition_extension_event(entity_instance, ty.type_definition(), extension_ty);
            }
            SystemEvent::EntityTypeExtensionAdded(ty, extension_ty) | SystemEvent::EntityTypeExtensionRemoved(ty, extension_ty) => {
                self.propagate_type_definition_extension_event(entity_instance, ty.type_definition(), extension_ty);
            }
            SystemEvent::RelationTypeExtensionAdded(ty, extension_ty) | SystemEvent::RelationTypeExtensionRemoved(ty, extension_ty) => {
                self.propagate_type_definition_extension_event(entity_instance, ty.type_definition(), extension_ty);
            }
            SystemEvent::TypeSystemChanged => entity_instance.set(EVENT.property_name(), json!(true)),
            SystemEvent::EntityInstanceCreated(id)
            | SystemEvent::EntityInstanceDeleted(id)
            | SystemEvent::FlowInstanceCreated(id)
            | SystemEvent::FlowInstanceDeleted(id) => entity_instance.set(EVENT.property_name(), json!(id)),
            SystemEvent::RelationInstanceCreated(edge_key) | SystemEvent::RelationInstanceDeleted(edge_key) => {
                entity_instance.set(EVENT.property_name(), json!(edge_key))
            }
        }
    }

    fn get_system_event_instances(&self) -> Vec<Arc<ReactiveEntityInstance>> {
        let reader = self.system_event_instances.0.read().unwrap();
        reader.values().cloned().collect()
    }

    fn get_system_event_instance(&self, event_type: SystemEventTypes) -> Option<Arc<ReactiveEntityInstance>> {
        let reader = self.system_event_instances.0.read().unwrap();
        reader.get(&event_type).cloned()
    }
}

impl SystemEventManagerImpl {
    fn propagate_type_definition_event(&self, entity_instance: Arc<ReactiveEntityInstance>, type_definition: TypeDefinition) {
        if let Ok(value) = serde_json::to_value(type_definition) {
            entity_instance.set(EVENT.property_name(), value);
            // Also emit event that the type system has been changed
            self.emit_event(SystemEvent::TypeSystemChanged);
        };
    }

    fn propagate_type_definition_component_event<T: Into<TypeDefinition>>(
        &self,
        entity_instance: Arc<ReactiveEntityInstance>,
        type_definition: T,
        component_ty: &ComponentTypeId,
    ) {
        if let Ok(v) = TypeDefinitionComponent::new(type_definition, component_ty.clone()).try_into() {
            entity_instance.set(EVENT.property_name(), v);
        };
        // Also emit event that the type system has been changed
        self.emit_event(SystemEvent::TypeSystemChanged);
    }

    fn propagate_type_definition_property_event(&self, entity_instance: Arc<ReactiveEntityInstance>, type_definition: TypeDefinition, property_name: String) {
        if let Ok(v) = TypeDefinitionProperty::new(type_definition, property_name).try_into() {
            entity_instance.set(EVENT.property_name(), v);
        };
        self.emit_event(SystemEvent::TypeSystemChanged);
    }

    fn propagate_type_definition_extension_event(
        &self,
        entity_instance: Arc<ReactiveEntityInstance>,
        type_definition: TypeDefinition,
        extension_ty: ExtensionTypeId,
    ) {
        if let Ok(v) = TypeDefinitionExtension::new(type_definition, extension_ty).try_into() {
            entity_instance.set(EVENT.property_name(), v);
        };
        self.emit_event(SystemEvent::TypeSystemChanged);
    }

    pub(crate) fn create_system_event_instances(&self) {
        let mut writer = self.system_event_instances.0.write().unwrap();
        writer.insert(
            SystemEventTypes::ComponentCreated,
            self.create_system_event_instance("/org/inexor/event/type/component/created"),
        );
        writer.insert(
            SystemEventTypes::ComponentPropertyAdded,
            self.create_system_event_instance("/org/inexor/event/type/component/property/added"),
        );
        writer.insert(
            SystemEventTypes::ComponentPropertyRemoved,
            self.create_system_event_instance("/org/inexor/event/type/component/property/removed"),
        );
        writer.insert(
            SystemEventTypes::ComponentExtensionAdded,
            self.create_system_event_instance("/org/inexor/event/type/component/extension/added"),
        );
        writer.insert(
            SystemEventTypes::ComponentExtensionRemoved,
            self.create_system_event_instance("/org/inexor/event/type/component/extension/removed"),
        );
        writer.insert(
            SystemEventTypes::ComponentDeleted,
            self.create_system_event_instance("/org/inexor/event/type/component/deleted"),
        );
        writer.insert(
            SystemEventTypes::EntityTypeCreated,
            self.create_system_event_instance("/org/inexor/event/type/entity/created"),
        );
        writer.insert(
            SystemEventTypes::EntityTypeComponentAdded,
            self.create_system_event_instance("/org/inexor/event/type/entity/component/added"),
        );
        writer.insert(
            SystemEventTypes::EntityTypeComponentRemoved,
            self.create_system_event_instance("/org/inexor/event/type/entity/component/removed"),
        );
        writer.insert(
            SystemEventTypes::EntityTypePropertyAdded,
            self.create_system_event_instance("/org/inexor/event/type/entity/property/added"),
        );
        writer.insert(
            SystemEventTypes::EntityTypePropertyRemoved,
            self.create_system_event_instance("/org/inexor/event/type/entity/property/removed"),
        );
        writer.insert(
            SystemEventTypes::EntityTypeExtensionAdded,
            self.create_system_event_instance("/org/inexor/event/type/entity/extension/added"),
        );
        writer.insert(
            SystemEventTypes::EntityTypeExtensionRemoved,
            self.create_system_event_instance("/org/inexor/event/type/entity/extension/removed"),
        );
        writer.insert(
            SystemEventTypes::EntityTypeDeleted,
            self.create_system_event_instance("/org/inexor/event/type/entity/deleted"),
        );
        writer.insert(
            SystemEventTypes::RelationTypeCreated,
            self.create_system_event_instance("/org/inexor/event/type/relation/created"),
        );
        writer.insert(
            SystemEventTypes::RelationTypeComponentAdded,
            self.create_system_event_instance("/org/inexor/event/type/relation/component/added"),
        );
        writer.insert(
            SystemEventTypes::RelationTypeComponentRemoved,
            self.create_system_event_instance("/org/inexor/event/type/relation/component/removed"),
        );
        writer.insert(
            SystemEventTypes::RelationTypePropertyAdded,
            self.create_system_event_instance("/org/inexor/event/type/relation/property/added"),
        );
        writer.insert(
            SystemEventTypes::RelationTypePropertyRemoved,
            self.create_system_event_instance("/org/inexor/event/type/relation/property/removed"),
        );
        writer.insert(
            SystemEventTypes::RelationTypeExtensionAdded,
            self.create_system_event_instance("/org/inexor/event/type/relation/extension/added"),
        );
        writer.insert(
            SystemEventTypes::RelationTypeExtensionRemoved,
            self.create_system_event_instance("/org/inexor/event/type/relation/extension/removed"),
        );
        writer.insert(
            SystemEventTypes::RelationTypeDeleted,
            self.create_system_event_instance("/org/inexor/event/type/relation/deleted"),
        );
        writer.insert(SystemEventTypes::FlowTypeCreated, self.create_system_event_instance("/org/inexor/event/type/flow/created"));
        writer.insert(SystemEventTypes::FlowTypeUpdated, self.create_system_event_instance("/org/inexor/event/type/flow/updated"));
        writer.insert(SystemEventTypes::FlowTypeDeleted, self.create_system_event_instance("/org/inexor/event/type/flow/deleted"));
        writer.insert(SystemEventTypes::TypeSystemChanged, self.create_system_event_instance("/org/inexor/event/type/changed"));
        writer.insert(
            SystemEventTypes::EntityInstanceCreated,
            self.create_system_event_instance("/org/inexor/event/instance/entity/created"),
        );
        writer.insert(
            SystemEventTypes::EntityInstanceDeleted,
            self.create_system_event_instance("/org/inexor/event/instance/entity/deleted"),
        );
        writer.insert(
            SystemEventTypes::RelationInstanceCreated,
            self.create_system_event_instance("/org/inexor/event/instance/relation/created"),
        );
        writer.insert(
            SystemEventTypes::RelationInstanceDeleted,
            self.create_system_event_instance("/org/inexor/event/instance/relation/deleted"),
        );
        writer.insert(
            SystemEventTypes::FlowInstanceCreated,
            self.create_system_event_instance("/org/inexor/event/instance/flow/created"),
        );
        writer.insert(
            SystemEventTypes::FlowInstanceDeleted,
            self.create_system_event_instance("/org/inexor/event/instance/flow/deleted"),
        );
    }

    pub(crate) fn create_system_event_instance<S: Into<String>>(&self, label: S) -> Arc<ReactiveEntityInstance> {
        ReactiveEntityInstanceBuilder::new(ENTITY_TYPE_SYSTEM_EVENT.clone())
            .property(SYSTEM_EVENT_PROPERTY_LABEL, json!(label.into()))
            .property(&EVENT.property_name(), json!(false))
            .build()
    }

    pub(crate) fn delete_system_event_instances(&self) {
        // let mut writer = self.event_instances.0.write().unwrap();
    }
}

#[async_trait]
impl Lifecycle for SystemEventManagerImpl {
    async fn post_init(&self) {
        self.create_system_event_instances();
    }

    async fn pre_shutdown(&self) {
        self.delete_system_event_instances();
    }
}
