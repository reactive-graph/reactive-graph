use std::ops::Deref;
use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use log::warn;
use serde_json::json;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_graph::ComponentOrEntityTypeId;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::ComponentTypeIds;
use reactive_graph_graph::Extension;
use reactive_graph_graph::ExtensionContainer;
use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::Extensions;
use reactive_graph_graph::NamespacedTypeComponentTypeIdContainer;
use reactive_graph_graph::NamespacedTypeContainer;
use reactive_graph_graph::NamespacedTypeExtensionContainer;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::NamespacedTypePropertyTypeContainer;
use reactive_graph_graph::Namespaces;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypeContainer;
use reactive_graph_graph::PropertyTypes;
use reactive_graph_graph::RelationType;
use reactive_graph_graph::RelationTypeAddComponentError;
use reactive_graph_graph::RelationTypeAddExtensionError;
use reactive_graph_graph::RelationTypeAddPropertyError;
use reactive_graph_graph::RelationTypeId;
use reactive_graph_graph::RelationTypeIds;
use reactive_graph_graph::RelationTypeMergeError;
use reactive_graph_graph::RelationTypeRemoveComponentError;
use reactive_graph_graph::RelationTypeRemoveExtensionError;
use reactive_graph_graph::RelationTypeRemovePropertyError;
use reactive_graph_graph::RelationTypeUpdateError;
use reactive_graph_graph::RelationTypeUpdateExtensionError;
use reactive_graph_graph::RelationTypeUpdatePropertyError;
use reactive_graph_graph::RelationTypes;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_runtime_model::EXTENSION_DIVERGENT;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::RelationTypeCreationError;
use reactive_graph_type_system_api::RelationTypeManager;
use reactive_graph_type_system_api::RelationTypeRegistrationError;
use reactive_graph_type_system_api::TypeSystemEvent;
use reactive_graph_type_system_api::TypeSystemEventManager;

#[derive(Component)]
pub struct RelationTypeManagerImpl {
    event_manager: Arc<dyn TypeSystemEventManager + Send + Sync>,

    component_manager: Arc<dyn ComponentManager + Send + Sync>,

    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,

    #[component(default = "RelationTypes::new")]
    relation_types: RelationTypes,
}

#[async_trait]
#[component_alias]
impl RelationTypeManager for RelationTypeManagerImpl {
    fn register(&self, relation_type: RelationType) -> Result<RelationType, RelationTypeRegistrationError> {
        let relation_ty = relation_type.ty.clone();
        if self.has(&relation_ty) {
            return Err(RelationTypeRegistrationError::RelationTypeAlreadyExists(relation_ty));
        }
        // Check if outbound type exists
        if relation_type.outbound_type.type_name() != "*" {
            match &relation_type.outbound_type {
                ComponentOrEntityTypeId::Component(component_ty) => {
                    if !self.component_manager.has(component_ty) {
                        warn!("Relation type {} not registered: Outbound component {} does not exist", &relation_ty, component_ty);
                        return Err(RelationTypeRegistrationError::OutboundComponentDoesNotExist(relation_ty, component_ty.clone()));
                    }
                }
                ComponentOrEntityTypeId::EntityType(entity_ty) => {
                    if !self.entity_type_manager.has(entity_ty) {
                        warn!("Relation type {} not registered: Outbound entity type {} does not exist", &relation_ty, entity_ty);
                        return Err(RelationTypeRegistrationError::OutboundEntityTypeDoesNotExist(relation_ty, entity_ty.clone()));
                    }
                }
            }
        }
        // Check if inbound type exists
        if relation_type.inbound_type.type_name() != "*" {
            match &relation_type.inbound_type {
                ComponentOrEntityTypeId::Component(component_ty) => {
                    if !self.component_manager.has(component_ty) {
                        warn!("Relation type {} not registered: Inbound component {} does not exist", &relation_ty, component_ty);
                        return Err(RelationTypeRegistrationError::InboundComponentDoesNotExist(relation_ty, component_ty.clone()));
                    }
                }
                ComponentOrEntityTypeId::EntityType(entity_ty) => {
                    if !self.entity_type_manager.has(entity_ty) {
                        warn!("Relation type {} not registered: Inbound entity type {} does not exist", &relation_ty, entity_ty);
                        return Err(RelationTypeRegistrationError::InboundEntityTypeDoesNotExist(relation_ty, entity_ty.clone()));
                    }
                }
            }
        }
        // Apply components
        let mut divergent = Vec::new();
        for component_ty in relation_type.components.iter() {
            let mut is_divergent = false;
            match self.component_manager.get(&component_ty) {
                Some(component) => {
                    // TODO: what if multiple components have the same property?
                    for (property_name, property_type) in component.properties {
                        // Own property wins
                        if !relation_type.has_own_property(&property_name) {
                            relation_type.properties.push(property_type.clone());
                        } else {
                            // Check for divergent data type
                            if let Some(relation_type_property_type) = relation_type.get_own_property(&property_type.name) {
                                if property_type.data_type != relation_type_property_type.data_type {
                                    is_divergent = true;
                                    warn!(
                                        "{}__{} has divergent data type {} to {}__{} which has data type {}",
                                        &relation_type.ty,
                                        &relation_type_property_type.name,
                                        &relation_type_property_type.data_type,
                                        component_ty.deref(),
                                        &property_type.name,
                                        &property_type.data_type
                                    );
                                }
                            }
                            // TODO: merge description (if no own description)
                            // TODO: merge extensions (for each: if own does not have the extension, add it)
                        }
                    }
                    // relation_type.properties.append(&mut component.properties.to_vec())
                }
                None => {
                    is_divergent = true;
                    warn!("Relation type {} not fully initialized: Missing component {}", &relation_ty, component_ty.deref())
                }
            }
            if is_divergent {
                divergent.push(component_ty.to_string());
            }
        }
        divergent.sort();
        let _ = relation_type.add_extension(Extension::new(EXTENSION_DIVERGENT.clone(), String::new(), json!(divergent)));
        self.relation_types.push(relation_type.clone());
        debug!("Registered relation type {}", &relation_ty);
        self.event_manager.emit_event(TypeSystemEvent::RelationTypeCreated(relation_ty));
        Ok(relation_type)
    }

