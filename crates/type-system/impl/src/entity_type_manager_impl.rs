use std::ops::Deref;
use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use log::warn;
use serde_json::json;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::ComponentTypeIds;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::EntityTypeAddComponentError;
use reactive_graph_graph::EntityTypeAddExtensionError;
use reactive_graph_graph::EntityTypeAddPropertyError;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::EntityTypeIds;
use reactive_graph_graph::EntityTypeMergeError;
use reactive_graph_graph::EntityTypeRemoveComponentError;
use reactive_graph_graph::EntityTypeRemoveExtensionError;
use reactive_graph_graph::EntityTypeRemovePropertyError;
use reactive_graph_graph::EntityTypeUpdateError;
use reactive_graph_graph::EntityTypeUpdateExtensionError;
use reactive_graph_graph::EntityTypeUpdatePropertyError;
use reactive_graph_graph::EntityTypes;
use reactive_graph_graph::Extension;
use reactive_graph_graph::ExtensionContainer;
use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::Extensions;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::NamespacedTypeComponentTypeIdContainer;
use reactive_graph_graph::NamespacedTypeContainer;
use reactive_graph_graph::NamespacedTypeExtensionContainer;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::NamespacedTypePropertyTypeContainer;
use reactive_graph_graph::Namespaces;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypeContainer;
use reactive_graph_graph::PropertyTypes;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_graph::divergent::Divergent;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::EntityTypeCreationError;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::EntityTypeRegistrationError;
use reactive_graph_type_system_api::NamespacedTypeManager;
use reactive_graph_type_system_api::TypeSystemEvent;
use reactive_graph_type_system_api::TypeSystemEventManager;
use reactive_graph_type_system_model::EXTENSION_DIVERGENT;

#[derive(Component)]
pub struct EntityTypeManagerImpl {
    event_manager: Arc<dyn TypeSystemEventManager + Send + Sync>,

    component_manager: Arc<dyn ComponentManager + Send + Sync>,

    namespaced_type_manager: Arc<dyn NamespacedTypeManager + Send + Sync>,

    #[component(default = "EntityTypes::new")]
    entity_types: EntityTypes,
}

#[async_trait]
#[component_alias]
impl EntityTypeManager for EntityTypeManagerImpl {
    fn register(&self, entity_type: EntityType) -> Result<EntityType, EntityTypeRegistrationError> {
        let ty = entity_type.ty.clone();
        self.namespaced_type_manager.register(ty.namespaced_type())?;
        if self.entity_types.contains_key(&ty) {
            return Err(EntityTypeRegistrationError::EntityTypeAlreadyExists(ty));
        }

        // Apply components
        let mut divergent = Divergent::new();
        for component_ty in entity_type.components.iter() {
            match self.component_manager.get(&component_ty) {
                Some(component) => {
                    let divergent_properties = entity_type.properties.merge_non_existent_properties(component.properties);
                    if !divergent_properties.is_empty() {
                        for divergent_property in divergent_properties.deref() {
                            warn!(
                                "{}__{} has divergent data type {} to {}__{} which has data type {}",
                                &entity_type.ty,
                                &divergent_property.existing().name,
                                &divergent_property.existing().data_type,
                                component_ty.deref(),
                                &divergent_property.divergent().name,
                                &divergent_property.divergent().data_type,
                            );
                        }
                        divergent.divergent_component(component_ty.deref(), divergent_properties);
                    }
                }
                None => {
                    divergent.unfulfilled_component(component_ty.deref());
                    warn!(
                        "Entity type {} not fully initialized: No component named {}",
                        entity_type.type_definition(),
                        component_ty.type_definition()
                    )
                }
            }
        }

        let divergent_components: Vec<String> = divergent.divergent_components().into_iter().map(|ty| ty.to_string()).collect();

        let _ = entity_type.add_extension(Extension::new(EXTENSION_DIVERGENT.clone(), String::new(), json!(divergent_components)));
        // entity_type
        //     .extensions
        //     .push(Extension::new(EXTENSION_DIVERGENT.clone(), String::new(), json!(divergent)));
        self.entity_types.push(entity_type.clone());
        // self.entity_types.0.write().unwrap().push(entity_type.clone());
        debug!("Registered entity type {}", entity_type.type_definition());
        self.event_manager.emit_event(TypeSystemEvent::EntityTypeCreated(entity_type.ty.clone()));
        Ok(entity_type)
    }

    fn get_all(&self) -> EntityTypes {
        self.entity_types.clone()
    }

    fn get_type_ids(&self) -> EntityTypeIds {
        self.entity_types.type_ids()
    }

    fn get_namespaces(&self) -> Namespaces {
        self.entity_types.namespaces()
    }

