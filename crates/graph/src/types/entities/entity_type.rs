use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

use const_format::formatcp;
use dashmap::DashMap;
use dashmap::iter::OwningIter;
use schemars::JsonSchema;
use schemars::Schema;
use schemars::SchemaGenerator;
use schemars::json_schema;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;
use typed_builder::TypedBuilder;

use crate::AddExtensionError;
use crate::AddPropertyError;
use crate::ComponentTypeId;
use crate::ComponentTypeIdContainer;
use crate::ComponentTypeIds;
use crate::Components;
use crate::EntityComponentTypeId;
use crate::EntityComponentTypeIds;
use crate::EntityTypeAddComponentError;
use crate::EntityTypeAddExtensionError;
use crate::EntityTypeAddPropertyError;
use crate::EntityTypeId;
use crate::EntityTypeIds;
use crate::EntityTypeMergeComponentPropertiesError;
use crate::EntityTypeMergeError;
use crate::EntityTypeMergeExtensionsError;
use crate::EntityTypeMergePropertiesError;
use crate::EntityTypeRemoveComponentError;
use crate::EntityTypeRemoveExtensionError;
use crate::EntityTypeRemovePropertyError;
use crate::EntityTypeUpdateExtensionError;
use crate::EntityTypeUpdatePropertyError;
use crate::ExtensionContainer;
use crate::ExtensionTypeId;
use crate::Extensions;
use crate::JSON_SCHEMA_ID_URI_PREFIX;
use crate::NamespaceSegment;
use crate::NamespacedType;
use crate::NamespacedTypeComponentPropertiesContainer;
use crate::NamespacedTypeComponentTypeIdContainer;
use crate::NamespacedTypeContainer;
use crate::NamespacedTypeExtensionContainer;
use crate::NamespacedTypeGetter;
use crate::NamespacedTypeIdContainer;
use crate::NamespacedTypePropertyTypeContainer;
use crate::PropertyType;
use crate::PropertyTypeContainer;
use crate::PropertyTypes;
use crate::RemoveExtensionError;
use crate::RemovePropertyError;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeDefinitionJsonSchema;
use crate::TypeDefinitionJsonSchemaGetter;
use crate::TypeDescriptionGetter;
use crate::TypeIdType;
use crate::UpdateExtensionError;
use crate::UpdatePropertyError;
use crate::divergent::DivergentPropertyTypes;
use crate::extension::Extension;
use crate::namespace::Namespace;

#[cfg(any(test, feature = "test"))]
use crate::NamespacedTypeError;
#[cfg(any(test, feature = "test"))]
use crate::RandomChildType;
#[cfg(any(test, feature = "test"))]
use crate::RandomChildTypeId;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedType;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedTypeIds;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedTypes;
#[cfg(any(test, feature = "test"))]
use reactive_graph_utils_test::r_string;

pub const JSON_SCHEMA_ID_ENTITY_TYPE: &str = formatcp!("{}/entity-type.schema.json", JSON_SCHEMA_ID_URI_PREFIX);

/// Entity types defines the type of entity instance.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, TypedBuilder)]
#[serde(tag = "$id", rename = "https://schema.reactive-graph.io/schema/json/entity-type.schema.json")]
#[schemars(
    title = "EntityType",
    deny_unknown_fields,
    extend("$id" = JSON_SCHEMA_ID_ENTITY_TYPE),
    transform = add_json_schema_id_property
)]
pub struct EntityType {
    /// The fully qualified namespace of the entity type.
    #[serde(rename = "type")]
    #[builder(setter(into))]
    pub ty: EntityTypeId,

    /// Textual description of the entity type.
    #[serde(default = "String::new")]
    #[builder(default, setter(into))]
    pub description: String,

    /// The names of the components of the entity type.
    #[serde(default = "ComponentTypeIds::new")]
    #[builder(default, setter(into))]
    pub components: ComponentTypeIds,

    /// The properties which are defined by the entity type.
    #[serde(default = "PropertyTypes::new")]
    #[builder(default, setter(into))]
    pub properties: PropertyTypes,

    /// Entity type specific extensions.
    #[serde(default = "Extensions::new")]
    #[builder(default, setter(into))]
    pub extensions: Extensions,
}

