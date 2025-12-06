use std::ops::Deref;

use async_trait::async_trait;
use dashmap::DashMap;
use serde_json::json;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::EntityInstance;
use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_graph::PropertyInstances;
use reactive_graph_graph::TypeDefinition;
use reactive_graph_graph::TypeDefinitionComponent;
use reactive_graph_graph::TypeDefinitionExtension;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_graph::TypeDefinitionProperty;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_model_core::reactive_graph::core::event::EventProperties;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_type_system_api::TYPE_SYSTEM_EVENT_PROPERTY_LABEL;
use reactive_graph_type_system_api::TypeSystemEvent;
use reactive_graph_type_system_api::TypeSystemEventManager;
use reactive_graph_type_system_api::TypeSystemEventTypes;
use reactive_graph_type_system_model::reactive_graph::type_system::type_system_event::TYPE_SYSTEM_EVENT;

#[derive(Component)]
pub struct TypeSystemEventManagerImpl {
    #[component(default = "DashMap::new")]
    system_event_instances: DashMap<TypeSystemEventTypes, ReactiveEntity>,
}

#[async_trait]
#[component_alias]
impl TypeSystemEventManager for TypeSystemEventManagerImpl {
    fn emit_event(&self, event: TypeSystemEvent) {
        let Some(entity_instance) = self.get_type_system_event_instance((&event).into()) else {
            return;
        };
        match event {
            TypeSystemEvent::ComponentCreated(ty) | TypeSystemEvent::ComponentDeleted(ty) => {
                self.propagate_type_definition_event(entity_instance, ty.type_definition());
            }
            TypeSystemEvent::ComponentPropertyAdded(ty, property_name)
            | TypeSystemEvent::ComponentPropertyUpdated(ty, property_name)
            | TypeSystemEvent::ComponentPropertyRemoved(ty, property_name) => {
                self.propagate_type_definition_property_event(entity_instance, ty.type_definition(), property_name);
            }
            TypeSystemEvent::ComponentPropertyRenamed(ty, old_property_name, new_property_name) => {
                self.propagate_type_definition_property_renamed_event(entity_instance, ty.type_definition(), old_property_name, new_property_name);
            }
            TypeSystemEvent::ComponentExtensionAdded(ty, extension_ty)
            | TypeSystemEvent::ComponentExtensionUpdated(ty, extension_ty)
            | TypeSystemEvent::ComponentExtensionRemoved(ty, extension_ty) => {
                self.propagate_type_definition_extension_event(entity_instance, ty.type_definition(), extension_ty);
            }
            TypeSystemEvent::ComponentExtensionRenamed(ty, old_extension_ty, new_extension_ty) => {
                self.propagate_type_definition_extension_renamed_event(entity_instance, ty.type_definition(), old_extension_ty, new_extension_ty);
            }
            TypeSystemEvent::EntityTypeCreated(ty) | TypeSystemEvent::EntityTypeDeleted(ty) => {
                self.propagate_type_definition_event(entity_instance, ty.type_definition());
            }
            TypeSystemEvent::RelationTypeCreated(ty) | TypeSystemEvent::RelationTypeDeleted(ty) => {
                self.propagate_type_definition_event(entity_instance, ty.type_definition());
            }
            TypeSystemEvent::FlowTypeCreated(ty) | TypeSystemEvent::FlowTypeUpdated(ty) | TypeSystemEvent::FlowTypeDeleted(ty) => {
                self.propagate_type_definition_event(entity_instance, ty.type_definition());
            }
            TypeSystemEvent::EntityTypeComponentAdded(ty, component_ty) | TypeSystemEvent::EntityTypeComponentRemoved(ty, component_ty) => {
                self.propagate_type_definition_component_event(entity_instance, ty.type_definition(), &component_ty);
            }
            TypeSystemEvent::RelationTypeComponentAdded(ty, component_ty) | TypeSystemEvent::RelationTypeComponentRemoved(ty, component_ty) => {
                self.propagate_type_definition_component_event(entity_instance, ty.type_definition(), &component_ty);
            }
            TypeSystemEvent::EntityTypePropertyAdded(ty, property_name) | TypeSystemEvent::EntityTypePropertyRemoved(ty, property_name) => {
                self.propagate_type_definition_property_event(entity_instance, ty.type_definition(), property_name);
            }
            TypeSystemEvent::RelationTypePropertyAdded(ty, property_name) | TypeSystemEvent::RelationTypePropertyRemoved(ty, property_name) => {
                self.propagate_type_definition_property_event(entity_instance, ty.type_definition(), property_name);
            }
            TypeSystemEvent::EntityTypeExtensionAdded(ty, extension_ty) | TypeSystemEvent::EntityTypeExtensionRemoved(ty, extension_ty) => {
                self.propagate_type_definition_extension_event(entity_instance, ty.type_definition(), extension_ty);
            }
            TypeSystemEvent::RelationTypeExtensionAdded(ty, extension_ty) | TypeSystemEvent::RelationTypeExtensionRemoved(ty, extension_ty) => {
                self.propagate_type_definition_extension_event(entity_instance, ty.type_definition(), extension_ty);
            }
            TypeSystemEvent::TypeSystemChanged => entity_instance.set(EventProperties::EVENT.as_ref(), json!(true)),
            TypeSystemEvent::EntityTypeComponentRenamed(_, _, _) => {}
            TypeSystemEvent::EntityTypeComponentUpdated(_, _) => {}
            TypeSystemEvent::EntityTypePropertyRenamed(_, _, _) => {}
            TypeSystemEvent::EntityTypePropertyUpdated(_, _) => {}
            TypeSystemEvent::EntityTypeExtensionRenamed(_, _, _) => {}
            TypeSystemEvent::EntityTypeExtensionUpdated(_, _) => {}
            TypeSystemEvent::RelationTypeComponentRenamed(_, _, _) => {}
            TypeSystemEvent::RelationTypeComponentUpdated(_, _) => {}
            TypeSystemEvent::RelationTypePropertyRenamed(_, _, _) => {}
            TypeSystemEvent::RelationTypePropertyUpdated(_, _) => {}
            TypeSystemEvent::RelationTypeExtensionRenamed(_, _, _) => {}
            TypeSystemEvent::RelationTypeExtensionUpdated(_, _) => {}
        }
    }