    fn get_by_namespace(&self, namespace: &Namespace) -> EntityTypes {
        self.entity_types.get_by_namespace(namespace)
    }

    fn get_types_by_namespace(&self, namespace: &Namespace) -> EntityTypeIds {
        self.entity_types.get_types_by_namespace(namespace)
    }

    fn get_by_having_component(&self, component_ty: &ComponentTypeId) -> EntityTypes {
        self.entity_types.get_by_having_component(component_ty)
    }

    fn has(&self, ty: &EntityTypeId) -> bool {
        self.entity_types.contains_key(ty)
    }

    fn get(&self, ty: &EntityTypeId) -> Option<EntityType> {
        self.entity_types.get(ty).map(|entity_type| entity_type.value().clone())
    }

    fn find(&self, search: &str) -> EntityTypes {
        self.entity_types.find(search)
    }

    fn count(&self) -> usize {
        self.entity_types.len()
    }

    fn count_by_namespace(&self, namespace: &Namespace) -> usize {
        self.entity_types.count_by_namespace(namespace)
    }

    fn create_entity_type(
        &self,
        ty: &EntityTypeId,
        description: &str,
        components: ComponentTypeIds,
        properties: PropertyTypes,
        extensions: Extensions,
    ) -> Result<EntityType, EntityTypeCreationError> {
        let entity_type = EntityType::builder()
            .ty(ty)
            .description(description)
            .components(components)
            .properties(properties)
            .extensions(extensions)
            .build();
        self.register(entity_type).map_err(EntityTypeCreationError::RegistrationError)
    }

    fn update_description(&self, ty: &EntityTypeId, description: &str) -> Result<EntityType, EntityTypeUpdateError> {
        if !self.has(ty) {
            return Err(EntityTypeUpdateError::EntityTypeDoesNotExist(ty.clone()));
        }
        for mut entity_type in self.entity_types.iter_mut() {
            if &entity_type.ty == ty {
                entity_type.description = description.to_string();
                // TODO: Notify about changed entity_type
            }
        }
        self.get(ty).ok_or(EntityTypeUpdateError::EntityTypeDoesNotExist(ty.clone()))
    }

    fn merge(&self, entity_type_to_merge: EntityType) -> Result<EntityType, EntityTypeMergeError> {
        let components = entity_type_to_merge.components.clone();
        let entity_type = self.entity_types.merge(entity_type_to_merge)?;
        let ty = entity_type.ty;
        // Also populate properties from new components
        for component_ty in components.iter() {
            if let Some(component) = self.component_manager.get(&component_ty) {
                for property_type in component.properties.iter() {
                    let _ = self.add_property(&ty, property_type.value().clone());
                }
            }
        }
        self.entity_types
            .get(&ty)
            .map(|entity_type| entity_type.value().clone())
            .ok_or(EntityTypeMergeError::EntityTypeDoesNotExist(ty))
    }

    fn add_component(&self, entity_ty: &EntityTypeId, component_ty: &ComponentTypeId) -> Result<(), EntityTypeAddComponentError> {
        let Some(component) = self.component_manager.get(component_ty) else {
            return Err(EntityTypeAddComponentError::ComponentDoesNotExist(component_ty.clone()));
        };
        self.entity_types.add_component(entity_ty, component_ty)?;
        let _ = self.entity_types.merge_properties(entity_ty, component.properties.clone());
        self.event_manager
            .emit_event(TypeSystemEvent::EntityTypeComponentAdded(entity_ty.clone(), component_ty.clone()));
        Ok(())
    }

    fn remove_component(&self, entity_ty: &EntityTypeId, component_ty: &ComponentTypeId) -> Result<ComponentTypeId, EntityTypeRemoveComponentError> {
        self.entity_types.remove_component(entity_ty, component_ty)?;
        if let Some(component) = self.component_manager.get(component_ty) {
            // TODO: what if multiple components have the same property?
            component.properties.iter().for_each(|property| {
                let _ = self.entity_types.remove_property(entity_ty, property.key());
                self.event_manager
                    .emit_event(TypeSystemEvent::EntityTypePropertyRemoved(entity_ty.clone(), property.key().clone()));
            });
        }
        self.event_manager
            .emit_event(TypeSystemEvent::EntityTypeComponentRemoved(entity_ty.clone(), component_ty.clone()));
        Ok(component_ty.clone())
    }

    fn add_property(&self, entity_ty: &EntityTypeId, property_type: PropertyType) -> Result<PropertyType, EntityTypeAddPropertyError> {
        let property_type = self.entity_types.add_property(entity_ty, property_type)?;
        self.event_manager
            .emit_event(TypeSystemEvent::EntityTypePropertyAdded(entity_ty.clone(), property_type.name.clone()));
        Ok(property_type)
    }