impl EntityType {
    /// Constructs an entity type from the given namespaced type with the given description, components, properties and extensions.
    pub fn new<T: Into<EntityTypeId>, S: Into<String>, C: Into<ComponentTypeIds>, P: Into<PropertyTypes>, E: Into<Extensions>>(
        ty: T,
        description: S,
        components: C,
        properties: P,
        extensions: E,
    ) -> EntityType {
        EntityType {
            ty: ty.into(),
            description: description.into(),
            components: components.into(),
            properties: properties.into(),
            extensions: extensions.into(),
        }
    }

    // TODO: Experimental
    pub fn builder_from_ty<T: Into<EntityTypeId>>(ty: T) -> EntityTypeBuilder<((EntityTypeId,), (), (), (), ())> {
        EntityType::builder().ty(ty.into())
    }
}

impl ComponentTypeIdContainer for EntityType {
    fn is_a(&self, ty: &ComponentTypeId) -> bool {
        self.components.is_a(ty)
    }

    fn add_component<T: Into<ComponentTypeId>>(&self, ty: T) -> bool {
        self.components.add_component(ty)
    }

    fn add_components<C: Into<ComponentTypeIds>>(&mut self, components_to_add: C) {
        self.components.add_components(components_to_add)
    }

    fn remove_component(&self, ty: &ComponentTypeId) -> Option<ComponentTypeId> {
        self.components.remove(ty)
    }

    fn remove_components<C: Into<ComponentTypeIds>>(&mut self, components_to_remove: C) {
        self.components.remove_components(components_to_remove)
    }

    fn get_components_cloned(&self) -> ComponentTypeIds {
        self.components.clone()
    }
}

impl PropertyTypeContainer for EntityType {
    fn has_own_property<S: Into<String>>(&self, property_name: S) -> bool {
        self.properties.has_own_property(property_name)
    }

    fn get_own_property<S: Into<String>>(&self, property_name: S) -> Option<PropertyType> {
        self.properties.get_own_property(property_name)
    }

    fn add_property<S: Into<PropertyType>>(&self, property_type: S) -> Result<PropertyType, AddPropertyError> {
        self.properties.add_property(property_type)
    }

    fn update_property<N: Into<String>, S: Into<PropertyType>>(&self, property_name: N, property_type: S) -> Result<PropertyType, UpdatePropertyError> {
        self.properties.update_property(property_name, property_type)
    }

    fn remove_property<S: Into<String>>(&self, property_name: S) -> Result<PropertyType, RemovePropertyError> {
        self.properties.remove_property(property_name)
    }

    fn merge_properties<P: Into<PropertyTypes>>(&mut self, properties_to_merge: P) {
        self.properties.merge_properties(properties_to_merge)
    }

    fn merge_non_existent_properties<P: Into<PropertyTypes>>(&self, properties_to_merge: P) -> DivergentPropertyTypes {
        self.properties.merge_non_existent_properties(properties_to_merge)
    }

    fn get_own_properties_cloned(&self) -> PropertyTypes {
        self.properties.clone()
    }
}

impl ExtensionContainer for EntityType {
    fn has_own_extension(&self, ty: &ExtensionTypeId) -> bool {
        self.extensions.has_own_extension(ty)
    }

    fn get_own_extension(&self, ty: &ExtensionTypeId) -> Option<Extension> {
        self.extensions.get_own_extension(ty)
    }

    fn add_extension<E: Into<Extension>>(&self, extension: E) -> Result<ExtensionTypeId, AddExtensionError> {
        self.extensions.add_extension(extension)
    }

    fn update_extension<T: Into<ExtensionTypeId>, E: Into<Extension>>(&self, ty: T, extension: E) -> Result<Extension, UpdateExtensionError> {
        self.extensions.update_extension(ty, extension)
    }

    fn remove_extension<T: Into<ExtensionTypeId>>(&self, ty: T) -> Result<Extension, RemoveExtensionError> {
        self.extensions.remove_extension(ty)
    }

    fn merge_extensions<E: Into<Extensions>>(&mut self, extensions_to_merge: E) {
        self.extensions.merge_extensions(extensions_to_merge)
    }

    fn get_own_extensions_cloned(&self) -> Extensions {
        self.extensions.clone()
    }
}

impl NamespacedTypeGetter for EntityType {
    fn namespaced_type(&self) -> NamespacedType {
        self.ty.namespaced_type()
    }

    fn namespace(&self) -> Namespace {
        self.ty.namespace()
    }

    fn path(&self) -> Namespace {
        self.ty.path()
    }

