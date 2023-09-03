use std::ops::Deref;
use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use log::trace;
use log::warn;
use serde_json::json;
use inexor_rgf_core_model::ExtensionContainer;

use crate::api::ComponentManager;
use crate::api::EntityTypeManager;
use crate::api::Lifecycle;
use crate::api::RelationTypeManager;
use crate::api::SystemEventManager;
use crate::di::*;
use crate::error::types::relation::RelationTypeRegistrationError;
use crate::error::types::relation::RelationTypeCreationError;
use crate::model::PropertyTypes;
use crate::model::Extensions;
use crate::model::ComponentTypeIds;
use crate::model::ComponentOrEntityTypeId;
use crate::model::ComponentTypeId;
use crate::model::Extension;
use crate::model::ExtensionTypeId;
use crate::model::NamespacedTypeComponentTypeIdContainer;
use crate::model::NamespacedTypeContainer;
use crate::model::NamespacedTypeExtensionContainer;
use crate::model::NamespacedTypeGetter;
use crate::model::NamespacedTypePropertyTypeContainer;
use crate::model::Namespaces;
use crate::model::PropertyType;
use crate::model::PropertyTypeContainer;
use crate::model::RelationType;
use crate::model::RelationTypeAddComponentError;
use crate::model::RelationTypeAddExtensionError;
use crate::model::RelationTypeAddPropertyError;
use crate::model::RelationTypeId;
use crate::model::RelationTypeIds;
use crate::model::RelationTypeMergeError;
use crate::model::RelationTypeRemoveComponentError;
use crate::model::RelationTypeRemoveExtensionError;
use crate::model::RelationTypeRemovePropertyError;
use crate::model::RelationTypes;
use crate::model::RelationTypeUpdateExtensionError;
use crate::model::RelationTypeUpdatePropertyError;
use crate::model::TypeDefinitionGetter;
use crate::model_runtime::EXTENSION_DIVERGENT;
use crate::plugins::RelationTypeProvider;
use crate::plugins::SystemEvent;

#[wrapper]
pub struct RelationTypesStorage(RelationTypes);

#[provides]
fn create_relation_type_storage() -> RelationTypesStorage {
    RelationTypesStorage(RelationTypes::new())
}

#[component]
pub struct RelationTypeManagerImpl {
    event_manager: Wrc<dyn SystemEventManager>,

    component_manager: Wrc<dyn ComponentManager>,

    entity_type_manager: Wrc<dyn EntityTypeManager>,

    relation_types: RelationTypesStorage,
}

