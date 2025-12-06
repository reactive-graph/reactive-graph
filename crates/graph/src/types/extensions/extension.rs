use serde_json::json;
use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

use dashmap::DashMap;
use dashmap::iter::OwningIter;
use schemars::JsonSchema;
use schemars::Schema;
use schemars::SchemaGenerator;
use schemars::json_schema;
use schemars::schema_for;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde_json::Value;
use std::borrow::Cow;
use typed_builder::TypedBuilder;

use crate::AddExtensionError;
use crate::EntityType;
use crate::EntityTypeId;
use crate::ExtensionContainer;
use crate::ExtensionJsonSchemaError;
use crate::ExtensionTypeId;
use crate::ExtensionTypeIds;
use crate::JsonSchemaIdGetter;
use crate::Namespace;
use crate::NamespaceSegment;
use crate::NamespacedType;
use crate::NamespacedTypeContainer;
use crate::NamespacedTypeGetter;
use crate::RemoveExtensionError;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeDescriptionGetter;
use crate::TypeIdType;
use crate::UpdateExtensionError;
use schemars::consts::meta_schemas::DRAFT2020_12;

#[cfg(any(test, feature = "test"))]
use crate::NamespacedTypeError;
#[cfg(any(test, feature = "test"))]
use crate::PropertyInstances;
#[cfg(any(test, feature = "test"))]
use crate::PropertyTypes;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedType;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedTypeId;
#[cfg(any(test, feature = "test"))]
use reactive_graph_utils_test::DefaultFrom;
#[cfg(any(test, feature = "test"))]
use reactive_graph_utils_test::r_string;
#[cfg(any(test, feature = "table"))]
use tabled::Tabled;

/// Extension on a type. The extension allows to extend information
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TypedBuilder)]
#[cfg_attr(any(test, feature = "table"), derive(Tabled))]
pub struct Extension {
    /// The fully qualified namespace of the extension.
    #[serde(rename = "type")]
    #[schemars(required)]
    #[builder(setter(into))]
    #[cfg_attr(any(test, feature = "table"), tabled(rename = "Type", inline))]
    pub ty: ExtensionTypeId,

    /// The fully qualified namespace of the entity type which is the type constraint of the extension.
    #[serde(rename = "entity_type")]
    #[builder(default, setter(into))]
    #[cfg_attr(any(test, feature = "table"), tabled(rename = "EntityType", inline))]
    pub entity_ty: Option<EntityTypeId>,

    /// Textual description of the extension.
    #[serde(default = "String::new")]
    #[builder(default, setter(into))]
    #[cfg_attr(any(test, feature = "table"), tabled(rename = "Description"))]
    pub description: String,

    /// The extension as JSON representation.
    #[cfg_attr(any(test, feature = "table"), tabled(skip))]
    pub extension: Value,
}

impl Extension {
    /// Constructs an extension from the given namespaced type with the given description, components, properties and extensions.
    pub fn new<T: Into<ExtensionTypeId>, S: Into<String>>(ty: T, description: S, extension: Value) -> Extension {
        Extension {
            ty: ty.into(),
            entity_ty: None,
            description: description.into(),
            extension,
        }
    }

    /// Constructs an extension from the given namespaced type with the given description, components, properties and extensions.
    pub fn new_with_type_constraint<T: Into<ExtensionTypeId>, ET: Into<EntityTypeId>, S: Into<String>>(
        ty: T,
        entity_ty: ET,
        description: S,
        extension: Value,
    ) -> Extension {
        Extension {
            ty: ty.into(),
            entity_ty: Some(entity_ty.into()),
            description: description.into(),
            extension,
        }
    }