    fn get_type_system_event_instances(&self) -> Vec<ReactiveEntity> {
        self.system_event_instances
            .iter()
            .map(|reactive_entity| reactive_entity.value().clone())
            .collect()
    }

    fn get_type_system_event_instance(&self, event_type: TypeSystemEventTypes) -> Option<ReactiveEntity> {
        self.system_event_instances
            .get(&event_type)
            .map(|reactive_entity| reactive_entity.value().clone())
    }
}

impl TypeSystemEventManagerImpl {
    fn propagate_type_definition_event(&self, entity_instance: ReactiveEntity, type_definition: TypeDefinition) {
        if let Ok(value) = serde_json::to_value(type_definition) {
            entity_instance.set(EventProperties::EVENT.as_ref(), value);
            // Also emit event that the type system has been changed
            self.emit_event(TypeSystemEvent::TypeSystemChanged);
        };
    }

    fn propagate_type_definition_component_event<T: Into<TypeDefinition>>(
        &self,
        entity_instance: ReactiveEntity,
        type_definition: T,
        component_ty: &ComponentTypeId,
    ) {
        if let Ok(v) = TypeDefinitionComponent::new(type_definition, component_ty.clone()).try_into() {
            entity_instance.set(EventProperties::EVENT.as_ref(), v);
        };
        // Also emit event that the type system has been changed
        self.emit_event(TypeSystemEvent::TypeSystemChanged);
    }

    fn propagate_type_definition_property_event(&self, entity_instance: ReactiveEntity, type_definition: TypeDefinition, property_name: String) {
        if let Ok(v) = TypeDefinitionProperty::new(type_definition, property_name).try_into() {
            entity_instance.set(EventProperties::EVENT.as_ref(), v);
        };
        self.emit_event(TypeSystemEvent::TypeSystemChanged);
    }