    fn get_all(&self) -> RelationTypes {
        self.relation_types.clone()
    }

    fn get_type_ids(&self) -> RelationTypeIds {
        self.relation_types.type_ids()
    }

    fn get_namespaces(&self) -> Namespaces {
        self.relation_types.namespaces()
    }

    fn get_by_namespace(&self, namespace: &str) -> RelationTypes {
        self.relation_types.get_by_namespace(namespace)
    }

    fn get_types_by_namespace(&self, namespace: &str) -> RelationTypeIds {
        self.relation_types.get_types_by_namespace(namespace)
    }

    fn get_by_having_component(&self, component_ty: &ComponentTypeId) -> RelationTypes {
        self.relation_types.get_by_having_component(component_ty)
    }

    fn get_outbound_relation_types(&self, outbound_ty: &ComponentOrEntityTypeId, wildcard: bool) -> RelationTypes {
        // TODO:
        // if wildcard && outbound_ty.namespace() == "*" {
        //     return self.get_all();
        // } else if wildcard && outbound_ty.type_name() == "*" {
        //     return self.get_by_namespace(outbound_ty.namespace());
        // } else {
        //     self.get_all()
        //         .into_iter()
        //         .filter(|relation_type| (wildcard && &relation_type.outbound_type.type_name() == "*") || outbound_ty == &relation_type.outbound_type)
        //         .collect()
        // }
        if wildcard && outbound_ty.type_name() == "*" {
            return self.get_all();
        }
        self.get_all()
            .into_iter()
            .filter(|(_, relation_type)| (wildcard && &relation_type.outbound_type.type_name() == "*") || outbound_ty == &relation_type.outbound_type)
            .map(|(_, relation_type)| relation_type)
            .collect()
    }

    fn get_inbound_relation_types(&self, inbound_ty: &ComponentOrEntityTypeId, wildcard: bool) -> RelationTypes {
        if wildcard && inbound_ty.type_name() == "*" {
            return self.get_all();
        }
        self.get_all()
            .into_iter()
            .filter(|(_, relation_type)| (wildcard && &relation_type.inbound_type.type_name() == "*") || inbound_ty == &relation_type.inbound_type)
            .map(|(_, relation_type)| relation_type)
            .collect()
    }

    fn has(&self, ty: &RelationTypeId) -> bool {
        self.relation_types.contains_key(ty)
    }

    fn has_by_type(&self, namespace: &str, type_name: &str) -> bool {
        self.relation_types.contains_key(&RelationTypeId::new_from_type(namespace, type_name))
    }