    pub fn json_schema(&self, entity_type: &EntityType) -> Result<Schema, ExtensionJsonSchemaError> {
        let Some(entity_ty) = &self.entity_ty else {
            return Err(ExtensionJsonSchemaError::NoSchemaEntityType(self.ty.clone()));
        };
        if &entity_type.ty != entity_ty {
            return Err(ExtensionJsonSchemaError::SchemaEntityTypeDoesNotMatch(
                self.ty.clone(),
                entity_ty.clone(),
                entity_type.ty.clone(),
            ));
        }

        let mut schema = schema_for!(Extension);
        let Some(json_schema) = schema.as_object_mut() else {
            return Err(ExtensionJsonSchemaError::SchemaModificationError);
        };

        json_schema.insert("$id".to_string(), self.json_schema_id().into());

        let Some(extension_properties) = json_schema.get_mut("properties").and_then(|p| p.as_object_mut()) else {
            return Err(ExtensionJsonSchemaError::SchemaModificationError);
        };

        let mut entity_type_properties = entity_type.properties.as_json_schema_properties();
        entity_type_properties.insert("$id".to_string(), self.json_schema_id_property());
        let mut required = entity_type.properties.names();
        required.sort();

        extension_properties.insert(
            "extension".to_string(),
            json!({
                "$schema": DRAFT2020_12,
                "$id": entity_type.json_schema_id(),
                "type": "object",
                "title": entity_type.type_name(),
                "description": entity_type.description(),
                "properties": entity_type_properties,
                "required": required,
            }),
        );
        Ok(schema)
    }
}

impl NamespacedTypeGetter for Extension {
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

impl TypeDefinitionGetter for Extension {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
    }

    fn type_id_type() -> TypeIdType {
        TypeIdType::Extension
    }
}

impl TypeDescriptionGetter for Extension {
    fn description(&self) -> String {
        self.description.clone()
    }
}

impl From<&Extension> for TypeDefinition {
    fn from(extension: &Extension) -> Self {
        extension.type_definition()
    }
}

impl AsRef<ExtensionTypeId> for Extension {
    fn as_ref(&self) -> &ExtensionTypeId {
        &self.ty
    }
}

impl PartialEq<ExtensionTypeId> for Extension {
    fn eq(&self, ty: &ExtensionTypeId) -> bool {
        self.ty == *ty
    }
}

impl PartialOrd<Self> for Extension {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Extension {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ty.cmp(&other.ty)
    }
}

impl Hash for Extension {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ty.hash(state);
        self.description.hash(state);
        // Ignore the extension content (for now)
        // let ext_str = serde_json::to_string(&self.extension);
        // ext_str.hash(state);
    }
}

#[derive(Clone, Debug, Default)]
pub struct Extensions(DashMap<ExtensionTypeId, Extension>);

impl Extensions {
    #[inline]
    pub fn new() -> Self {
        NamespacedTypeContainer::new()
    }

    #[inline]
    pub fn push<E: Into<Extension>>(&self, extension: E) -> Option<Extension> {
        NamespacedTypeContainer::push(self, extension)
    }

    pub fn extension<E: Into<Extension>>(self, extension: E) -> Self {
        self.push(extension);
        self
    }
}

impl ExtensionContainer for Extensions {
    fn has_own_extension(&self, ty: &ExtensionTypeId) -> bool {
        self.0.contains_key(ty)
    }

    fn get_own_extension(&self, ty: &ExtensionTypeId) -> Option<Extension> {
        self.0.get(ty).map(|e| e.value().clone())
    }

    fn add_extension<E: Into<Extension>>(&self, extension: E) -> Result<ExtensionTypeId, AddExtensionError> {
        let extension = extension.into();
        let ty = extension.ty.clone();
        if self.0.contains_key(&ty) {
            return Err(AddExtensionError::ExtensionAlreadyExist(ty));
        }
        self.push(extension);
        Ok(ty)
    }

    fn update_extension<T: Into<ExtensionTypeId>, E: Into<Extension>>(&self, ty: T, extension: E) -> Result<Extension, UpdateExtensionError> {
        let ty = ty.into();
        if !self.0.contains_key(&ty) {
            return Err(UpdateExtensionError::ExtensionDoesNotExist(ty));
        }
        let _ = self.0.remove(&ty);
        let extension = extension.into();
        self.push(extension.clone());
        Ok(extension)
    }

    fn remove_extension<T: Into<ExtensionTypeId>>(&self, ty: T) -> Result<Extension, RemoveExtensionError> {
        let ty = ty.into();
        self.0
            .remove(&ty)
            .map(|(_, extension)| extension)
            .ok_or(RemoveExtensionError::ExtensionDoesNotExist(ty))
    }

    fn merge_extensions<E: Into<Extensions>>(&mut self, extensions_to_merge: E) {
        let extensions_to_merge = extensions_to_merge.into();
        for (ty, extension_to_merge) in extensions_to_merge {
            if !self.0.contains_key(&ty) {
                self.push(extension_to_merge);
            } else if let Some(mut existing_extension) = self.0.get_mut(&ty) {
                existing_extension.description = extension_to_merge.description.clone();
                existing_extension.extension = extension_to_merge.extension.clone();
            }
        }
    }

