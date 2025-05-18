use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

use dashmap::DashMap;
use dashmap::iter::OwningIter;
#[cfg(any(test, feature = "test"))]
use default_test::DefaultTest;
#[cfg(any(test, feature = "test"))]
use rand::Rng;
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
use crate::EntityTypeAddComponentError;
use crate::EntityTypeAddExtensionError;
use crate::EntityTypeAddPropertyError;
use crate::EntityTypeId;
use crate::EntityTypeIds;
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
use crate::NamespacedTypeComponentTypeIdContainer;
use crate::NamespacedTypeContainer;
use crate::NamespacedTypeExtensionContainer;
use crate::NamespacedTypeGetter;
use crate::NamespacedTypePropertyTypeContainer;
use crate::PropertyType;
use crate::PropertyTypeContainer;
use crate::PropertyTypes;
use crate::RemoveExtensionError;
use crate::RemovePropertyError;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;
use crate::UpdateExtensionError;
use crate::UpdatePropertyError;
use crate::extension::Extension;
#[cfg(any(test, feature = "test"))]
use reactive_graph_utils_test::r_string;

/// Entity types defines the type of entity instance.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, TypedBuilder)]
#[serde(tag = "$id", rename = "https://schema.reactive-graph.io/schema/json/entity-type.schema.json")]
#[schemars(title = "EntityType", deny_unknown_fields, extend("$id" = "https://schema.reactive-graph.io/schema/json/entity-type.schema.json"))]
pub struct EntityType {
    /// The type definition contains the namespace and the type name.
    #[serde(flatten)]
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