    fn type_name(&self) -> NamespaceSegment {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for EntityType {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
    }

    fn type_id_type() -> TypeIdType {
        TypeIdType::EntityType
    }
}

impl TypeDescriptionGetter for EntityType {
    fn description(&self) -> String {
        self.description.clone()
    }
}

impl TypeDefinitionJsonSchemaGetter for EntityType {
    fn json_schema(&self) -> Schema {
        TypeDefinitionJsonSchema::from(self)
            .description(&self.description)
            .required_id_property("id")
            .into()
    }
}

impl AsRef<EntityTypeId> for EntityType {
    fn as_ref(&self) -> &EntityTypeId {
        &self.ty
    }
}

impl PartialEq<EntityTypeId> for EntityType {
    fn eq(&self, ty: &EntityTypeId) -> bool {
        self.ty == *ty
    }
}

impl PartialOrd<Self> for EntityType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EntityType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ty.cmp(&other.ty)
    }
}

impl From<&EntityType> for TypeDefinition {
    fn from(entity_type: &EntityType) -> Self {
        entity_type.type_definition()
    }
}

impl From<&EntityType> for Schema {
    fn from(entity_type: &EntityType) -> Self {
        entity_type.json_schema()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
// #[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct EntityTypes(DashMap<EntityTypeId, EntityType>);

impl EntityTypes {
    #[inline]
    pub fn new() -> Self {
        NamespacedTypeContainer::new()
    }

    #[inline]
    pub fn entity<E: Into<EntityType>>(self, entity_type: E) -> Self {
        NamespacedTypeContainer::push(&self, entity_type);
        self
    }

    #[inline]
    pub fn push<E: Into<EntityType>>(&self, entity_type: E) -> Option<EntityType> {
        NamespacedTypeContainer::push(self, entity_type)
    }

    pub fn merge<C: Into<EntityType>>(&self, entity_type_to_merge: C) -> Result<EntityType, EntityTypeMergeError> {
        let entity_type_to_merge = entity_type_to_merge.into();
        let Some(mut entity_type) = self.get_mut(&entity_type_to_merge.ty) else {
            return Err(EntityTypeMergeError::EntityTypeDoesNotExist(entity_type_to_merge.ty));
        };
        entity_type.description = entity_type_to_merge.description;
        entity_type.add_components(entity_type_to_merge.components);
        entity_type.merge_properties(entity_type_to_merge.properties);
        entity_type.merge_extensions(entity_type_to_merge.extensions);
        Ok(entity_type.clone())
    }

    #[inline]
    pub fn push_all<E: Into<Self>>(&self, entity_types: E) {
        NamespacedTypeContainer::push_all(self, entity_types.into())
    }
}

impl NamespacedTypeContainer for EntityTypes {
    type TypeId = EntityTypeId;
    type TypeIds = EntityTypeIds;
    type Type = EntityType;
}

impl NamespacedTypeComponentTypeIdContainer<EntityTypeId, EntityTypeAddComponentError, EntityTypeRemoveComponentError> for EntityTypes {
    fn get_by_having_component(&self, component_ty: &ComponentTypeId) -> Self {
        self.0
            .iter()
            .filter(|entity_type| entity_type.is_a(component_ty))
            .map(|entity_type| entity_type.value().clone())
            .collect()
    }

    fn add_component(&self, entity_ty: &EntityTypeId, component_ty: &ComponentTypeId) -> Result<(), EntityTypeAddComponentError> {
        let Some(entity_type) = self.0.get_mut(entity_ty) else {
            return Err(EntityTypeAddComponentError::EntityTypeDoesNotExist(entity_ty.clone()));
        };
        if entity_type.is_a(component_ty) {
            return Err(EntityTypeAddComponentError::IsAlreadyA(component_ty.clone()));
        }
        let _ = entity_type.add_component(component_ty);
        Ok(())
    }

    fn remove_component(&self, entity_ty: &EntityTypeId, component_ty: &ComponentTypeId) -> Result<ComponentTypeId, EntityTypeRemoveComponentError> {
        let Some(entity_type) = self.0.get_mut(entity_ty) else {
            return Err(EntityTypeRemoveComponentError::EntityTypeDoesNotExist(entity_ty.clone()));
        };
        entity_type
            .remove_component(component_ty)
            .ok_or(EntityTypeRemoveComponentError::IsNotA(component_ty.clone()))
    }

    fn get_component_type_ids(&self) -> ComponentTypeIds {
        self.0.iter().map(|entity_type| entity_type.components.clone()).collect()
        // let all_component_tys = ComponentTypeIds::new();
        // for component_tys in self.0.iter().map(|entity_type| entity_type.components.clone()) {
        //     all_component_tys.insert_all(component_tys);
        // }
        // all_component_tys
    }
}

impl
    NamespacedTypePropertyTypeContainer<
        EntityTypeId,
        EntityTypeAddPropertyError,
        EntityTypeUpdatePropertyError,
        EntityTypeRemovePropertyError,
        EntityTypeMergePropertiesError,
    > for EntityTypes
{
    fn add_property<P: Into<PropertyType>>(&self, entity_ty: &EntityTypeId, property_type: P) -> Result<PropertyType, EntityTypeAddPropertyError> {
        let Some(entity_type) = self.0.get_mut(entity_ty) else {
            return Err(EntityTypeAddPropertyError::EntityTypeDoesNotExist(entity_ty.clone()));
        };
        entity_type.add_property(property_type).map_err(EntityTypeAddPropertyError::AddPropertyError)
    }

    fn update_property<N: Into<String>, P: Into<PropertyType>>(
        &self,
        entity_ty: &EntityTypeId,
        property_name: N,
        property_type: P,
    ) -> Result<PropertyType, EntityTypeUpdatePropertyError> {
        let Some(entity_type) = self.0.get_mut(entity_ty) else {
            return Err(EntityTypeUpdatePropertyError::EntityTypeDoesNotExist(entity_ty.clone()));
        };
        entity_type
            .update_property(property_name, property_type)
            .map_err(EntityTypeUpdatePropertyError::UpdatePropertyError)
    }

    fn remove_property<P: Into<String>>(&self, entity_ty: &EntityTypeId, property_name: P) -> Result<PropertyType, EntityTypeRemovePropertyError> {
        let Some(entity_type) = self.0.get_mut(entity_ty) else {
            return Err(EntityTypeRemovePropertyError::EntityTypeDoesNotExist(entity_ty.clone()));
        };
        entity_type
            .remove_property(property_name)
            .map_err(EntityTypeRemovePropertyError::RemovePropertyError)
    }

    fn merge_properties<P: Into<PropertyTypes>>(&self, entity_ty: &EntityTypeId, properties_to_merge: P) -> Result<(), EntityTypeMergePropertiesError> {
        let Some(mut entity_type) = self.0.get_mut(entity_ty) else {
            return Err(EntityTypeMergePropertiesError::EntityTypeDoesNotExist(entity_ty.clone()));
        };
        entity_type.merge_properties(properties_to_merge);
        Ok(())
    }
}

impl NamespacedTypeComponentPropertiesContainer<EntityTypeId, EntityTypeMergeComponentPropertiesError> for EntityTypes {
    fn merge_component_properties<C: Into<Components>>(&self, components: C) -> Result<(), EntityTypeMergeComponentPropertiesError> {
        let components = components.into();
        let lookup_tys = components.type_ids();

        // First check without modification
        let missing_components: EntityComponentTypeIds = self
            .0
            .iter()
            .map(|entity_type| (entity_type.key().clone(), entity_type.components.clone()))
            .flat_map(|(entity_ty, component_tys)| {
                component_tys
                    .into_iter()
                    .map(move |component_ty| EntityComponentTypeId::new(entity_ty.clone(), component_ty.clone()))
            })
            .filter(|entity_component_ty| !lookup_tys.contains(&entity_component_ty.component_ty))
            .collect();
        if !missing_components.is_empty() {
            return Err(EntityTypeMergeComponentPropertiesError::ComponentDoesNotExist(missing_components));
        }

        self.0.iter_mut().for_each(|entity_type| {
            for component_ty in entity_type.components.iter() {
                if let Some(component) = components.get(component_ty.key()) {
                    entity_type.merge_non_existent_properties(component.properties);
                }
            }
        });
        Ok(())
    }
}

impl
    NamespacedTypeExtensionContainer<
        EntityTypeId,
        EntityTypeAddExtensionError,
        EntityTypeUpdateExtensionError,
        EntityTypeRemoveExtensionError,
        EntityTypeMergeExtensionsError,
    > for EntityTypes
{
    fn add_extension<E: Into<Extension>>(&self, entity_ty: &EntityTypeId, extension: E) -> Result<ExtensionTypeId, EntityTypeAddExtensionError> {
        let Some(entity_type) = self.0.get_mut(entity_ty) else {
            return Err(EntityTypeAddExtensionError::EntityTypeDoesNotExist(entity_ty.clone()));
        };
        entity_type.add_extension(extension).map_err(EntityTypeAddExtensionError::AddExtensionError)
    }

    fn update_extension<T: Into<ExtensionTypeId>, E: Into<Extension>>(
        &self,
        entity_ty: &EntityTypeId,
        extension_ty: T,
        extension: E,
    ) -> Result<Extension, EntityTypeUpdateExtensionError> {
        let Some(entity_type) = self.0.get_mut(entity_ty) else {
            return Err(EntityTypeUpdateExtensionError::EntityTypeDoesNotExist(entity_ty.clone()));
        };
        entity_type
            .update_extension(extension_ty, extension)
            .map_err(EntityTypeUpdateExtensionError::UpdateExtensionError)
    }

    fn remove_extension<T: Into<ExtensionTypeId>>(&self, entity_ty: &EntityTypeId, extension_ty: T) -> Result<Extension, EntityTypeRemoveExtensionError> {
        let Some(entity_type) = self.0.get_mut(entity_ty) else {
            return Err(EntityTypeRemoveExtensionError::EntityTypeDoesNotExist(entity_ty.clone()));
        };
        entity_type
            .remove_extension(extension_ty)
            .map_err(EntityTypeRemoveExtensionError::RemoveExtensionError)
    }

    fn merge_extensions<E: Into<Extensions>>(&mut self, entity_ty: &EntityTypeId, extensions_to_merge: E) -> Result<(), EntityTypeMergeExtensionsError> {
        let Some(mut entity_type) = self.0.get_mut(entity_ty) else {
            return Err(EntityTypeMergeExtensionsError::EntityTypeDoesNotExist(entity_ty.clone()));
        };
        entity_type.merge_extensions(extensions_to_merge);
        Ok(())
    }
}

impl Deref for EntityTypes {
    type Target = DashMap<EntityTypeId, EntityType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EntityTypes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for EntityTypes {
    type Item = (EntityTypeId, EntityType);
    type IntoIter = OwningIter<EntityTypeId, EntityType>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for EntityTypes {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().all(|self_entity_type| other.contains_key(&self_entity_type.ty))
            && other.iter().all(|other_entity_type| self.contains_key(&other_entity_type.ty))
    }
}

impl Eq for EntityTypes {}

impl Hash for EntityTypes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_vec().hash(state);
    }
}

impl JsonSchema for EntityTypes {
    fn schema_name() -> Cow<'static, str> {
        "EntityTypes".into()
    }

