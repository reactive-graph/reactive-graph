use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::debug;
use log::error;
use log::trace;
use log::warn;
use wildmatch::WildMatch;

use crate::api::ComponentManager;
use crate::api::EntityTypeComponentError;
use crate::api::EntityTypeCreationError;
use crate::api::EntityTypeExtensionError;
use crate::api::EntityTypeImportError;
use crate::api::EntityTypeManager;
use crate::api::EntityTypePropertyError;
use crate::api::EntityTypeRegistrationError;
use crate::api::Lifecycle;
use crate::api::SystemEventManager;
use crate::builder::EntityTypeBuilder;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::implementation::COMPONENT_EVENT;
use crate::implementation::COMPONENT_LABELED;
use crate::implementation::NAMESPACE_CORE;
use crate::implementation::NAMESPACE_FLOW;
use crate::implementation::TYPE_GENERIC_FLOW;
use crate::implementation::TYPE_SYSTEM_EVENT;
use crate::model::ComponentTypeId;
use crate::model::EntityType;
use crate::model::EntityTypeId;
use crate::model::Extension;
use crate::model::ExtensionContainer;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyType;
use crate::model::TypeContainer;
use crate::model::TypeDefinitionGetter;
use crate::plugins::EntityTypeProvider;
use crate::plugins::SystemEvent;

#[wrapper]
pub struct EntityTypesStorage(RwLock<Vec<EntityType>>);

#[provides]
fn create_entity_types_storage() -> EntityTypesStorage {
    EntityTypesStorage(RwLock::new(Vec::new()))
}

#[component]
pub struct EntityTypeManagerImpl {
    event_manager: Wrc<dyn SystemEventManager>,

    component_manager: Wrc<dyn ComponentManager>,

    entity_types: EntityTypesStorage,
}

impl EntityTypeManagerImpl {
    pub(crate) fn create_base_entity_types(&self) {
        let ty_component_labeled = ComponentTypeId::new_from_type(NAMESPACE_CORE, COMPONENT_LABELED);
        let _ = self.register(
            EntityTypeBuilder::new_from_type(NAMESPACE_CORE, TYPE_SYSTEM_EVENT)
                .description("Events of the type system")
                .component(ty_component_labeled.clone())
                .component(ComponentTypeId::new_from_type(NAMESPACE_CORE, COMPONENT_EVENT))
                .build(),
        );
        let _ = self.register(
            EntityTypeBuilder::new_from_type(NAMESPACE_FLOW, TYPE_GENERIC_FLOW)
                .description("Generic flow without inputs and outputs")
                .component(ty_component_labeled)
                .build(),
        );
    }
}

#[async_trait]
#[provides]
impl EntityTypeManager for EntityTypeManagerImpl {
    fn register(&self, mut entity_type: EntityType) -> Result<EntityType, EntityTypeRegistrationError> {
        if self.has(&entity_type.ty) {
            return Err(EntityTypeRegistrationError::EntityTypeAlreadyExists(entity_type.ty));
        }
        for component_ty in entity_type.components.iter() {
            match self.component_manager.get(component_ty) {
                Some(component) => {
                    // TODO: what if multiple components have the same property?
                    entity_type.properties.append(&mut component.clone().properties)
                }
                None => warn!(
                    "Entity type {} not fully initialized: No component named {}",
                    entity_type.type_definition().to_string(),
                    component_ty.type_definition().to_string()
                ),
            }
        }
        self.entity_types.0.write().unwrap().push(entity_type.clone());
        debug!("Registered entity type {}", entity_type.type_definition().to_string());
        self.event_manager.emit_event(SystemEvent::EntityTypeCreated(entity_type.ty.clone()));
        Ok(entity_type)
    }

    fn get_all(&self) -> Vec<EntityType> {
        self.entity_types.0.read().unwrap().to_vec()
    }

    fn get_by_namespace(&self, namespace: &str) -> Vec<EntityType> {
        self.entity_types
            .0
            .read()
            .unwrap()
            .iter()
            .filter(|entity_type| entity_type.namespace() == namespace)
            .cloned()
            .collect()
    }