    fn get(&self, ty: &RelationTypeId) -> Option<RelationType> {
        self.relation_types.get(ty).map(|relation_type| relation_type.value().clone())
    }

    fn get_by_type(&self, namespace: &str, type_name: &str) -> Option<RelationType> {
        self.get(&RelationTypeId::new_from_type(namespace, type_name))
    }

    fn find_by_type_name(&self, search: &str) -> RelationTypes {
        self.relation_types.find_by_type_name(search)
    }

    fn count(&self) -> usize {
        self.relation_types.len()
    }

    /// Returns the count of relation types of the given namespace.
    fn count_by_namespace(&self, namespace: &str) -> usize {
        self.relation_types.count_by_namespace(namespace)
    }

    fn create_relation_type(
        &self,
        outbound_type: &ComponentOrEntityTypeId,
        ty: &RelationTypeId,
        inbound_type: &ComponentOrEntityTypeId,
        description: &str,
        components: ComponentTypeIds,
        properties: PropertyTypes,
        extensions: Extensions,
    ) -> Result<RelationType, RelationTypeCreationError> {
        let relation_type = RelationType::builder()
            .outbound_type(outbound_type)
            .ty(ty)
            .inbound_type(inbound_type)
            .description(description)
            .components(components)
            .properties(properties)
            .extensions(extensions)
            .build();
        self.register(relation_type).map_err(RelationTypeCreationError::RegistrationError)
    }

    fn update_description(&self, ty: &RelationTypeId, description: &str) -> Result<RelationType, RelationTypeUpdateError> {
        if !self.has(ty) {
            return Err(RelationTypeUpdateError::RelationTypeDoesNotExist(ty.clone()));
        }
        for mut relation_type in self.relation_types.iter_mut() {
            if &relation_type.ty == ty {
                relation_type.description = description.to_string();
                // TODO: Notify about changed relation_type
            }
        }
        self.get(ty).ok_or(RelationTypeUpdateError::RelationTypeDoesNotExist(ty.clone()))
    }

    fn merge(&self, relation_type_to_merge: RelationType) -> Result<RelationType, RelationTypeMergeError> {
        let components = relation_type_to_merge.components.clone();
        let relation_type = self.relation_types.merge(relation_type_to_merge)?;
        let ty = relation_type.ty;
        // Also populate properties from new components
        for component_ty in components.iter() {
            if let Some(component) = self.component_manager.get(&component_ty) {
                for property_type in component.properties.iter() {
                    let _ = self.add_property(&ty, property_type.value().clone());
                }
            }
        }
        self.relation_types
            .get(&ty)
            .map(|relation_type| relation_type.value().clone())
            .ok_or(RelationTypeMergeError::RelationTypeDoesNotExist(ty))
    }

    fn add_component(&self, relation_ty: &RelationTypeId, component_ty: &ComponentTypeId) -> Result<(), RelationTypeAddComponentError> {
        let Some(component) = self.component_manager.get(component_ty) else {
            return Err(RelationTypeAddComponentError::ComponentDoesNotExist(component_ty.clone()));
        };
        self.relation_types.add_component(relation_ty, component_ty)?;
        let _ = self.relation_types.merge_properties(relation_ty, component.properties.clone());
        self.event_manager
            .emit_event(TypeSystemEvent::RelationTypeComponentAdded(relation_ty.clone(), component_ty.clone()));
        Ok(())
    }

    fn remove_component(&self, relation_ty: &RelationTypeId, component_ty: &ComponentTypeId) -> Result<ComponentTypeId, RelationTypeRemoveComponentError> {
        self.relation_types.remove_component(relation_ty, component_ty)?;
        if let Some(component) = self.component_manager.get(component_ty) {
            // TODO: what if multiple components have the same property?
            component.properties.iter().for_each(|property| {
                let _ = self.relation_types.remove_property(relation_ty, property.key());
                self.event_manager
                    .emit_event(TypeSystemEvent::RelationTypePropertyRemoved(relation_ty.clone(), property.key().clone()));
            });
        }
        self.event_manager
            .emit_event(TypeSystemEvent::RelationTypeComponentRemoved(relation_ty.clone(), component_ty.clone()));
        Ok(component_ty.clone())
    }