    fn json_schema(schema_generator: &mut SchemaGenerator) -> Schema {
        let sub_schema: Schema = schema_generator.subschema_for::<EntityType>();
        json_schema!({
            "type": "array",
            "items": sub_schema,
            "description": "Entity Types",
        })
    }
}

impl From<Vec<EntityType>> for EntityTypes {
    fn from(entity_types: Vec<EntityType>) -> Self {
        Self(entity_types.into_iter().map(|entity_type| (entity_type.ty.clone(), entity_type)).collect())
    }
}

impl From<EntityTypes> for Vec<EntityType> {
    fn from(entity_types: EntityTypes) -> Self {
        entity_types.to_vec()
    }
}

impl From<&EntityTypes> for Vec<EntityType> {
    fn from(entity_types: &EntityTypes) -> Self {
        entity_types.0.iter().map(|entity_type| entity_type.clone()).collect()
    }
}

impl From<DashMap<EntityTypeId, EntityType>> for EntityTypes {
    fn from(entity_types: DashMap<EntityTypeId, EntityType>) -> Self {
        Self(entity_types)
    }
}

impl From<&DashMap<EntityTypeId, EntityType>> for EntityTypes {
    fn from(entity_types: &DashMap<EntityTypeId, EntityType>) -> Self {
        Self(entity_types.clone())
    }
}