#[async_trait]
#[provides]
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
        // relation_type
        //     .extensions
        //     .push();
        // self.relation_types.0.write().unwrap().push(relation_type.clone());
        self.relation_types.push(relation_type.clone());
        debug!("Registered relation type {}", &relation_ty);
        self.event_manager.emit_event(SystemEvent::RelationTypeCreated(relation_ty));
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

    fn create(
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

    fn merge(&self, relation_type_to_merge: RelationType) -> Result<RelationType, RelationTypeMergeError> {
        self.relation_types.merge(relation_type_to_merge)
        // let ty = relation_type_to_merge.ty;
        // if !self.has(&ty) {
        //     return Err(RelationTypeMergeError::RelationTypeDoesNotExists(ty));
        // }
        // for component_ty in relation_type_to_merge.components {
        //     let _ = self.add_component(&ty, &component_ty);
        // }
        // let mut guard = self.relation_types.0.write().unwrap();
        // let Some(relation_type) = guard.iter_mut().find(|r| r.ty == ty) else {
        //     return Err(RelationTypeMergeError::RelationTypeDoesNotExists(ty));
        // };
        // relation_type.outbound_type = relation_type_to_merge.outbound_type;
        // relation_type.inbound_type = relation_type_to_merge.inbound_type;
        // relation_type.description = relation_type_to_merge.description.clone();
        // relation_type.merge_properties(relation_type_to_merge.properties);
        // relation_type.merge_extensions(relation_type_to_merge.extensions);
        // Ok(relation_type.clone())
    }

    fn add_component(&self, relation_ty: &RelationTypeId, component_ty: &ComponentTypeId) -> Result<(), RelationTypeAddComponentError> {
        if !self.component_manager.has(component_ty) {
            return Err(RelationTypeAddComponentError::ComponentDoesNotExist(component_ty.clone()));
        }
        self.relation_types.add_component(relation_ty, component_ty)
        // let mut guard = self.relation_types.0.write().unwrap();
        // for relation_type in guard.iter_mut() {
        //     if &relation_type.ty == ty {
        //         if relation_type.is_a(component_ty) {
        //             return Err(RelationTypeComponentError::ComponentAlreadyAssigned);
        //         }
        //         match self.component_manager.get(component_ty) {
        //             Some(component) => {
        //                 relation_type.components.push(component_ty.clone());
        //                 relation_type.merge_properties(component.properties);
        //             }
        //             None => {
        //                 return Err(RelationTypeComponentError::ComponentDoesNotExist);
        //             }
        //         }
        //         self.event_manager
        //             .emit_event(SystemEvent::RelationTypeComponentAdded(relation_type.ty.clone(), component_ty.clone()));
        //     }
        // }
        // Ok(())
    }

    fn remove_component(&self, relation_ty: &RelationTypeId, component_ty: &ComponentTypeId) -> Result<ComponentTypeId, RelationTypeRemoveComponentError> {
        self.relation_types.remove_component(relation_ty, component_ty)
        // let mut guard = self.relation_types.0.write().unwrap();
        // for relation_type in guard.iter_mut() {
        //     if &relation_type.ty == ty {
        //         relation_type.components.retain(|c| c != component_ty);
        //         self.event_manager
        //             .emit_event(SystemEvent::RelationTypeComponentRemoved(relation_type.ty.clone(), component_ty.clone()));
        //     }
        // }
    }

    fn add_property(&self, relation_ty: &RelationTypeId, property: PropertyType) -> Result<PropertyType, RelationTypeAddPropertyError> {
        self.relation_types.add_property(relation_ty, property)
        // let mut guard = self.relation_types.0.write().unwrap();
        // for relation_type in guard.iter_mut() {
        //     if &relation_type.ty == ty {
        //         if relation_type.has_own_property(property.name.clone()) {
        //             return Err(RelationTypePropertyError::PropertyAlreadyExists);
        //         }
        //         relation_type.properties.push(property.clone());
        //         self.event_manager
        //             .emit_event(SystemEvent::RelationTypePropertyAdded(relation_type.ty.clone(), property.name.clone()));
        //     }
        // }
        // Ok(())
    }

    fn update_property(&self, relation_ty: &RelationTypeId, property_name: &str, property_type: PropertyType) -> Result<PropertyType, RelationTypeUpdatePropertyError> {
        self.relation_types.update_property(relation_ty, property_name, property_type)

    }

    fn remove_property(&self, relation_ty: &RelationTypeId, property_name: &str) -> Result<PropertyType, RelationTypeRemovePropertyError> {
        self.relation_types.remove_property(relation_ty, property_name)
        // let mut guard = self.relation_types.0.write().unwrap();
        // for relation_type in guard.iter_mut() {
        //     if &relation_type.ty == ty {
        //         relation_type.properties.retain(|property| property.name != property_name);
        //         self.event_manager
        //             .emit_event(SystemEvent::RelationTypePropertyRemoved(relation_type.ty.clone(), property_name.to_string()));
        //     }
        // }
    }

    fn add_extension(&self, relation_ty: &RelationTypeId, extension: Extension) -> Result<ExtensionTypeId, RelationTypeAddExtensionError> {
        self.relation_types.add_extension(relation_ty, extension)
        // let mut guard = self.relation_types.0.write().unwrap();
        // for relation_type in guard.iter_mut() {
        //     if &relation_type.ty == ty {
        //         let extension_ty = extension.ty.clone();
        //         if relation_type.has_own_extension(&extension_ty) {
        //             return Err(RelationTypeExtensionError::ExtensionAlreadyExists(extension_ty));
        //         }
        //         relation_type.extensions.push(extension.clone());
        //         self.event_manager
        //             .emit_event(SystemEvent::RelationTypeExtensionAdded(relation_type.ty.clone(), extension_ty));
        //     }
        // }
        // Ok(())
    }

    fn update_extension(&self, relation_ty: &RelationTypeId, extension_ty: &ExtensionTypeId, extension: Extension) -> Result<Extension, RelationTypeUpdateExtensionError> {
        self.relation_types.update_extension(relation_ty, extension_ty, extension)
    }

    fn remove_extension(&self, relation_ty: &RelationTypeId, extension_ty: &ExtensionTypeId) -> Result<Extension, RelationTypeRemoveExtensionError> {
        self.relation_types.remove_extension(relation_ty, extension_ty)
        // let mut guard = self.relation_types.0.write().unwrap();
        // for relation_type in guard.iter_mut() {
        //     if &relation_type.ty == ty {
        //         relation_type.extensions.retain(|extension| &extension.ty != extension_ty);
        //         self.event_manager
        //             .emit_event(SystemEvent::RelationTypeExtensionRemoved(relation_type.ty.clone(), extension_ty.clone()));
        //     }
        // }
    }

    // TODO: parameter "cascade": flow types and relation instances (and their dependencies) depends on a relation type
    fn delete(&self, ty: &RelationTypeId) -> Option<RelationType> {
        self.relation_types
            .remove(ty)
            .map(|(ty, entity_type)| {
                self.event_manager.emit_event(SystemEvent::RelationTypeDeleted(ty.clone()));
                entity_type
            })
            // .inspect(|x| {
            //     self.event_manager.emit_event(SystemEvent::RelationTypeDeleted(ty.clone()));
            // })
            // .map(|(_, relation_type)| relation_type)
        // if !self.has(ty) {
        //     return false;
        // }
        // self.relation_types.0.write().unwrap().retain(|relation_type| &relation_type.ty != ty);
        // self.event_manager.emit_event(SystemEvent::RelationTypeDeleted(ty.clone()));
        // true
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

    fn add_provider(&self, relation_type_provider: Arc<dyn RelationTypeProvider>) {
        for relation_type in relation_type_provider.get_relation_types() {
            trace!("Registering relation type: {}", relation_type.type_definition().to_string());
            if self.register(relation_type.clone()).is_err() {
                trace!("Merging relation type: {}", relation_type.type_definition().to_string());
                let _ = self.merge(relation_type);
            }
        }
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

    use crate::get_runtime;
    use crate::model::Component;
    use crate::model::ComponentOrEntityTypeId;
    use crate::model::ComponentTypeId;
    use crate::model::ComponentTypeIdContainer;
    use crate::model::ComponentTypeIds;
    use crate::model::EntityType;
    use crate::model::Extensions;
    use crate::model::NamespacedTypeGetter;
    use crate::model::PropertyType;
    use crate::model::PropertyTypeContainer;
    use crate::model::PropertyTypes;
    use crate::model::RelationType;
    use crate::model::RelationTypeId;
    use crate::test_utils::r_string;

    #[test]
    fn test_register_relation_type() {
        let runtime = get_runtime();
        let entity_type_manager = runtime.get_entity_type_manager();
        let relation_type_manager = runtime.get_relation_type_manager();

        let outbound_type = entity_type_manager.register(EntityType::default_test()).expect("Failed to register outbound entity type");
        let outbound_ty: ComponentOrEntityTypeId = (&outbound_type).into();

        let inbound_type = entity_type_manager.register(EntityType::default_test()).expect("Failed to register inbound entity type");
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
        let runtime = get_runtime();
        let entity_type_manager = runtime.get_entity_type_manager();
        let relation_type_manager = runtime.get_relation_type_manager();

        let outbound_type = entity_type_manager.register(EntityType::default_test()).expect("Failed to register outbound entity type");
        let outbound_ty: ComponentOrEntityTypeId = (&outbound_type).into();

        let inbound_type = entity_type_manager.register(EntityType::default_test()).expect("Failed to register inbound entity type");
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
        assert!(relation_type_manager.get(&relation_ty).is_none(), "It shouldn't be possible to get the relation type anymore because it's no more registered");
    }

    #[test]
    fn test_get_relation_types() {
        let runtime = get_runtime();
        let entity_type_manager = runtime.get_entity_type_manager();
        let relation_type_manager = runtime.get_relation_type_manager();

        let outbound_type = entity_type_manager.register(EntityType::default_test()).expect("Failed to register outbound entity type");
        let outbound_ty: ComponentOrEntityTypeId = (&outbound_type).into();

        let inbound_type = entity_type_manager.register(EntityType::default_test()).expect("Failed to register inbound entity type");
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
        let runtime = get_runtime();
        let component_manager = runtime.get_component_manager();
        let entity_type_manager = runtime.get_entity_type_manager();
        let relation_type_manager = runtime.get_relation_type_manager();

        let outbound_type = entity_type_manager.register(EntityType::default_test()).expect("Failed to register outbound entity type");
        let outbound_ty: ComponentOrEntityTypeId = (&outbound_type).into();

        let inbound_type = entity_type_manager.register(EntityType::default_test()).expect("Failed to register inbound entity type");
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
        let runtime = get_runtime();
        let entity_type_manager = runtime.get_entity_type_manager();
        let relation_type_manager = runtime.get_relation_type_manager();

        let outbound_type = entity_type_manager.register(EntityType::default_test()).expect("Failed to register outbound entity type");
        let outbound_ty: ComponentOrEntityTypeId = (&outbound_type).into();

        let inbound_type = entity_type_manager.register(EntityType::default_test()).expect("Failed to register inbound entity type");
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