    fn add_property(&self, relation_ty: &RelationTypeId, property: PropertyType) -> Result<PropertyType, RelationTypeAddPropertyError> {
        let property_type = self.relation_types.add_property(relation_ty, property)?;
        self.event_manager
            .emit_event(TypeSystemEvent::RelationTypePropertyAdded(relation_ty.clone(), property_type.name.clone()));
        Ok(property_type)
    }

    fn update_property(
        &self,
        relation_ty: &RelationTypeId,
        property_name: &str,
        property_type: PropertyType,
    ) -> Result<PropertyType, RelationTypeUpdatePropertyError> {
        let property_type = self.relation_types.update_property(relation_ty, property_name, property_type)?;
        if property_name == property_type.name {
            self.event_manager.emit_event(TypeSystemEvent::RelationTypePropertyRenamed(
                relation_ty.clone(),
                property_name.to_string(),
                property_type.name.clone(),
            ));
        }
        self.event_manager
            .emit_event(TypeSystemEvent::RelationTypePropertyUpdated(relation_ty.clone(), property_name.to_string()));
        Ok(property_type)
    }

    fn remove_property(&self, relation_ty: &RelationTypeId, property_name: &str) -> Result<PropertyType, RelationTypeRemovePropertyError> {
        let property_type = self.relation_types.remove_property(relation_ty, property_name)?;
        self.event_manager
            .emit_event(TypeSystemEvent::RelationTypePropertyRemoved(relation_ty.clone(), property_name.to_string()));
        Ok(property_type)
    }

    fn add_extension(&self, relation_ty: &RelationTypeId, extension: Extension) -> Result<ExtensionTypeId, RelationTypeAddExtensionError> {
        let extension_ty = self.relation_types.add_extension(relation_ty, extension)?;
        self.event_manager
            .emit_event(TypeSystemEvent::RelationTypeExtensionAdded(relation_ty.clone(), extension_ty.clone()));
        Ok(extension_ty)
    }

    fn update_extension(
        &self,
        relation_ty: &RelationTypeId,
        extension_ty: &ExtensionTypeId,
        extension: Extension,
    ) -> Result<Extension, RelationTypeUpdateExtensionError> {
        let extension = self.relation_types.update_extension(relation_ty, extension_ty, extension)?;
        if extension_ty == &extension.ty {
            self.event_manager
                .emit_event(TypeSystemEvent::RelationTypeExtensionRenamed(relation_ty.clone(), extension_ty.clone(), extension.ty.clone()));
        }
        self.event_manager
            .emit_event(TypeSystemEvent::RelationTypeExtensionUpdated(relation_ty.clone(), extension.ty.clone()));
        Ok(extension)
    }

    fn remove_extension(&self, relation_ty: &RelationTypeId, extension_ty: &ExtensionTypeId) -> Result<Extension, RelationTypeRemoveExtensionError> {
        let extension = self.relation_types.remove_extension(relation_ty, extension_ty)?;
        self.event_manager
            .emit_event(TypeSystemEvent::RelationTypeExtensionRemoved(relation_ty.clone(), extension_ty.clone()));
        Ok(extension)
    }

    // TODO: parameter "cascade": flow types and relation instances (and their dependencies) depends on a relation type
    fn delete(&self, ty: &RelationTypeId) -> Option<RelationType> {
        self.relation_types.remove(ty).map(|(ty, relation_type)| {
            self.event_manager.emit_event(TypeSystemEvent::RelationTypeDeleted(ty.clone()));
            relation_type
        })
    }

    fn validate(&self, ty: &RelationTypeId) -> bool {
        if let Some(relation_type) = self.get(ty) {
            return relation_type.components.iter().all(|component_ty| self.component_manager.has(&component_ty))
                && match &relation_type.outbound_type {
                    ComponentOrEntityTypeId::EntityType(entity_ty) => self.entity_type_manager.validate(entity_ty),
                    ComponentOrEntityTypeId::Component(component_ty) => self.component_manager.has(component_ty),
                }
                && match &relation_type.inbound_type {
                    ComponentOrEntityTypeId::EntityType(entity_ty) => self.entity_type_manager.validate(entity_ty),
                    ComponentOrEntityTypeId::Component(component_ty) => self.component_manager.has(component_ty),
                };
        }
        false
    }
}

#[async_trait]
impl Lifecycle for RelationTypeManagerImpl {
    async fn shutdown(&self) {
        self.relation_types.clear()
    }
}

#[cfg(test)]
mod tests {
    use default_test::DefaultTest;