impl From<EntityTypes> for DashMap<EntityTypeId, EntityType> {
    fn from(entity_types: EntityTypes) -> Self {
        entity_types.0
    }
}

impl FromIterator<EntityType> for EntityTypes {
    fn from_iter<I: IntoIterator<Item = EntityType>>(iter: I) -> Self {
        let entity_types = Self::new();
        for entity_type in iter {
            entity_types.insert(entity_type.ty.clone(), entity_type);
        }
        entity_types
    }
}

// Experimental
impl EntityTypeBuilder<((EntityTypeId,), (), (), (), ())> {
    #[allow(clippy::type_complexity)]
    pub fn component<C: Into<ComponentTypeId>>(self, component_ty: C) -> EntityTypeBuilder<((EntityTypeId,), (), (ComponentTypeIds,), (), ())> {
        let (ty, description, _, properties, extensions) = self.fields;
        EntityTypeBuilder {
            fields: (ty, description, (ComponentTypeIds::new().component(component_ty),), properties, extensions),
            phantom: self.phantom,
        }
    }
}

// Experimental
impl EntityTypeBuilder<((EntityTypeId,), (), (ComponentTypeIds,), (), ())> {
    #[allow(clippy::type_complexity)]
    pub fn component<C: Into<ComponentTypeId>>(self, ty: C) -> EntityTypeBuilder<((EntityTypeId,), (), (ComponentTypeIds,), (), ())> {
        self.fields.2.0.insert(ty.into());
        self
    }
}