    fn get_by_having_component(&self, component_ty: &ComponentTypeId) -> Vec<EntityType> {
        self.entity_types
            .0
            .read()
            .unwrap()
            .iter()
            .filter(|entity_type| entity_type.components.contains(component_ty))
            .cloned()
            .collect()
    }

    fn has(&self, ty: &EntityTypeId) -> bool {
        self.entity_types.0.read().unwrap().iter().any(|entity_type| &entity_type.ty == ty)
    }

    fn has_by_type(&self, namespace: &str, type_name: &str) -> bool {
        self.has(&EntityTypeId::new_from_type(namespace, type_name))
    }

    fn get(&self, ty: &EntityTypeId) -> Option<EntityType> {
        self.entity_types.0.read().unwrap().iter().find(|entity_type| &entity_type.ty == ty).cloned()
    }

    fn get_by_type(&self, namespace: &str, type_name: &str) -> Option<EntityType> {
        self.get(&EntityTypeId::new_from_type(namespace, type_name))
    }

    fn find(&self, search: &str) -> Vec<EntityType> {
        let matcher = WildMatch::new(search);
        self.entity_types
            .0
            .read()
            .unwrap()
            .iter()
            .filter(|entity_type| matcher.matches(entity_type.type_name().as_str()))
            .cloned()
            .collect()
    }

    fn count(&self) -> usize {
        self.entity_types.0.read().unwrap().len()
    }

    fn count_by_namespace(&self, namespace: &str) -> usize {
        self.entity_types.0.read().unwrap().iter().filter(|e| e.namespace() == namespace).count()
    }