    use crate::TypeSystemImpl;
    use reactive_graph_graph::Component;
    use reactive_graph_graph::ComponentOrEntityTypeId;
    use reactive_graph_graph::ComponentTypeId;
    use reactive_graph_graph::ComponentTypeIdContainer;
    use reactive_graph_graph::ComponentTypeIds;
    use reactive_graph_graph::EntityType;
    use reactive_graph_graph::Extensions;
    use reactive_graph_graph::NamespacedTypeGetter;
    use reactive_graph_graph::PropertyType;
    use reactive_graph_graph::PropertyTypeContainer;
    use reactive_graph_graph::PropertyTypes;
    use reactive_graph_graph::RelationType;
    use reactive_graph_graph::RelationTypeId;
    use reactive_graph_test_utils::r_string;
    use reactive_graph_type_system_api::TypeSystem;

    #[test]
    fn test_register_relation_type() {
        reactive_graph_test_utils::init_logger();
        let type_system = reactive_graph_di::get_container::<TypeSystemImpl>();
        let entity_type_manager = type_system.get_entity_type_manager();
        let relation_type_manager = type_system.get_relation_type_manager();

        let outbound_type = entity_type_manager
            .register(EntityType::default_test())
            .expect("Failed to register outbound entity type");
        let outbound_ty: ComponentOrEntityTypeId = (&outbound_type).into();

        let inbound_type = entity_type_manager
            .register(EntityType::default_test())
            .expect("Failed to register inbound entity type");
        let inbound_ty: ComponentOrEntityTypeId = (&inbound_type).into();

        let relation_ty = RelationTypeId::default_test();

        let relation_type = RelationType::builder()
            .outbound_type(&outbound_ty)
            .ty(&relation_ty)
            .inbound_type(&inbound_ty)
            .build_with_defaults();

        let _relation_type = relation_type_manager.register(relation_type).expect("Failed to register relation type");

        assert!(relation_type_manager.has(&relation_ty), "The relation type should be registered.");

        let relation_type = relation_type_manager.get(&relation_ty).expect("Failed to get the relation type by type id");
        assert_eq!(relation_ty.namespace(), relation_type.namespace(), "The relation type's namespace mismatches");
        assert_eq!(relation_ty.type_name(), relation_type.type_name(), "The relation type's type_name mismatches");
    }

    #[test]
    fn test_create_and_delete_relation_type() {
        reactive_graph_test_utils::init_logger();
        let type_system = reactive_graph_di::get_container::<TypeSystemImpl>();
        let entity_type_manager = type_system.get_entity_type_manager();
        let relation_type_manager = type_system.get_relation_type_manager();

        let outbound_type = entity_type_manager
            .register(EntityType::default_test())
            .expect("Failed to register outbound entity type");
        let outbound_ty: ComponentOrEntityTypeId = (&outbound_type).into();

        let inbound_type = entity_type_manager
            .register(EntityType::default_test())
            .expect("Failed to register inbound entity type");
        let inbound_ty: ComponentOrEntityTypeId = (&inbound_type).into();

        let namespace = r_string();
        let type_name = r_string();
        let relation_ty = RelationTypeId::new_from_type(&namespace, &type_name);

        let relation_type = RelationType::builder()
            .outbound_type(&outbound_ty)
            .ty(&relation_ty)
            .inbound_type(&inbound_ty)
            .build_with_defaults();

        let _relation_type = relation_type_manager.register(relation_type).expect("Failed to register relation type");
        assert!(relation_type_manager.has(&relation_ty));
        assert!(relation_type_manager.has_by_type(&namespace, &type_name));

        let relation_type_2 = relation_type_manager.get(&relation_ty).expect("Failed to get relation type by type id");
        assert_eq!(namespace, relation_type_2.namespace(), "The relation type's namespace mismatches");
        assert_eq!(type_name, relation_type_2.type_name(), "The relation type's type_name mismatches");
        relation_type_manager.delete(&relation_ty).expect("Failed to delete relation type by type id");
        assert!(!relation_type_manager.has(&relation_ty), "Relation type should not be registered anymore");
        assert!(
            relation_type_manager.get(&relation_ty).is_none(),
            "It shouldn't be possible to get the relation type anymore because it's no more registered"
        );
    }