    // TODO: make this better!
    fn propagate_type_definition_property_renamed_event(
        &self,
        entity_instance: ReactiveEntity,
        type_definition: TypeDefinition,
        old_property_name: String,
        new_property_name: String,
    ) {
        if let Ok(v) = TypeDefinitionProperty::new(type_definition.clone(), old_property_name).try_into() {
            entity_instance.set(EventProperties::EVENT.as_ref(), v);
        };
        self.emit_event(TypeSystemEvent::TypeSystemChanged);
        if let Ok(v) = TypeDefinitionProperty::new(type_definition, new_property_name).try_into() {
            entity_instance.set(EventProperties::EVENT.as_ref(), v);
        };
        self.emit_event(TypeSystemEvent::TypeSystemChanged);
    }

    fn propagate_type_definition_extension_event(&self, entity_instance: ReactiveEntity, type_definition: TypeDefinition, extension_ty: ExtensionTypeId) {
        if let Ok(v) = TypeDefinitionExtension::new(type_definition, extension_ty).try_into() {
            entity_instance.set(EventProperties::EVENT.as_ref(), v);
        };
        self.emit_event(TypeSystemEvent::TypeSystemChanged);
    }

    // TODO: make this better!
    fn propagate_type_definition_extension_renamed_event(
        &self,
        entity_instance: ReactiveEntity,
        type_definition: TypeDefinition,
        old_extension_ty: ExtensionTypeId,
        new_extension_ty: ExtensionTypeId,
    ) {
        if let Ok(v) = TypeDefinitionExtension::new(type_definition.clone(), old_extension_ty).try_into() {
            entity_instance.set(EventProperties::EVENT.as_ref(), v);
        };
        self.emit_event(TypeSystemEvent::TypeSystemChanged);
        if let Ok(v) = TypeDefinitionExtension::new(type_definition, new_extension_ty).try_into() {
            entity_instance.set(EventProperties::EVENT.as_ref(), v);
        };
        self.emit_event(TypeSystemEvent::TypeSystemChanged);
    }