    fn update_property(
        &self,
        entity_ty: &EntityTypeId,
        property_name: &str,
        property_type: PropertyType,
    ) -> Result<PropertyType, EntityTypeUpdatePropertyError> {
        let property_type = self.entity_types.update_property(entity_ty, property_name, property_type)?;
        if property_name == property_type.name {
            self.event_manager.emit_event(TypeSystemEvent::EntityTypePropertyRenamed(
                entity_ty.clone(),
                property_name.to_string(),
                property_type.name.clone(),
            ));
        }
        self.event_manager
            .emit_event(TypeSystemEvent::EntityTypePropertyUpdated(entity_ty.clone(), property_name.to_string()));
        Ok(property_type)
    }

    fn remove_property(&self, entity_ty: &EntityTypeId, property_name: &str) -> Result<PropertyType, EntityTypeRemovePropertyError> {
        let property_type = self.entity_types.remove_property(entity_ty, property_name)?;
        self.event_manager
            .emit_event(TypeSystemEvent::EntityTypePropertyRemoved(entity_ty.clone(), property_name.to_string()));
        Ok(property_type)
    }

    fn add_extension(&self, entity_ty: &EntityTypeId, extension: Extension) -> Result<ExtensionTypeId, EntityTypeAddExtensionError> {
        let extension_ty = self.entity_types.add_extension(entity_ty, extension)?;
        self.event_manager
            .emit_event(TypeSystemEvent::EntityTypeExtensionAdded(entity_ty.clone(), extension_ty.clone()));
        Ok(extension_ty)
    }

    fn update_extension(
        &self,
        entity_ty: &EntityTypeId,
        extension_ty: &ExtensionTypeId,
        extension: Extension,
    ) -> Result<Extension, EntityTypeUpdateExtensionError> {
        let extension = self.entity_types.update_extension(entity_ty, extension_ty, extension)?;
        if extension_ty == &extension.ty {
            self.event_manager
                .emit_event(TypeSystemEvent::EntityTypeExtensionRenamed(entity_ty.clone(), extension_ty.clone(), extension.ty.clone()));
        }
        self.event_manager
            .emit_event(TypeSystemEvent::EntityTypeExtensionUpdated(entity_ty.clone(), extension.ty.clone()));
        Ok(extension)
    }

    fn remove_extension(&self, entity_ty: &EntityTypeId, extension_ty: &ExtensionTypeId) -> Result<Extension, EntityTypeRemoveExtensionError> {
        let extension = self.entity_types.remove_extension(entity_ty, extension_ty)?;
        self.event_manager
            .emit_event(TypeSystemEvent::EntityTypeExtensionRemoved(entity_ty.clone(), extension_ty.clone()));
        Ok(extension)
    }

    // TODO: parameter "cascade": relation types, flow types and entity instances (and their dependencies) depends on a entity type
    // TODO: first delete the entity instance of this type, then delete the entity type itself.
    fn delete(&self, ty: &EntityTypeId) -> Option<EntityType> {
        self.namespaced_type_manager.delete(ty.as_ref());
        self.entity_types.remove(ty).map(|(entity_ty, entity_type)| {
            self.event_manager.emit_event(TypeSystemEvent::EntityTypeDeleted(entity_ty.clone()));
            entity_type
        })
    }

    fn validate(&self, ty: &EntityTypeId) -> bool {
        if let Some(entity_type) = self.get(ty) {
            return entity_type.components.iter().all(|component| self.component_manager.has(&component));
        }
        false
    }
}

#[async_trait]
impl Lifecycle for EntityTypeManagerImpl {
    async fn shutdown(&self) {
        self.entity_types.clear()
    }
}

#[cfg(test)]
mod test {
    use crate::TypeSystemSystemImpl;
    use reactive_graph_graph::Component;
    use reactive_graph_graph::ComponentTypeIdContainer;
    use reactive_graph_graph::ComponentTypeIds;
    use reactive_graph_graph::EntityType;
    use reactive_graph_graph::EntityTypeId;
    use reactive_graph_graph::Extensions;
    use reactive_graph_graph::PropertyType;
    use reactive_graph_graph::PropertyTypeContainer;
    use reactive_graph_graph::PropertyTypes;
    use reactive_graph_graph::RandomNamespacedType;
    use reactive_graph_graph::RandomNamespacedTypeId;
    use reactive_graph_graph::RandomNamespacedTypeIds;
    use reactive_graph_graph::RandomNamespacedTypes;
    use reactive_graph_type_system_api::TypeSystemSystem;
    use reactive_graph_utils_test::r_string;

