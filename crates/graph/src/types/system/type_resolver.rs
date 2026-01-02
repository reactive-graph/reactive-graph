use crate::Component;
use crate::ComponentTypeId;
use crate::ComponentTypeIdContainer;
use crate::ComponentTypeIds;
use crate::Components;
use crate::EntityType;
use crate::EntityTypeId;
use crate::EntityTypeIds;
use crate::EntityTypes;
use crate::FlowType;
use crate::FlowTypeId;
use crate::FlowTypeIds;
use crate::FlowTypes;
use crate::PropertyType;
use crate::PropertyTypeContainer;
use crate::RelationType;
use crate::RelationTypeId;
use crate::RelationTypeIds;
use crate::RelationTypes;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;
use crate::TypeResolveError;
use crate::TypeResolveError::ComponentResolveError;
use crate::TypeResolveError::EntityTypeResolveError;
use crate::TypeResolveError::FlowTypeResolveError;
use crate::TypeResolveError::RelationTypeResolveError;
use crate::TypeSystem;
use dashmap::DashMap;
use std::ops::Deref;

pub struct TypeResolver {
    /// A map of crate names and type systems.
    type_systems: DashMap<String, TypeSystem>,
}

impl TypeResolver {
    pub fn new() -> Self {
        Self { type_systems: DashMap::new() }
    }

    pub fn from<TS: Into<TypeSystem>>(type_system: TS) -> Self {
        let resolver = Self::new();
        resolver.insert(String::new(), type_system.into());
        resolver
    }

    pub fn crate_name<T: TypeDefinitionGetter>(&self, ty: &T) -> Option<String> {
        let type_definition = ty.type_definition();
        match type_definition.type_id_type {
            TypeIdType::Behaviour => None,
            TypeIdType::Component => self.component_crate_name(&ComponentTypeId::new(type_definition.namespaced_type)),
            TypeIdType::EntityType => self.entity_type_crate_name(&EntityTypeId::new(type_definition.namespaced_type)),
            TypeIdType::Extension => None,
            TypeIdType::RelationType => self.relation_type_crate_name(&RelationTypeId::new(type_definition.namespaced_type)),
            TypeIdType::FlowType => self.flow_type_crate_name(&FlowTypeId::new(type_definition.namespaced_type)),
        }
    }

    pub fn component(&self, ty: &ComponentTypeId) -> Option<Component> {
        for type_system in self.type_systems.iter() {
            if let Some(component) = type_system.components().get(ty) {
                return Some(component.clone());
            }
        }
        None
    }

    pub fn components(&self, tys: &ComponentTypeIds) -> Result<Components, TypeResolveError> {
        let components = Components::new();
        for ty in tys.iter() {
            components.push(self.component(&ty).ok_or(ComponentResolveError(ty.clone()))?);
        }
        Ok(components)
    }

    pub fn component_crate_name(&self, ty: &ComponentTypeId) -> Option<String> {
        for type_system in self.type_systems.iter() {
            if type_system.components().contains_key(ty) {
                return Some(type_system.key().clone());
            }
        }
        None
    }

    pub fn entity_type(&self, ty: &EntityTypeId) -> Option<EntityType> {
        for type_system in self.type_systems.iter() {
            if let Some(entity_type) = type_system.entity_types().get(ty) {
                return Some(entity_type.clone());
            }
        }
        None
    }

    pub fn entity_types(&self, tys: &EntityTypeIds) -> Result<EntityTypes, TypeResolveError> {
        let entity_types = EntityTypes::new();
        for ty in tys.iter() {
            entity_types.push(self.entity_type(&ty).ok_or(EntityTypeResolveError(ty.clone()))?);
        }
        Ok(entity_types)
    }

    pub fn entity_type_crate_name(&self, ty: &EntityTypeId) -> Option<String> {
        for type_system in self.type_systems.iter() {
            if type_system.entity_types().contains_key(ty) {
                return Some(type_system.key().clone());
            }
        }
        None
    }

    pub fn relation_type(&self, ty: &RelationTypeId) -> Option<RelationType> {
        for type_system in self.type_systems.iter() {
            if let Some(relation_type) = type_system.relation_types().get(ty) {
                return Some(relation_type.clone());
            }
        }
        None
    }

    pub fn relation_types(&self, tys: &RelationTypeIds) -> Result<RelationTypes, TypeResolveError> {
        let relation_types = RelationTypes::new();
        for ty in tys.iter() {
            relation_types.push(self.relation_type(&ty).ok_or(RelationTypeResolveError(ty.clone()))?);
        }
        Ok(relation_types)
    }

    pub fn relation_type_crate_name(&self, ty: &RelationTypeId) -> Option<String> {
        for type_system in self.type_systems.iter() {
            if type_system.relation_types().contains_key(ty) {
                return Some(type_system.key().clone());
            }
        }
        None
    }

    pub fn flow_type(&self, ty: &FlowTypeId) -> Option<FlowType> {
        for type_system in self.type_systems.iter() {
            if let Some(flow_type) = type_system.flow_types().get(ty) {
                return Some(flow_type.clone());
            }
        }
        None
    }

    pub fn flow_types(&self, tys: &FlowTypeIds) -> Result<FlowTypes, TypeResolveError> {
        let flow_types = FlowTypes::new();
        for ty in tys.iter() {
            flow_types.push(self.flow_type(&ty).ok_or(FlowTypeResolveError(ty.clone()))?);
        }
        Ok(flow_types)
    }

    pub fn flow_type_crate_name(&self, ty: &FlowTypeId) -> Option<String> {
        for type_system in self.type_systems.iter() {
            if type_system.flow_types().contains_key(ty) {
                return Some(type_system.key().clone());
            }
        }
        None
    }

    pub fn resolve_properties_sorted<TY: TypeDefinitionGetter + PropertyTypeContainer + ComponentTypeIdContainer>(
        &self,
        type_: &TY,
    ) -> Result<Vec<PropertyType>, TypeResolveError> {
        let properties = type_.get_own_properties_cloned();
        for (_, component) in self.components(&type_.get_components_cloned())? {
            properties.merge_non_existent_properties(component.properties);
        }
        let mut properties = properties.to_vec();
        properties.sort();
        Ok(properties)
    }
}

impl Deref for TypeResolver {
    type Target = DashMap<String, TypeSystem>;

    fn deref(&self) -> &Self::Target {
        &self.type_systems
    }
}

impl From<TypeSystem> for TypeResolver {
    fn from(type_system: TypeSystem) -> Self {
        let type_systems = DashMap::new();
        type_systems.insert(String::new(), type_system.into());
        Self { type_systems }
    }
}
