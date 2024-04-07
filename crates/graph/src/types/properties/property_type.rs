use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

use dashmap::DashMap;
use schemars::gen::SchemaGenerator;
use schemars::schema::InstanceType;
use schemars::schema::ObjectValidation;
use schemars::schema::Schema;
use schemars::schema::SchemaObject;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde_json::Value;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::AddExtensionError;
use crate::AddPropertyError;
use crate::DataType;
use crate::Extension;
use crate::ExtensionContainer;
use crate::ExtensionTypeId;
use crate::Extensions;
use crate::Mutability;
use crate::PropertyTypeContainer;
use crate::RemoveExtensionError;
use crate::RemovePropertyError;
use crate::SocketType;
use crate::UpdateExtensionError;
use crate::UpdatePropertyError;

pub static NAMESPACE_PROPERTY_TYPE: Uuid = Uuid::from_u128(0x1ab7c8109dcd11c180b400d02fd540c7);

/// Definition of a property. The definition contains
/// the name of the property, the data type and the socket
/// type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, TypedBuilder)]
pub struct PropertyType {
    /// The name of the property
    #[builder(setter(into))]
    pub name: String,

    /// The description of the property.
    #[serde(default = "String::new")]
    #[builder(default, setter(into))]
    pub description: String,

    /// The data type of the property
    pub data_type: DataType,

    /// Specifies the type of socket - either input socket or output socket or none
    #[serde(default = "SocketType::none")]
    #[builder(default = SocketType::None)]
    pub socket_type: SocketType,

    /// Specifies if the property is mutable.
    #[serde(default = "Mutability::mutable")]
    #[builder(default = Mutability::Mutable)]
    pub mutability: Mutability,

    /// Property specific extensions
    #[serde(default = "Extensions::new")]
    #[builder(default, setter(into))]
    pub extensions: Extensions,
}

impl PropertyType {
    pub fn new<S: Into<String>>(name: S, data_type: DataType) -> PropertyType {
        PropertyType {
            name: name.into(),
            description: String::new(),
            data_type,
            socket_type: SocketType::None,
            mutability: Mutability::Mutable,
            extensions: Extensions::new(),
        }
    }

    pub fn new_with_socket<S: Into<String>>(name: S, data_type: DataType, socket_type: SocketType) -> PropertyType {
        PropertyType {
            name: name.into(),
            description: String::new(),
            data_type,
            socket_type,
            mutability: Mutability::Mutable,
            extensions: Extensions::new(),
        }
    }

    pub fn input<S: Into<String>>(name: S, data_type: DataType) -> PropertyType {
        PropertyType {
            name: name.into(),
            description: String::new(),
            data_type,
            socket_type: SocketType::Input,
            mutability: Mutability::Mutable,
            extensions: Extensions::new(),
        }
    }

    pub fn output<S: Into<String>>(name: S, data_type: DataType) -> PropertyType {
        PropertyType {
            name: name.into(),
            description: String::new(),
            data_type,
            socket_type: SocketType::Output,
            mutability: Mutability::Immutable,
            extensions: Extensions::new(),
        }
    }

    pub fn new_with_all<S: Into<String>>(
        name: S,
        description: S,
        data_type: DataType,
        socket_type: SocketType,
        mutability: Mutability,
        extensions: Extensions,
    ) -> PropertyType {
        PropertyType {
            name: name.into(),
            description: description.into(),
            data_type,
            socket_type,
            mutability,
            extensions,
        }
    }

    pub fn bool<S: Into<String>>(name: S) -> PropertyType {
        PropertyType::new(name, DataType::Bool)
    }

    pub fn bool_input<S: Into<String>>(name: S) -> PropertyType {
        PropertyType::new_with_socket(name, DataType::Bool, SocketType::Input)
    }

    pub fn bool_output<S: Into<String>>(name: S) -> PropertyType {
        PropertyType::new_with_socket(name, DataType::Bool, SocketType::Output)
    }

    pub fn number<S: Into<String>>(name: S) -> PropertyType {
        PropertyType::new(name, DataType::Number)
    }

    pub fn number_input<S: Into<String>>(name: S) -> PropertyType {
        PropertyType::new_with_socket(name, DataType::Number, SocketType::Input)
    }

    pub fn number_output<S: Into<String>>(name: S) -> PropertyType {
        PropertyType::new_with_socket(name, DataType::Number, SocketType::Output)
    }

    pub fn string<S: Into<String>>(name: S) -> PropertyType {
        PropertyType::new(name, DataType::String)
    }

    pub fn string_input<S: Into<String>>(name: S) -> PropertyType {
        PropertyType::new_with_socket(name, DataType::String, SocketType::Input)
    }