    fn get_own_extensions_cloned(&self) -> Extensions {
        self.clone()
    }
}

impl NamespacedTypeContainer for Extensions {
    type TypeId = ExtensionTypeId;
    type TypeIds = ExtensionTypeIds;
    type Type = Extension;
}

impl Deref for Extensions {
    type Target = DashMap<ExtensionTypeId, Extension>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for Extensions {
    type Item = (ExtensionTypeId, Extension);
    type IntoIter = OwningIter<ExtensionTypeId, Extension>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for Extensions {
    fn eq(&self, other: &Self) -> bool {
        self.iter().all(|self_extension| {
            other
                .get(self_extension.key())
                .filter(|other_extension| other_extension.value() == self_extension.value())
                .is_some()
        }) && other.iter().all(|other_extension| {
            self.get(other_extension.key())
                .filter(|self_extension| self_extension.value() == other_extension.value())
                .is_some()
        })
    }
}

impl Eq for Extensions {}

impl Hash for Extensions {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.to_vec().iter().for_each(|extension| {
            // extension.name.hash(hasher);
            extension.hash(hasher);
            // TODO: hash extension.extension
        });
    }
}

impl Serialize for Extensions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.iter())
    }
}

impl<'de> Deserialize<'de> for Extensions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Vec::<Extension>::deserialize(deserializer)?.into())
    }
}

impl JsonSchema for Extensions {
    fn schema_name() -> Cow<'static, str> {
        "Extensions".into()
    }

    fn json_schema(schema_generator: &mut SchemaGenerator) -> Schema {
        let sub_schema: Schema = schema_generator.subschema_for::<Extension>();
        json_schema!({
            "type": "array",
            "items": sub_schema,
            "description": "Extensions",
        })
    }
}

impl From<Vec<Extension>> for Extensions {
    fn from(extensions: Vec<Extension>) -> Self {
        extensions.into_iter().collect()
    }
}

impl From<Extensions> for Vec<Extension> {
    fn from(extensions: Extensions) -> Self {
        extensions.to_vec()
        // extensions.into_iter().map(|(_, extension)| extension).collect()
    }
}

impl From<&Extensions> for Vec<Extension> {
    fn from(extensions: &Extensions) -> Self {
        extensions.0.iter().map(|extension| extension.clone()).collect()
    }
}

impl From<DashMap<ExtensionTypeId, Extension>> for Extensions {
    fn from(extensions: DashMap<ExtensionTypeId, Extension>) -> Self {
        Self(extensions)
    }
}

impl From<&DashMap<ExtensionTypeId, Extension>> for Extensions {
    fn from(extensions: &DashMap<ExtensionTypeId, Extension>) -> Self {
        Self(extensions.clone())
    }
}

impl From<Extensions> for DashMap<ExtensionTypeId, Extension> {
    fn from(extensions: Extensions) -> Self {
        extensions.0
    }
}

impl FromIterator<Extension> for Extensions {
    fn from_iter<I: IntoIterator<Item = Extension>>(iter: I) -> Self {
        let extensions = Extensions::new();
        for extension in iter {
            extensions.push(extension);
        }
        extensions
    }
}

#[cfg(any(test, feature = "test"))]
impl RandomNamespacedType for Extension {
    type Error = NamespacedTypeError;
    type TypeId = ExtensionTypeId;

    fn random_type() -> Result<Self, NamespacedTypeError> {
        Self::random_type_with_id(&ExtensionTypeId::random_type_id()?)
    }

    fn random_type_with_id(ty: &Self::TypeId) -> Result<Self, Self::Error> {
        let entity_ty = if rand::random_bool(0.2) {
            Some(EntityTypeId::random_type_id()?)
        } else {
            None
        };
        let extension = match &entity_ty {
            None => json!(r_string()),
            Some(entity_ty) => {
                let property_types = PropertyTypes::random_types_no_extensions();
                let property_instances = PropertyInstances::default_from(&property_types).property("$id", entity_ty.json_schema_id());
                json!(property_instances)
            }
        };
        Ok(Extension::builder()
            .ty(ty)
            .entity_ty(entity_ty)
            .description(r_string())
            .extension(extension)
            .build())
    }
}