    fn create(
        &self,
        ty: &EntityTypeId,
        description: &str,
        components: Vec<ComponentTypeId>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> Result<EntityType, EntityTypeCreationError> {
        self.register(EntityType::new(ty.clone(), description, components.to_vec(), properties.to_vec(), extensions.to_vec()))
            .map_err(EntityTypeCreationError::RegistrationError)
    }

    fn add_component(&self, ty: &EntityTypeId, component_ty: &ComponentTypeId) -> Result<(), EntityTypeComponentError> {
        let mut guard = self.entity_types.0.write().unwrap();
        for entity_type in guard.iter_mut() {
            if &entity_type.ty == ty {
                if entity_type.is_a(component_ty) {
                    return Err(EntityTypeComponentError::ComponentAlreadyAssigned);
                }
                match self.component_manager.get(component_ty) {
                    Some(component) => {
                        entity_type.components.push(component_ty.clone());
                        // TODO: what if multiple components have the same property?
                        entity_type.properties.append(&mut component.clone().properties)
                    }
                    None => {
                        return Err(EntityTypeComponentError::ComponentDoesNotExist);
                    }
                }
                self.event_manager
                    .emit_event(SystemEvent::EntityTypeComponentAdded(ty.clone(), component_ty.clone()));
            }
        }
        Ok(())
    }

    fn remove_component(&self, ty: &EntityTypeId, component_ty: &ComponentTypeId) {
        let mut guard = self.entity_types.0.write().unwrap();
        for entity_type in guard.iter_mut() {
            if &entity_type.ty == ty {
                entity_type.components.retain(|c| c != component_ty);
                // TODO: what if multiple components have the same property?
                if let Some(component) = self.component_manager.get(component_ty) {
                    let properties_to_remove: Vec<String> = component.properties.iter().map(|property| property.name.clone()).collect();
                    entity_type.properties.retain(|property| !properties_to_remove.contains(&property.name));
                }
                self.event_manager
                    .emit_event(SystemEvent::EntityTypeComponentRemoved(ty.clone(), component_ty.clone()));
            }
        }
    }

    fn add_property(&self, ty: &EntityTypeId, property: PropertyType) -> Result<(), EntityTypePropertyError> {
        let mut guard = self.entity_types.0.write().unwrap();
        for entity_type in guard.iter_mut() {
            if &entity_type.ty == ty {
                if entity_type.has_own_property(property.name.clone()) {
                    return Err(EntityTypePropertyError::PropertyAlreadyExists);
                }
                entity_type.properties.push(property.clone());
                self.event_manager
                    .emit_event(SystemEvent::EntityTypePropertyAdded(ty.clone(), property.name.clone()));
            }
        }
        Ok(())
    }

    fn remove_property(&self, ty: &EntityTypeId, property_name: &str) {
        let mut guard = self.entity_types.0.write().unwrap();
        for entity_type in guard.iter_mut() {
            if &entity_type.ty == ty {
                entity_type.properties.retain(|property| property.name != property_name);
                self.event_manager
                    .emit_event(SystemEvent::EntityTypePropertyRemoved(ty.clone(), property_name.to_string()));
            }
        }
    }

    fn add_extension(&self, ty: &EntityTypeId, extension: Extension) -> Result<(), EntityTypeExtensionError> {
        let mut guard = self.entity_types.0.write().unwrap();
        for entity_type in guard.iter_mut() {
            if &entity_type.ty == ty {
                if entity_type.has_own_extension(extension.name.clone()) {
                    return Err(EntityTypeExtensionError::ExtensionAlreadyExists);
                }
                entity_type.extensions.push(extension.clone());
                self.event_manager
                    .emit_event(SystemEvent::EntityTypeExtensionAdded(ty.clone(), extension.name.clone()));
            }
        }
        Ok(())
    }

    fn remove_extension(&self, ty: &EntityTypeId, extension_name: &str) {
        let mut guard = self.entity_types.0.write().unwrap();
        for entity_type in guard.iter_mut() {
            if &entity_type.ty == ty {
                entity_type.extensions.retain(|extension| extension.name != extension_name);
                self.event_manager
                    .emit_event(SystemEvent::EntityTypeExtensionRemoved(ty.clone(), extension_name.to_string()));
            }
        }
    }

    /// TODO: first delete the entity instance of this type, then delete the entity type itself.
    fn delete(&self, ty: &EntityTypeId) {
        self.entity_types.0.write().unwrap().retain(|entity_type| &entity_type.ty != ty);
        self.event_manager.emit_event(SystemEvent::EntityTypeDeleted(ty.clone()));
    }

    fn validate(&self, ty: &EntityTypeId) -> bool {
        if let Some(entity_type) = self.get(ty) {
            return entity_type.components.iter().all(|component| self.component_manager.has(component));
        }
        false
    }

    fn import(&self, path: &str) -> Result<EntityType, EntityTypeImportError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let entity_type: EntityType = serde_json::from_reader(reader)?;
        self.register(entity_type).map_err(EntityTypeImportError::RegistrationError)
    }

    fn export(&self, ty: &EntityTypeId, path: &str) {
        if let Some(entity_type) = self.get(ty) {
            match File::create(path) {
                Ok(file) => {
                    let result = serde_json::to_writer_pretty(&file, &entity_type);
                    if result.is_err() {
                        error!("Failed to export entity type {} to {}: {}", ty.type_definition().to_string(), path, result.err().unwrap());
                    }
                }
                Err(error) => error!("Failed to export entity type {} to {}: {}", ty.type_definition().to_string(), path, error.to_string()),
            }
        }
    }

    fn add_provider(&self, entity_type_provider: Arc<dyn EntityTypeProvider>) {
        for entity_type in entity_type_provider.get_entity_types() {
            trace!("Registering entity type: {}", entity_type.type_definition().to_string());
            let _ = self.register(entity_type);
        }
    }

    fn get_entity_type_categories(&self) -> Vec<String> {
        self.get_all()
            .iter()
            .filter_map(|entity_type| {
                entity_type
                    .extensions
                    .iter()
                    .find(|extension| extension.name == *"entity_type_category")
                    .map(|extension| extension.name.clone())
            })
            .collect()
    }
}

impl Lifecycle for EntityTypeManagerImpl {
    fn init(&self) {
        self.create_base_entity_types();
    }

    fn shutdown(&self) {
        // TODO: remove?
        self.entity_types.0.write().unwrap().clear();
    }
}