    pub fn string_output<S: Into<String>>(name: S) -> PropertyType {
        PropertyType::new_with_socket(name, DataType::String, SocketType::Output)
    }

    pub fn array<S: Into<String>>(name: S) -> PropertyType {
        PropertyType::new(name, DataType::Array)
    }

    pub fn array_input<S: Into<String>>(name: S) -> PropertyType {
        PropertyType::new_with_socket(name, DataType::Array, SocketType::Input)
    }

    pub fn array_output<S: Into<String>>(name: S) -> PropertyType {
        PropertyType::new_with_socket(name, DataType::Array, SocketType::Output)
    }

    pub fn object<S: Into<String>>(name: S) -> PropertyType {
        PropertyType::new(name, DataType::Object)
    }

    pub fn object_input<S: Into<String>>(name: S) -> PropertyType {
        PropertyType::new_with_socket(name, DataType::Object, SocketType::Input)
    }

    pub fn object_output<S: Into<String>>(name: S) -> PropertyType {
        PropertyType::new_with_socket(name, DataType::Object, SocketType::Output)
    }

    /// Returns true, if the property contains an extension with the given type.
    pub fn has_extension(&self, ty: &ExtensionTypeId) -> bool {
        self.extensions.iter().any(|extension| &extension.ty == ty)
    }

    pub fn get_extension(&self, ty: &ExtensionTypeId) -> Option<Extension> {
        self.extensions.get(ty).map(|extension| extension.value().clone())
        // self.extensions.iter().find(|extension| &extension.ty == ty).cloned()
    }
}

impl ExtensionContainer for PropertyType {
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

impl PartialOrd<Self> for PropertyType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PropertyType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

pub trait PropertyTypeDefinition {
    /// The property name.
    fn property_name(&self) -> String;

    /// The default value of the property.
    fn default_value(&self) -> Value;
}

#[derive(Clone, Debug, Default)] // , Serialize, Deserialize
                                 // #[serde(serialize_with="serialize_property_types", deserialize_with="deserialize_property_types")]
pub struct PropertyTypes(DashMap<String, PropertyType>);

impl PropertyTypes {
    pub fn new() -> Self {
        Self(DashMap::new())
    }

    pub fn property<P: Into<PropertyType>>(self, property: P) -> Self {
        self.push(property);
        self
    }

    pub fn push<P: Into<PropertyType>>(&self, property: P) {
        let property = property.into();
        self.0.insert(property.name.clone(), property);
    }

    pub fn to_vec(&self) -> Vec<PropertyType> {
        let mut property_types: Vec<PropertyType> = self.0.iter().map(|property| property.value().clone()).collect();
        property_types.sort();
        property_types
    }
}

impl PropertyTypeContainer for PropertyTypes {
    fn has_own_property<S: Into<String>>(&self, property_name: S) -> bool {
        let property_name = property_name.into();
        self.0.contains_key(&property_name)
    }

    fn get_own_property<S: Into<String>>(&self, property_name: S) -> Option<PropertyType> {
        let property_name = property_name.into();
        self.0.get(&property_name).map(|p| p.value().clone())
    }

    fn add_property<S: Into<PropertyType>>(&self, property_type: S) -> Result<PropertyType, AddPropertyError> {
        let property_type = property_type.into();
        if self.0.contains_key(&property_type.name) {
            return Err(AddPropertyError::PropertyAlreadyExist(property_type.name));
        }
        self.push(property_type.clone());
        Ok(property_type)
    }

    fn update_property<N: Into<String>, S: Into<PropertyType>>(&self, property_name: N, property_type: S) -> Result<PropertyType, UpdatePropertyError> {
        let property_name = property_name.into();
        if !self.0.contains_key(&property_name) {
            return Err(UpdatePropertyError::PropertyDoesNotExist(property_name));
        }
        let _ = self.0.remove(&property_name);
        let property_type = property_type.into();
        self.push(property_type.clone());
        Ok(property_type)
    }

    fn remove_property<S: Into<String>>(&self, property_name: S) -> Result<PropertyType, RemovePropertyError> {
        let property_name = property_name.into();
        self.0
            .remove(&property_name)
            .map(|(_, property_type)| property_type)
            .ok_or(RemovePropertyError::PropertyDoesNotExist(property_name))
    }