    pub(crate) fn create_system_event_instances(&self) {
        // let mut writer = self.system_event_instances.write().unwrap();
        self.system_event_instances.insert(
            TypeSystemEventTypes::ComponentCreated,
            self.create_system_event_instance("/io/reactive-graph/event/type/component/created"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::ComponentPropertyAdded,
            self.create_system_event_instance("/io/reactive-graph/event/type/component/property/added"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::ComponentPropertyRenamed,
            self.create_system_event_instance("/io/reactive-graph/event/type/component/property/renamed"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::ComponentPropertyUpdated,
            self.create_system_event_instance("/io/reactive-graph/event/type/component/property/updated"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::ComponentPropertyRemoved,
            self.create_system_event_instance("/io/reactive-graph/event/type/component/property/removed"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::ComponentExtensionAdded,
            self.create_system_event_instance("/io/reactive-graph/event/type/component/extension/added"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::ComponentExtensionRenamed,
            self.create_system_event_instance("/io/reactive-graph/event/type/component/extension/renamed"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::ComponentExtensionUpdated,
            self.create_system_event_instance("/io/reactive-graph/event/type/component/extension/updated"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::ComponentExtensionRemoved,
            self.create_system_event_instance("/io/reactive-graph/event/type/component/extension/removed"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::ComponentDeleted,
            self.create_system_event_instance("/io/reactive-graph/event/type/component/deleted"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::EntityTypeCreated,
            self.create_system_event_instance("/io/reactive-graph/event/type/entity/created"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::EntityTypeComponentAdded,
            self.create_system_event_instance("/io/reactive-graph/event/type/entity/component/added"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::EntityTypeComponentRemoved,
            self.create_system_event_instance("/io/reactive-graph/event/type/entity/component/removed"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::EntityTypePropertyAdded,
            self.create_system_event_instance("/io/reactive-graph/event/type/entity/property/added"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::EntityTypePropertyUpdated,
            self.create_system_event_instance("/io/reactive-graph/event/type/entity/property/updated"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::EntityTypePropertyRenamed,
            self.create_system_event_instance("/io/reactive-graph/event/type/entity/property/renamed"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::EntityTypePropertyRemoved,
            self.create_system_event_instance("/io/reactive-graph/event/type/entity/property/removed"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::EntityTypeExtensionAdded,
            self.create_system_event_instance("/io/reactive-graph/event/type/entity/extension/added"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::EntityTypeExtensionUpdated,
            self.create_system_event_instance("/io/reactive-graph/event/type/entity/extension/updated"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::EntityTypeExtensionRenamed,
            self.create_system_event_instance("/io/reactive-graph/event/type/entity/extension/renamed"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::EntityTypeExtensionRemoved,
            self.create_system_event_instance("/io/reactive-graph/event/type/entity/extension/removed"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::EntityTypeDeleted,
            self.create_system_event_instance("/io/reactive-graph/event/type/entity/deleted"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::RelationTypeCreated,
            self.create_system_event_instance("/io/reactive-graph/event/type/relation/created"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::RelationTypeComponentAdded,
            self.create_system_event_instance("/io/reactive-graph/event/type/relation/component/added"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::RelationTypeComponentRemoved,
            self.create_system_event_instance("/io/reactive-graph/event/type/relation/component/removed"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::RelationTypePropertyAdded,
            self.create_system_event_instance("/io/reactive-graph/event/type/relation/property/added"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::RelationTypePropertyUpdated,
            self.create_system_event_instance("/io/reactive-graph/event/type/relation/property/updated"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::RelationTypePropertyRenamed,
            self.create_system_event_instance("/io/reactive-graph/event/type/relation/property/renamed"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::RelationTypePropertyRemoved,
            self.create_system_event_instance("/io/reactive-graph/event/type/relation/property/removed"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::RelationTypeExtensionAdded,
            self.create_system_event_instance("/io/reactive-graph/event/type/relation/extension/added"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::RelationTypeExtensionUpdated,
            self.create_system_event_instance("/io/reactive-graph/event/type/relation/extension/updated"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::RelationTypeExtensionRenamed,
            self.create_system_event_instance("/io/reactive-graph/event/type/relation/extension/renamed"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::RelationTypeExtensionRemoved,
            self.create_system_event_instance("/io/reactive-graph/event/type/relation/extension/removed"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::RelationTypeDeleted,
            self.create_system_event_instance("/io/reactive-graph/event/type/relation/deleted"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::FlowTypeCreated,
            self.create_system_event_instance("/io/reactive-graph/event/type/flow/created"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::FlowTypeUpdated,
            self.create_system_event_instance("/io/reactive-graph/event/type/flow/updated"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::FlowTypeDeleted,
            self.create_system_event_instance("/io/reactive-graph/event/type/flow/deleted"),
        );
        self.system_event_instances.insert(
            TypeSystemEventTypes::TypeSystemChanged,
            self.create_system_event_instance("/io/reactive-graph/event/type/changed"),
        );
    }

    pub(crate) fn create_system_event_instance<S: Into<String>>(&self, label: S) -> ReactiveEntity {
        EntityInstance::builder()
            .ty(TYPE_SYSTEM_EVENT.deref())
            .properties(
                PropertyInstances::new()
                    .property(TYPE_SYSTEM_EVENT_PROPERTY_LABEL, json!(label.into()))
                    .property(EventProperties::EVENT.as_ref(), json!(false)),
            )
            .build()
            .into()
    }

    pub(crate) fn delete_system_event_instances(&self) {

        // let mut writer = self.event_instances.write().unwrap();
    }
}

#[async_trait]
impl Lifecycle for TypeSystemEventManagerImpl {
    async fn post_init(&self) {
        self.create_system_event_instances();
    }

    async fn pre_shutdown(&self) {
        self.delete_system_event_instances();
    }
}
