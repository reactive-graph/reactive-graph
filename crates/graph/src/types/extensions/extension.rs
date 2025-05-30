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
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde_json::Value;
#[cfg(any(test, feature = "test"))]
use serde_json::json;
use std::borrow::Cow;
use typed_builder::TypedBuilder;

use crate::AddExtensionError;
use crate::ExtensionContainer;
use crate::ExtensionTypeId;
use crate::ExtensionTypeIds;
use crate::NamespacedTypeContainer;
use crate::NamespacedTypeGetter;
use crate::RemoveExtensionError;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;
use crate::UpdateExtensionError;
#[cfg(any(test, feature = "test"))]
use reactive_graph_utils_test::r_string;

/// Extension on a type. The extension allows to extend information
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TypedBuilder)]
pub struct Extension {
    /// The type definition contains the namespace and the type name.
    #[serde(flatten)]
    #[schemars(required)]
    pub ty: ExtensionTypeId,

    /// Textual description of the extension.
    #[serde(default = "String::new")]
    pub description: String,

    /// The extension as JSON representation.
    pub extension: Value,
}

impl Extension {
    /// Constructs an extension from the given namespaced type with the given description, components, properties and extensions.
    pub fn new<T: Into<ExtensionTypeId>, S: Into<String>>(ty: T, description: S, extension: Value) -> Extension {
        Extension {
            ty: ty.into(),
            description: description.into(),
            extension,
        }
    }

    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S, description: S, extension: Value) -> Extension {
        Extension {
            ty: ExtensionTypeId::new_from_type(namespace, type_name),
            description: description.into(),
            extension,
        }
    }
}

impl NamespacedTypeGetter for Extension {
    fn namespace(&self) -> String {
        self.ty.namespace()
    }

    fn type_name(&self) -> String {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for Extension {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
    }
}

impl From<&Extension> for TypeDefinition {
    fn from(extension: &Extension) -> Self {
        TypeDefinition {
            type_id_type: TypeIdType::Extension,
            namespace: extension.namespace(),
            type_name: extension.type_name(),
        }
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
    pub fn new() -> Self {
        Extensions(DashMap::new())
    }

    pub fn extension<E: Into<Extension>>(self, extension: E) -> Self {
        self.push(extension);
        self
    }

    // pub fn push<E: Into<Extension>>(&self, extension: E) {
    //     let extension = extension.into();
    //     self.0.insert(extension.ty.clone(), extension);
    // }
    //
    // // TODO: Impl templated free function
    // // TODO: Rename to values() ?
    // pub fn to_vec(&self) -> Vec<Extension> {
    //     let mut extensions: Vec<Extension> = self.0.iter().map(|extension| extension.value().clone()).collect();
    //     extensions.sort();
    //     extensions
    // }
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
}

impl NamespacedTypeContainer for Extensions {
    type TypeId = ExtensionTypeId;
    type TypeIds = ExtensionTypeIds;
    type Type = Extension;

    fn new() -> Self {
        Self(DashMap::new())
    }

    fn push<E: Into<Extension>>(&self, extension: E) {
        let extension = extension.into();
        self.insert(extension.ty.clone(), extension);
    }
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
        extensions.into_iter().map(|(_, extension)| extension).collect()
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
impl DefaultTest for Extension {
    fn default_test() -> Self {
        Extension::builder()
            .ty(ExtensionTypeId::generate_random())
            .description(r_string())
            .extension(json!(r_string()))
            .build()
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for Extensions {
    fn default_test() -> Self {
        let extensions = Extensions::new();
        let mut rng = rand::rng();
        for _ in 0..rng.random_range(0..10) {
            extensions.push(Extension::default_test());
        }
        extensions
    }
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;

    use crate::Extension;

    #[test]
    fn extension_json_schema() {
        let schema = schema_for!(Extension);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