    fn merge_properties<P: Into<PropertyTypes>>(&mut self, properties_to_merge: P) {
        let properties_to_merge = properties_to_merge.into();
        properties_to_merge.into_iter().for_each(|(property_name, property_to_merge)| {
            if !self.0.contains_key(&property_name) {
                // let p = property_to_merge;
                self.push(property_to_merge);
            } else if let Some(mut existing_property) = self.0.get_mut(&property_name) {
                existing_property.description = property_to_merge.description.clone();
                existing_property.data_type = property_to_merge.data_type;
                existing_property.socket_type = property_to_merge.socket_type;
                existing_property.mutability = property_to_merge.mutability;
                existing_property.merge_extensions(property_to_merge.extensions);
            }
        });
        // for property_to_merge in properties_to_merge.into() {
        //     if !self.has_own_property(&property_to_merge.name) {
        //         self.properties.push(property_to_merge);
        //     } else if let Some(existing_property) = self.properties.iter_mut().find(|p| p.name == property_to_merge.name) {
        //         existing_property.description = property_to_merge.description.clone();
        //         existing_property.data_type = property_to_merge.data_type;
        //         existing_property.socket_type = property_to_merge.socket_type;
        //         existing_property.mutability = property_to_merge.mutability;
        //         existing_property.merge_extensions(property_to_merge.extensions);
        //     }
        // }
    }
}

impl Deref for PropertyTypes {
    type Target = DashMap<String, PropertyType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PropertyTypes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for PropertyTypes {
    type Item = (String, PropertyType);
    type IntoIter = dashmap::iter::OwningIter<String, PropertyType>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for PropertyTypes {
    fn eq(&self, other: &Self) -> bool {
        self.iter().all(|self_property| {
            other
                .get(self_property.key())
                // .map(|other_property| other_property.value())
                .filter(|other_property| other_property.value() == self_property.value())
                .is_some()
        }) && other.iter().all(|other_property| {
            self.get(other_property.key())
                // .map(|self_property| self_property.value())
                .filter(|self_property| self_property.value() == other_property.value())
                .is_some()
        })
    }
}

impl Eq for PropertyTypes {}

impl Hash for PropertyTypes {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.to_vec().iter().for_each(|property| {
            // property.name.hash(hasher);
            property.hash(hasher);
        });
    }
}

impl Serialize for PropertyTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.iter())
    }
}

impl<'de> Deserialize<'de> for PropertyTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Vec::<PropertyType>::deserialize(deserializer)?.into())
    }
}