    #[test]
    fn test_register_entity_type() {
        reactive_graph_utils_test::init_logger();
        let type_system = reactive_graph_di::get_container::<TypeSystemSystemImpl>();
        let entity_type_manager = type_system.get_entity_type_manager();

        let entity_ty = EntityTypeId::random_type_id().unwrap();
        let description = r_string();
        let components = ComponentTypeIds::random_type_ids().unwrap();
        let properties = PropertyTypes::random_types(1..5).unwrap();
        let extensions = Extensions::random_types(1..3).unwrap();

        let entity_type = EntityType::new(&entity_ty, &description, components, properties, extensions);
        let entity_type = entity_type_manager.register(entity_type.clone()).expect("Failed to register the entity type!");
        assert!(entity_type_manager.has(&entity_type.ty));
        assert!(entity_type_manager.has(&entity_ty));
        assert_eq!(Some(entity_type.clone()), entity_type_manager.get(&entity_ty));
    }

    #[test]
    fn test_create_and_delete_entity_type() {
        reactive_graph_utils_test::init_logger();
        let type_system = reactive_graph_di::get_container::<TypeSystemSystemImpl>();
        let entity_type_manager = type_system.get_entity_type_manager();

        let entity_type = entity_type_manager
            .register(EntityType::random_type().unwrap())
            .expect("Failed to register the entity type!");
        let ty = entity_type.ty.clone();

        assert!(entity_type_manager.has(&ty), "The entity type should be registered!");
        entity_type_manager.delete(&ty).expect("Failed to delete the entity type!");
        assert!(!entity_type_manager.has(&ty), "The entity type shouldn't be registered anymore!");
        assert!(entity_type_manager.get(&ty).is_none(), "The entity type shouldn't be registered anymore!");
    }

    #[test]
    fn test_get_entity_types() {
        reactive_graph_utils_test::init_logger();
        let type_system = reactive_graph_di::get_container::<TypeSystemSystemImpl>();
        let entity_type_manager = type_system.get_entity_type_manager();

        let entity_type = entity_type_manager
            .register(EntityType::random_type().unwrap())
            .expect("Failed to register the entity type!");
        assert!(entity_type_manager.has(&entity_type.ty), "The entity type should be registered!");
        let entity_types = entity_type_manager.get_all();
        assert_eq!(1, entity_types.len(), "There should be exactly one entity type!");
        for entity_type in entity_types.iter() {
            assert!(
                entity_type_manager.has(&entity_type.ty),
                "It should be possible to check if the returned entity types are registered!"
            );
            let _ = entity_type_manager
                .get(&entity_type.ty)
                .expect("It should be possible to get the returned entity types by type id!");
        }
    }

    #[test]
    fn test_register_entity_type_has_component() {
        reactive_graph_utils_test::init_logger();
        let type_system = reactive_graph_di::get_container::<TypeSystemSystemImpl>();
        let component_manager = type_system.get_component_manager();
        let entity_type_manager = type_system.get_entity_type_manager();

        let component = component_manager
            .register(Component::random_type().unwrap())
            .expect("Failed to register component!");

        let entity_ty = EntityTypeId::random_type_id().unwrap();
        let entity_type = EntityType::builder_from_ty(&entity_ty).component(&component.ty).build();

        let _entity_type = entity_type_manager.register(entity_type).expect("Failed to register entity type!");
        let entity_type = entity_type_manager
            .get(&entity_ty)
            .expect("It should be possible to get the entity type by type id!");
        assert!(entity_type.is_a(&component.ty), "The entity type should contain the component!");
        assert!(entity_type.components.contains(&component.ty), "The entity type should contain the component!");
    }

    #[test]
    fn test_register_entity_type_has_property() {
        reactive_graph_utils_test::init_logger();
        let type_system = reactive_graph_di::get_container::<TypeSystemSystemImpl>();
        let entity_type_manager = type_system.get_entity_type_manager();

        let property_type = PropertyType::random_type().unwrap();

        let entity_ty = EntityTypeId::random_type_id().unwrap();
        let entity_type = EntityType::builder_from_ty(&entity_ty)
            .components(ComponentTypeIds::random_type_ids().unwrap())
            .property(property_type.clone())
            .build();

        let _entity_type = entity_type_manager.register(entity_type).expect("Failed to register entity type!");
        let entity_type = entity_type_manager
            .get(&entity_ty)
            .expect("It should be possible to get the entity type by type id!");
        assert!(entity_type.has_own_property(&property_type.name));
        assert!(entity_type.properties.contains_key(&property_type.name));
    }
}