// Experimental
impl EntityTypeBuilder<((EntityTypeId,), (), (ComponentTypeIds,), (), ())> {
    #[allow(clippy::type_complexity)]
    pub fn property<P: Into<PropertyType>>(self, property: P) -> EntityTypeBuilder<((EntityTypeId,), (), (ComponentTypeIds,), (PropertyTypes,), ())> {
        let (ty, description, components, _, extensions) = self.fields;
        EntityTypeBuilder {
            fields: (ty, description, components, (PropertyTypes::new().property(property),), extensions),
            phantom: self.phantom,
        }
    }
}

// Experimental
impl EntityTypeBuilder<((EntityTypeId,), (), (ComponentTypeIds,), (PropertyTypes,), ())> {
    #[allow(clippy::type_complexity)]
    pub fn property<P: Into<PropertyType>>(self, property: P) -> EntityTypeBuilder<((EntityTypeId,), (), (ComponentTypeIds,), (PropertyTypes,), ())> {
        self.fields.3.0.push(property.into());
        self
    }
}

#[cfg(any(test, feature = "test"))]
impl RandomNamespacedType for EntityType {
    type Error = NamespacedTypeError;
    type TypeId = EntityTypeId;

    fn random_type() -> Result<Self, NamespacedTypeError> {
        Self::random_child_type(&Namespace::random_path()?)
    }

    fn random_type_with_id(ty: &Self::TypeId) -> Result<Self, Self::Error> {
        Ok(Self::builder()
            .ty(ty)
            .description(r_string())
            .components(ComponentTypeIds::random_type_ids()?)
            .properties(PropertyTypes::random_types(0..10)?)
            .extensions(Extensions::random_types(0..10)?)
            .build())
    }
}

#[cfg(any(test, feature = "test"))]
impl RandomChildType for EntityType {
    type Error = NamespacedTypeError;

    fn random_child_type(namespace: &Namespace) -> Result<Self, Self::Error> {
        Self::random_type_with_id(&NamespacedType::random_child_type_id(namespace)?.into())
    }
}