impl JsonSchema for PropertyTypes {
    fn schema_name() -> String {
        "PropertyTypes".to_owned()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        let subschema = gen.subschema_for::<Value>();
        SchemaObject {
            instance_type: Some(InstanceType::Object.into()),
            object: Some(Box::new(ObjectValidation {
                additional_properties: Some(Box::new(subschema)),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}

impl From<Vec<PropertyType>> for PropertyTypes {
    fn from(property_types: Vec<PropertyType>) -> Self {
        property_types.into_iter().collect()
    }
}

impl From<PropertyTypes> for Vec<PropertyType> {
    fn from(property_types: PropertyTypes) -> Self {
        property_types.into_iter().map(|(_, property_type)| property_type).collect()
    }
}

impl FromIterator<PropertyType> for PropertyTypes {
    fn from_iter<I: IntoIterator<Item = PropertyType>>(iter: I) -> Self {
        let properties = PropertyTypes::new();
        for property in iter {
            properties.push(property);
            // properties.insert(property.name.clone(), property);
        }
        properties
    }
}

#[cfg(any(test, feature = "test"))]
use default_test::DefaultTest;
#[cfg(any(test, feature = "test"))]
use reactive_graph_test_utils::r_string;
#[cfg(any(test, feature = "test"))]
use rand::Rng;

#[cfg(any(test, feature = "test"))]
impl DefaultTest for PropertyType {
    fn default_test() -> Self {
        PropertyType::builder()
            .name(r_string())
            .description(r_string())
            .data_type(DataType::default_test())
            .mutability(Mutability::default_test())
            .socket_type(SocketType::default_test())
            .extensions(Extensions::default_test())
            .build()
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for PropertyTypes {
    fn default_test() -> Self {
        let property_types = PropertyTypes::new();
        let mut rng = rand::thread_rng();
        for _ in 0..rng.gen_range(0..10) {
            property_types.push(PropertyType::default_test());
        }
        property_types
    }
}

#[cfg(any(test, feature = "test"))]
impl PropertyTypes {
    pub fn new_with_string_property<S: Into<String>>(property_name: S) -> Self {
        let property_types = PropertyTypes::new();
        property_types.push(PropertyType::new(property_name, DataType::String));
        property_types
    }
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;

    use crate::DataType;
    use crate::Extensions;
    use crate::Mutability;
    use crate::PropertyType;
    use crate::SocketType;
    use reactive_graph_test_utils::r_string;

    #[test]
    fn property_type_test() {
        let property_name = r_string();

        let property_type = PropertyType {
            name: property_name.clone(),
            description: String::new(),
            data_type: DataType::String,
            socket_type: SocketType::None,
            mutability: Mutability::Mutable,
            extensions: Extensions::new(),
        };

        assert_eq!(property_name.clone(), property_type.name);
        assert_eq!(DataType::String, property_type.data_type);
    }

    #[test]
    fn property_type_serde_test() {
        let property_name = r_string();

        let property_type = PropertyType {
            name: property_name.clone(),
            description: String::new(),
            data_type: DataType::String,
            socket_type: SocketType::None,
            mutability: Mutability::Mutable,
            extensions: Extensions::new(),
        };

        let result = serde_json::to_string_pretty(&property_type.clone());
        assert!(result.is_ok());
        let result_2 = serde_json::from_str(result.unwrap().as_str());
        assert!(result_2.is_ok());
        let property_type_2: PropertyType = result_2.unwrap();

        assert_eq!(property_name.clone(), property_type_2.name);
        assert_eq!(DataType::String, property_type_2.data_type);
        assert_eq!(SocketType::None, property_type_2.socket_type);
    }

    #[test]
    fn property_type_new_test() {
        let property_name = r_string();
        let property_type = PropertyType::new(property_name.clone(), DataType::String);
        assert_eq!(property_name.clone(), property_type.name);
        assert_eq!(DataType::String, property_type.data_type);
        assert_eq!(SocketType::None, property_type.socket_type);
    }

    #[test]
    fn property_type_new_with_socket_test() {
        let property_name = r_string();
        let property_type = PropertyType::new_with_socket(property_name.clone(), DataType::String, SocketType::Input);
        assert_eq!(property_name.clone(), property_type.name);
        assert_eq!(DataType::String, property_type.data_type);
        assert_eq!(SocketType::Input, property_type.socket_type);
    }

    #[test]
    fn property_type_input_socket_test() {
        let property_name = r_string();
        let property_type = PropertyType::input(property_name.clone(), DataType::String);
        assert_eq!(property_name.clone(), property_type.name);
        assert_eq!(DataType::String, property_type.data_type);
        assert_eq!(SocketType::Input, property_type.socket_type);
    }

    #[test]
    fn property_type_output_socket_test() {
        let property_name = r_string();
        let property_type = PropertyType::output(property_name.clone(), DataType::String);
        assert_eq!(property_name.clone(), property_type.name);
        assert_eq!(DataType::String, property_type.data_type);
        assert_eq!(SocketType::Output, property_type.socket_type);
    }

    #[test]
    fn property_type_new_with_all_test() {
        let property_name = r_string();
        let description = r_string();
        let property_type = PropertyType::new_with_all(
            property_name.clone(),
            description.clone(),
            DataType::String,
            SocketType::Input,
            Mutability::Mutable,
            Extensions::new(),
        );
        assert_eq!(property_name.clone(), property_type.name);
        assert_eq!(description.clone(), property_type.description);
        assert_eq!(DataType::String, property_type.data_type);
        assert_eq!(SocketType::Input, property_type.socket_type);
    }

    #[test]
    fn property_type_bool_test() {
        let property_name = r_string();
        let property_type = PropertyType::bool(&property_name);
        assert_eq!(property_name, property_type.name);
        assert_eq!(DataType::Bool, property_type.data_type);
        assert_eq!(SocketType::None, property_type.socket_type);
    }

    #[test]
    fn property_type_number_test() {
        let property_name = r_string();
        let property_type = PropertyType::number(&property_name);
        assert_eq!(property_name, property_type.name);
        assert_eq!(DataType::Number, property_type.data_type);
        assert_eq!(SocketType::None, property_type.socket_type);
    }

    #[test]
    fn property_type_string_test() {
        let property_name = r_string();
        let property_type = PropertyType::string(&property_name);
        assert_eq!(property_name, property_type.name);
        assert_eq!(DataType::String, property_type.data_type);
        assert_eq!(SocketType::None, property_type.socket_type);
    }

    #[test]
    fn property_type_array_test() {
        let property_name = r_string();
        let property_type = PropertyType::array(&property_name);
        assert_eq!(property_name, property_type.name);
        assert_eq!(DataType::Array, property_type.data_type);
        assert_eq!(SocketType::None, property_type.socket_type);
    }

    #[test]
    fn property_type_object_test() {
        let property_name = r_string();
        let property_type = PropertyType::object(&property_name);
        assert_eq!(property_name, property_type.name);
        assert_eq!(DataType::Object, property_type.data_type);
        assert_eq!(SocketType::None, property_type.socket_type);
    }

    #[test]
    fn property_type_json_schema() {
        let schema = schema_for!(PropertyType);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