#[cfg(test)]
mod tests {
    use crate::EntityInstance;
    use crate::EntityType;
    use crate::EntityTypeId;
    use crate::Extension;
    use crate::ExtensionTypeId;
    use crate::NamespacedTypeGetter;
    use crate::RandomNamespacedType;
    use crate::RandomNamespacedTypeId;
    use reactive_graph_utils_test::DefaultTryFrom;
    use reactive_graph_utils_test::r_string;
    use schemars::schema_for;
    use serde_json::json;

    #[test]
    fn build_extension_without_entity_type() {
        let description = r_string();
        let ty = ExtensionTypeId::random_type_id().unwrap();
        let extension = Extension::builder()
            .ty(&ty)
            .description(&description)
            .extension(json!({
                "hello": "World"
            }))
            .build();
        assert_eq!(ty.namespace(), extension.namespace());
        assert_eq!(ty.path(), extension.path());
        assert_eq!(ty.type_name(), extension.type_name());
        assert_eq!(description, extension.description);
        assert_eq!(None, extension.entity_ty);
    }

    #[test]
    fn build_extension_with_entity_type() {
        let description = r_string();
        let ty = ExtensionTypeId::random_type_id().unwrap();
        let entity_type = EntityType::random_type().unwrap();
        let extension_payload = EntityInstance::default_try_from(&entity_type).unwrap();
        let extension_value = serde_json::to_value(&extension_payload).unwrap();
        let extension = Extension::builder()
            .ty(&ty)
            .description(&description)
            .entity_ty(Some(entity_type.ty.clone()))
            .extension(extension_value.clone())
            .build();
        assert_eq!(ty.namespace(), extension.namespace());
        assert_eq!(ty.path(), extension.path());
        assert_eq!(ty.type_name(), extension.type_name());
        assert_eq!(description, extension.description);
        assert_eq!(Some(entity_type.ty), extension.entity_ty);
        assert_eq!(extension_value, extension.extension);
        assert_eq!(extension_payload, serde_json::from_value::<EntityInstance>(extension.extension).unwrap());
    }

    #[test]
    fn create_extension() {
        let ty = ExtensionTypeId::random_type_id().unwrap();
        let description = r_string();
        let json = json!({
            "hello": "World"
        });
        let extension = Extension::new(&ty, &description, json.clone());
        assert_eq!(ty.namespace(), extension.namespace());
        assert_eq!(ty.path(), extension.path());
        assert_eq!(ty.type_name(), extension.type_name());
        assert_eq!(None, extension.entity_ty);
        assert_eq!(description, extension.description);
        assert_eq!(json, extension.extension);
    }

    #[test]
    fn create_extension_with_type_constraint() {
        let ty = ExtensionTypeId::random_type_id().unwrap();
        let entity_ty = EntityTypeId::random_type_id().unwrap();
        let description = r_string();
        let json = json!({
            "hello": "World"
        });
        let extension = Extension::new_with_type_constraint(&ty, &entity_ty, &description, json.clone());
        assert_eq!(ty.namespace(), extension.namespace());
        assert_eq!(ty.path(), extension.path());
        assert_eq!(ty.type_name(), extension.type_name());
        assert_eq!(Some(entity_ty), extension.entity_ty);
        assert_eq!(description, extension.description);
        assert_eq!(json, extension.extension);
    }

    #[test]
    fn extension_json_schema() {
        let schema = schema_for!(Extension);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }

    #[test]
    fn extension_dynamic_json_schema() {
        let description = r_string();
        let ty = ExtensionTypeId::random_type_id().unwrap();
        let entity_type = EntityType::random_type().unwrap();
        let extension_payload = EntityInstance::default_try_from(&entity_type).unwrap();
        let extension_value = serde_json::to_value(&extension_payload).unwrap();
        let extension = Extension::builder()
            .ty(&ty)
            .description(&description)
            .entity_ty(Some(entity_type.ty.clone()))
            .extension(extension_value.clone())
            .build();
        let schema = extension
            .json_schema(&entity_type)
            .expect("Failed to generate dynamic json schema for extension!");
        println!("{}", serde_json::to_string_pretty(schema.as_value()).unwrap());
    }
}