    pub fn new_from_type<N: Into<String>, T: Into<String>, D: Into<String>, C: Into<ComponentTypeIds>, P: Into<PropertyTypes>, E: Into<Extensions>>(
        namespace: N,
        type_name: T,
        description: D,
        components: C,
        properties: P,
        extensions: E,
    ) -> EntityType {
        EntityType {
            ty: EntityTypeId::new_from_type(namespace, type_name),
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
        self.properties.merge_properties(properties_to_merge);
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
}

impl NamespacedTypeGetter for EntityType {
    fn namespace(&self) -> String {
        self.ty.namespace()
    }

    fn type_name(&self) -> String {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for EntityType {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
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
        TypeDefinition {
            type_id_type: TypeIdType::EntityType,
            namespace: entity_type.namespace(),
            type_name: entity_type.type_name(),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
// #[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct EntityTypes(DashMap<EntityTypeId, EntityType>);

impl EntityTypes {
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
}

impl NamespacedTypeContainer for EntityTypes {
    type TypeId = EntityTypeId;
    type TypeIds = EntityTypeIds;
    type Type = EntityType;

    fn new() -> Self {
        Self(DashMap::new())
    }

    fn push<E: Into<EntityType>>(&self, entity_type: E) {
        let entity_type = entity_type.into();
        self.0.insert(entity_type.ty.clone(), entity_type);
    }
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
impl DefaultTest for EntityType {
    fn default_test() -> Self {
        EntityType::builder()
            .ty(EntityTypeId::default_test())
            .description(r_string())
            .components(ComponentTypeIds::default_test())
            .properties(PropertyTypes::default_test())
            .extensions(Extensions::default_test())
            .build()
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for EntityTypes {
    fn default_test() -> Self {
        let entity_types = EntityTypes::new();
        let mut rng = rand::rng();
        for _ in 0..rng.random_range(0..10) {
            entity_types.push(EntityType::default_test());
        }
        entity_types
    }
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;
    use serde_json::json;

    use crate::ComponentTypeId;
    use crate::ComponentTypeIdContainer;
    use crate::DataType;
    use crate::EntityType;
    use crate::EntityTypeId;
    use crate::Extension;
    use crate::ExtensionContainer;
    use crate::ExtensionTypeId;
    use crate::NamespacedTypeGetter;
    use crate::PropertyType;
    use crate::PropertyTypeContainer;
    use crate::SocketType;
    use crate::TypeDefinitionGetter;
    use reactive_graph_utils_test::r_string;

    #[test]
    fn create_entity_type_test() {
        let entity_type_name = r_string();

        let namespace = r_string();
        let description = r_string();

        let component_name = r_string();
        let mut component_names = Vec::new();
        let component_ty = ComponentTypeId::new_from_type(&namespace, &component_name);
        component_names.push(component_ty.clone());

        let mut property_types = Vec::new();
        let property_name = "property_name";
        let property_type = PropertyType::new(property_name, DataType::String);
        property_types.push(property_type.clone());

        let mut extensions = Vec::new();
        let extension_namespace = r_string();
        let extension_name = r_string();
        let extension_ty = ExtensionTypeId::new_from_type(&extension_namespace, &extension_name);
        let extension_value = json!("extension_value");
        let extension = Extension {
            ty: extension_ty.clone(),
            description: r_string(),
            extension: extension_value.clone(),
        };
        extensions.push(extension.clone());
        let other_extension_ty = ExtensionTypeId::new_from_type(&extension_namespace, &r_string());
        let other_extension = Extension::new(&other_extension_ty, r_string(), extension_value.clone());
        extensions.push(other_extension);

        let ty = EntityTypeId::new_from_type(&namespace, &entity_type_name);
        let entity_type = EntityType::new(ty, &description, component_names, property_types, extensions);

        assert_eq!(namespace, entity_type.namespace());

        assert_eq!(entity_type_name, entity_type.type_name());

        assert_eq!(format!("e__{}__{}", &namespace, &entity_type_name), entity_type.type_definition().to_string());

        assert_eq!(description, entity_type.description);

        assert!(entity_type.components.contains(&component_ty));
        assert!(entity_type.is_a(&component_ty));

        assert!(entity_type.properties.contains_key(property_name));
        assert!(entity_type.has_own_property(property_name));
        assert!(!entity_type.has_own_property(r_string()));
        assert_eq!(property_name, entity_type.get_own_property(property_name).unwrap().name);
        assert_eq!(property_type.data_type, entity_type.get_own_property(property_name).unwrap().data_type);
        assert_eq!(property_type, entity_type.get_own_property(property_name).unwrap());

        assert!(entity_type.extensions.contains_key(&extension_ty));
        assert!(entity_type.has_own_extension(&extension_ty));
        assert_eq!(extension_value, entity_type.get_own_extension(&extension_ty).unwrap().extension);
        assert_eq!(extension.extension, entity_type.get_own_extension(&extension_ty).unwrap().extension);

        let non_existing_extension = ExtensionTypeId::new_from_type(r_string(), r_string());
        assert!(!entity_type.extensions.contains_key(&non_existing_extension));
        assert!(!entity_type.has_own_extension(&non_existing_extension));
    }

    #[test]
    fn entity_type_ser_test() {
        let ty = EntityTypeId::new_from_type("ene", "ete");
        let et = EntityType::new(ty, "d", Vec::new(), Vec::new(), Vec::new());
        println!("{}", serde_json::to_string_pretty(&et).expect("Failed to serialize entity type"));
    }

    #[test]
    fn entity_type_de_test() {
        let s = r#"{
  "namespace": "abc",
  "type_name": "def",
  "description": "d",
  "components": [
    {
      "namespace": "mno",
      "type_name": "pqr"
    }
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
      "namespace": "ext_namespace",
      "type_name": "ext_name",
      "extension": "ext_value"
    }
  ]
}"#;
        let entity_type: EntityType = serde_json::from_str(s).unwrap();
        assert_eq!("abc", entity_type.namespace());
        assert_eq!("def", entity_type.type_name());
        assert_eq!("e__abc__def", entity_type.ty.to_string());
        assert_eq!("d", entity_type.description);
        assert_eq!(1, entity_type.components.len());
        let component = entity_type.components.get(&ComponentTypeId::new_from_type("mno", "pqr")).unwrap();
        assert_eq!("mno", component.namespace());
        assert_eq!("pqr", component.type_name());
        assert_eq!(1, entity_type.properties.len());
        let property = entity_type.get_own_property("property_name").unwrap();
        assert_eq!("property_name", property.name);
        assert_eq!(DataType::String, property.data_type);
        assert_eq!(SocketType::Input, property.socket_type);
        assert_eq!(1, entity_type.extensions.len());
        let extension = entity_type
            .get_own_extension(&ExtensionTypeId::new_from_type("ext_namespace", "ext_name"))
            .unwrap();
        assert_eq!("ext_namespace", extension.ty.namespace());
        assert_eq!("ext_name", extension.ty.type_name());
        assert_eq!(json!("ext_value"), extension.extension);
    }

    #[test]
    fn entity_type_json_schema() {
        let schema = schema_for!(EntityType);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