fn add_json_schema_id_property(schema: &mut Schema) {
    crate::json_schema::add_json_schema_id_property(schema, JSON_SCHEMA_ID_ENTITY_TYPE);
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;
    use std::str::FromStr;

    use crate::ComponentTypeId;
    use crate::ComponentTypeIdContainer;
    use crate::ComponentTypeIds;
    use crate::EntityType;
    use crate::EntityTypeId;
    use crate::Extension;
    use crate::ExtensionContainer;
    use crate::ExtensionTypeId;
    use crate::Extensions;
    use crate::NamespacedTypeGetter;
    use crate::PropertyType;
    use crate::PropertyTypeContainer;
    use crate::PropertyTypes;
    use crate::RandomNamespacedType;
    use crate::RandomNamespacedTypeId;
    use crate::RandomNamespacedTypeIds;
    use crate::RandomNamespacedTypes;
    use crate::TypeDefinitionJsonSchemaGetter;
    use reactive_graph_utils_test::r_string;

    #[test]
    fn build_entity_type_test() {
        let description = r_string();
        let ty = EntityTypeId::random_type_id().unwrap();
        let components = ComponentTypeIds::random_type_ids().unwrap();
        let properties = PropertyTypes::random_types(1..10).unwrap();
        let extensions = Extensions::random_types(1..2).unwrap();
        let entity_type = EntityType::builder()
            .ty(&ty)
            .description(&description)
            .components(components.clone())
            .properties(properties.clone())
            .extensions(extensions.clone())
            .build();
        assert_eq!(ty.namespace(), entity_type.namespace());
        assert_eq!(ty.path(), entity_type.path());
        assert_eq!(ty.type_name(), entity_type.type_name());
        assert_eq!(description, entity_type.description);
        assert_eq!(components.len(), entity_type.components.len());
        assert_eq!(properties.len(), entity_type.properties.len());
        assert_eq!(extensions.len(), entity_type.extensions.len());
    }

    #[test]
    fn create_entity_type_test() {
        let ty = EntityTypeId::random_type_id().unwrap();
        let description = r_string();
        let properties = PropertyTypes::random_types_no_extensions();
        let extensions = Extensions::random_types(1..2).unwrap();
        let components = ComponentTypeIds::random_type_ids().unwrap();
        let entity_type = EntityType::new(&ty, &description, components.clone(), properties.clone(), extensions.clone());
        assert_eq!(ty.namespace(), entity_type.namespace());
        assert_eq!(ty.path(), entity_type.path());
        assert_eq!(ty.type_name(), entity_type.type_name());
        assert_eq!(description, entity_type.description);
        assert_eq!(components.len(), entity_type.components.len());
        assert_eq!(components, entity_type.components);
        assert_eq!(properties.len(), entity_type.properties.len());
        assert_eq!(extensions.len(), entity_type.extensions.len());
        assert!(!entity_type.has_own_property(r_string()));
        assert!(!entity_type.has_own_extension(&ExtensionTypeId::random_type_id().unwrap()));
    }

    #[test]
    fn entity_type_is_a_test() {
        let component_ty = ComponentTypeId::from_str("fully::qualified::namespace::Component").unwrap();
        let component_tys = ComponentTypeIds::new().component(&component_ty);
        let entity_type = EntityType::builder()
            .ty(EntityTypeId::random_type_id().unwrap())
            .components(component_tys)
            .build();
        assert_eq!(1, entity_type.components.len());
        assert!(entity_type.is_a(&component_ty));
        assert!(!entity_type.is_a(&ComponentTypeId::random_type_id().unwrap()));
    }

    #[test]
    fn entity_type_has_property_test() {
        let property_name = r_string();
        let properties = PropertyTypes::new().property(PropertyType::string(&property_name));
        let entity_type = EntityType::builder().ty(EntityTypeId::random_type_id().unwrap()).properties(properties).build();
        assert_eq!(1, entity_type.properties.len());
        assert!(entity_type.has_own_property(property_name));
        assert!(!entity_type.has_own_property(r_string()));
    }

    #[test]
    fn entity_type_has_extension_test() {
        let extension = Extension::random_type().unwrap();
        let extensions = Extensions::new().extension(extension.clone());
        let entity_type = EntityType::builder().ty(EntityTypeId::random_type_id().unwrap()).extensions(extensions).build();
        assert_eq!(1, entity_type.extensions.len());
        assert!(entity_type.has_own_extension(&extension.ty));
        assert!(!entity_type.has_own_extension(&ExtensionTypeId::random_type_id().unwrap()));
    }

    #[test]
    fn entity_type_deserialize_fully_valid_test() {
        let entity_ty = EntityTypeId::from_str("fully::qualified::namespace::EntityType").unwrap();
        let component_ty = ComponentTypeId::from_str("fully::qualified::namespace::Component").unwrap();
        let extension_ty = ExtensionTypeId::from_str("fully::qualified::namespace::Extension").unwrap();
        let entity_type = serde_json::from_str::<EntityType>(
            r#"{
          "type": "fully::qualified::namespace::EntityType",
          "description": "d",
          "components": [
            "fully::qualified::namespace::Component"
          ],
          "properties": [
                {
                  "name": "property_name",
                  "data_type": "string",
                  "socket_type": "input"
                }
          ],
          "extensions": [
                {
                  "type": "fully::qualified::namespace::Extension",
                  "extension": ""
                }
          ]
        }"#,
        )
        .expect("Failed to deserialize entity type");
        assert_eq!(entity_ty, entity_type.ty);
        assert_eq!("d", entity_type.description);
        assert_eq!(1, entity_type.components.len());
        assert!(entity_type.is_a(&component_ty));
        assert_eq!(1, entity_type.properties.len());
        assert!(entity_type.get_own_property("property_name").is_some());
        assert_eq!(1, entity_type.extensions.len());
        assert!(entity_type.get_own_extension(&extension_ty).is_some());
    }

    #[test]
    fn entity_type_deserialize_description_optional_test() {
        assert!(
            serde_json::from_str::<EntityType>(
                r#"{
          "type": "fully::qualified::namespace::Type",
          "components": [],
          "properties": [],
          "extensions": []
        }"#
            )
            .expect("Failed to deserialize entity type")
            .description
            .is_empty()
        );
    }

    #[test]
    fn entity_type_deserialize_components_optional_test() {
        assert_eq!(
            0,
            serde_json::from_str::<EntityType>(
                r#"{
          "type": "fully::qualified::namespace::Type",
          "description": "d",
          "properties": [],
          "extensions": []
        }"#
            )
            .expect("Failed to deserialize entity type")
            .properties
            .len()
        );
    }

    #[test]
    fn entity_type_deserialize_invalid_component_test() {
        assert!(
            serde_json::from_str::<EntityType>(
                r#"{
          "type": "fully::qualified::namespace::Type",
          "description": "d",
          "components": [
            "type": "InvalidTypeName::namespace"
          ],
          "properties": [],
          "extensions": []
        }"#
            )
            .inspect_err(|e| println!("{}", e))
            .is_err()
        );
    }

    #[test]
    fn entity_type_deserialize_properties_optional_test() {
        assert_eq!(
            0,
            serde_json::from_str::<EntityType>(
                r#"{
          "type": "fully::qualified::namespace::Type",
          "description": "d",
          "components": [],
          "extensions": []
        }"#
            )
            .expect("Failed to deserialize entity type")
            .properties
            .len()
        );
    }

    #[test]
    fn entity_type_deserialize_invalid_property_test() {
        assert!(
            serde_json::from_str::<EntityType>(
                r#"{
          "type": "fully::qualified::namespace::Type",
          "description": "d",
          "components": [],
          "properties": [
                {
                  "name": "property_name",
                  "data_type": "strng",
                  "socket_type": "put"
                }
          ],
          "extensions": []
        }"#
            )
            .inspect_err(|e| println!("{}", e))
            .is_err()
        );
    }

    #[test]
    fn entity_type_deserialize_extensions_optional_test() {
        assert_eq!(
            0,
            serde_json::from_str::<EntityType>(
                r#"{
          "type": "fully::qualified::namespace::Type",
          "description": "d",
          "components": [],
          "properties": []
        }"#
            )
            .expect("Failed to deserialize entity type")
            .extensions
            .len()
        );
    }

    #[test]
    fn entity_type_deserialize_invalid_extension_test() {
        assert!(
            serde_json::from_str::<EntityType>(
                r#"{
          "type": "fully::qualified::namespace::Type",
          "description": "d",
          "components": [],
          "properties": [],
          "extensions": [
                {
                  "type": "InvalidTypeName::namespace",
                  "extension": ""
                }
          ]
        }"#
            )
            .inspect_err(|e| println!("{}", e))
            .is_err()
        );
    }

    #[test]
    fn entity_type_deserialize_invalid_namespace_test() {
        assert!(
            serde_json::from_str::<EntityType>(
                r#"{
          "type": "invalid::namespace",
          "description": "d",
          "components": [],
          "properties": [],
          "extensions": []
        }"#
            )
            .inspect_err(|e| println!("{}", e))
            .is_err()
        );
    }

    #[test]
    fn entity_type_deserialize_invalid_type_test() {
        assert!(
            serde_json::from_str::<EntityType>(
                r#"{
          "type": "InvalidTypeName",
          "description": "d",
          "components": [],
          "properties": [],
          "extensions": []
        }"#
            )
            .inspect_err(|e| println!("{}", e))
            .is_err()
        );
    }

    #[test]
    fn entity_type_deserialize_missing_type_test() {
        assert!(
            serde_json::from_str::<EntityType>(
                r#"{
          "description": "d",
          "components": [],
          "properties": [],
          "extensions": []
        }"#
            )
            .inspect_err(|e| println!("{}", e))
            .is_err()
        );
    }

    #[test]
    fn entity_type_ser_test() {
        let entity_type = EntityType::random_type().unwrap();
        println!("{}", serde_json::to_string_pretty(&entity_type).expect("Failed to serialize entity type"));
    }

    #[test]
    fn entity_type_json_schema() {
        let schema = schema_for!(EntityType);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }

    #[test]
    fn entity_type_dynamic_json_schema() {
        let entity_type = EntityType::random_type().unwrap();
        let schema = entity_type.json_schema();
        println!("{}", serde_json::to_string_pretty(schema.as_value()).unwrap());
    }
}