    #[test]
    fn test_get_relation_types() {
        reactive_graph_test_utils::init_logger();
        let type_system = reactive_graph_di::get_container::<TypeSystemImpl>();
        let entity_type_manager = type_system.get_entity_type_manager();
        let relation_type_manager = type_system.get_relation_type_manager();

        let outbound_type = entity_type_manager
            .register(EntityType::default_test())
            .expect("Failed to register outbound entity type");
        let outbound_ty: ComponentOrEntityTypeId = (&outbound_type).into();

        let inbound_type = entity_type_manager
            .register(EntityType::default_test())
            .expect("Failed to register inbound entity type");
        let inbound_ty: ComponentOrEntityTypeId = (&inbound_type).into();

        let namespace = r_string();
        let type_name = r_string();
        let relation_ty = RelationTypeId::new_from_type(&namespace, &type_name);

        let relation_type = RelationType::builder()
            .outbound_type(&outbound_ty)
            .ty(&relation_ty)
            .inbound_type(&inbound_ty)
            .build_with_defaults();

        let _relation_type = relation_type_manager.register(relation_type).expect("Failed to register relation type");

        let relation_types = relation_type_manager.get_all();
        assert_eq!(1, relation_types.len());
        for relation_type in relation_types.iter() {
            assert!(relation_type_manager.has(&relation_type.ty));
        }
    }

    #[test]
    fn test_register_relation_type_has_component() {
        reactive_graph_test_utils::init_logger();
        let type_system = reactive_graph_di::get_container::<TypeSystemImpl>();
        let component_manager = type_system.get_component_manager();
        let entity_type_manager = type_system.get_entity_type_manager();
        let relation_type_manager = type_system.get_relation_type_manager();

        let outbound_type = entity_type_manager
            .register(EntityType::default_test())
            .expect("Failed to register outbound entity type");
        let outbound_ty: ComponentOrEntityTypeId = (&outbound_type).into();

        let inbound_type = entity_type_manager
            .register(EntityType::default_test())
            .expect("Failed to register inbound entity type");
        let inbound_ty: ComponentOrEntityTypeId = (&inbound_type).into();

        let component = component_manager.register(Component::default_test()).expect("Failed to register component");
        let component_ty = component.ty.clone();

        let relation_type = RelationType::builder()
            .outbound_type(&outbound_ty)
            .ty(RelationTypeId::default_test())
            .inbound_type(&inbound_ty)
            .component(&component_ty)
            .properties(PropertyTypes::default_test())
            .extensions(Extensions::default_test())
            .build();
        let relation_ty = relation_type.ty.clone();

        let _relation_type = relation_type_manager.register(relation_type).expect("Failed to register relation type");

        let relation_type = relation_type_manager.get(&relation_ty).expect("Failed to get relation type");
        assert!(relation_type.is_a(&component_ty), "Relation type must contain the component");
        assert!(relation_type.components.contains(&component_ty), "Relation type components must contain the component");
        assert!(!relation_type.is_a(&ComponentTypeId::default_test()), "Relation type must not container another component");
    }

    #[test]
    fn test_register_relation_type_has_property() {
        reactive_graph_test_utils::init_logger();
        let type_system = reactive_graph_di::get_container::<TypeSystemImpl>();
        let entity_type_manager = type_system.get_entity_type_manager();
        let relation_type_manager = type_system.get_relation_type_manager();

        let outbound_type = entity_type_manager
            .register(EntityType::default_test())
            .expect("Failed to register outbound entity type");
        let outbound_ty: ComponentOrEntityTypeId = (&outbound_type).into();

        let inbound_type = entity_type_manager
            .register(EntityType::default_test())
            .expect("Failed to register inbound entity type");
        let inbound_ty: ComponentOrEntityTypeId = (&inbound_type).into();

        let property_type = PropertyType::default_test();
        let property_name = property_type.name.clone();

        let relation_type = RelationType::builder()
            .outbound_type(&outbound_ty)
            .ty(RelationTypeId::default_test())
            .inbound_type(&inbound_ty)
            .components(ComponentTypeIds::default_test())
            .property(property_type)
            // .properties(PropertyTypes::default_test())
            .extensions(Extensions::default_test())
            .build();
        let relation_ty = relation_type.ty.clone();

        let _relation_type = relation_type_manager.register(relation_type).expect("Failed to register relation type!");

        let relation_type = relation_type_manager.get(&relation_ty).expect("Failed to get relation type!");
        assert!(relation_type.has_own_property(&property_name), "The property is missing in the relation type!");
        assert!(!relation_type.has_own_property(r_string()), "The relation type should not have a non-existent property!");
    }
}
