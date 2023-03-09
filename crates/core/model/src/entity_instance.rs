use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;

use indradb::VertexProperties;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;
use uuid::Uuid;

use crate::EntityTypeId;
use crate::Extension;
use crate::ExtensionContainer;
use crate::ExtensionTypeId;
use crate::MutablePropertyInstanceSetter;
use crate::NamespacedTypeGetter;
use crate::PropertyInstanceGetter;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;

/// Entity instances represents an typed object which contains properties.
///
/// The entity type defines the properties (name, data type and socket type).
///
/// In contrast to the entity type the entity instance stores values in it's
/// properties.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EntityInstance {
    /// The type definition of the entity type.
    #[serde(flatten)]
    pub ty: EntityTypeId,

    /// The unique identifier of the entity instance.
    pub id: Uuid,

    /// The description of the entity instance.
    #[serde(default = "String::new")]
    pub description: String,

    /// The properties of then entity instance.
    ///
    /// Each property is represented by it's name (String) and it's value. The value is
    /// a representation of a JSON. Therefore the value can be boolean, number, string,
    /// array or an object. For more information about the data types please look at
    /// https://docs.serde.rs/serde_json/value/enum.Value.html
    #[serde(default = "HashMap::new")]
    pub properties: HashMap<String, Value>,

    /// Entity instance specific extensions.
    #[serde(default = "Vec::new")]
    pub extensions: Vec<Extension>,
}

impl EntityInstance {
    /// Constructs a new entity instance with the given type.
    pub fn new<T: Into<EntityTypeId>>(ty: T, id: Uuid, properties: HashMap<String, Value>) -> EntityInstance {
        EntityInstance {
            ty: ty.into(),
            id,
            description: String::new(),
            properties,
            extensions: Vec::new(),
        }
    }

    /// Constructs a new entity instance with the given namespace, type_name, id and properties.
    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S, id: Uuid, properties: HashMap<String, Value>) -> EntityInstance {
        EntityInstance {
            ty: EntityTypeId::new_from_type(namespace, type_name),
            id,
            description: String::new(),
            properties,
            extensions: Vec::new(),
        }
    }

    /// Constructs a new entity instance with the given type and id but without properties.
    pub fn new_without_properties<T: Into<EntityTypeId>>(ty: T, id: Uuid) -> EntityInstance {
        EntityInstance {
            ty: ty.into(),
            id,
            description: String::new(),
            properties: HashMap::new(),
            extensions: Vec::new(),
        }
    }
}

impl TryFrom<VertexProperties> for EntityInstance {
    type Error = ();

    fn try_from(properties: VertexProperties) -> Result<Self, Self::Error> {
        let ty = EntityTypeId::try_from(&properties.vertex.t)?;
        let id = properties.vertex.id;
        let properties: HashMap<String, Value> = properties.props.iter().map(|p| (p.name.to_string(), p.value.clone())).collect();
        Ok(EntityInstance {
            ty,
            id,
            description: String::new(),
            properties,
            extensions: Vec::new(),
        })
    }
}

impl PropertyInstanceGetter for EntityInstance {
    fn get<S: Into<String>>(&self, property_name: S) -> Option<Value> {
        self.properties.get(&property_name.into()).cloned()
    }

    fn as_bool<S: Into<String>>(&self, property_name: S) -> Option<bool> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_bool())
    }

    fn as_u64<S: Into<String>>(&self, property_name: S) -> Option<u64> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_u64())
    }

    fn as_i64<S: Into<String>>(&self, property_name: S) -> Option<i64> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_i64())
    }

    fn as_f64<S: Into<String>>(&self, property_name: S) -> Option<f64> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_f64())
    }

    fn as_string<S: Into<String>>(&self, property_name: S) -> Option<String> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_str().map(|s| s.to_string()))
    }

    fn as_array<S: Into<String>>(&self, property_name: S) -> Option<Vec<Value>> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_array().map(Vec::clone))
    }

    fn as_object<S: Into<String>>(&self, property_name: S) -> Option<Map<String, Value>> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_object().map(Map::clone))
    }
}

impl MutablePropertyInstanceSetter for EntityInstance {
    fn set<S: Into<String>>(&mut self, property_name: S, value: Value) {
        if let Some(property_value) = self.properties.get_mut(&property_name.into()) {
            *property_value = value
        }
    }
}

impl ExtensionContainer for EntityInstance {
    fn has_own_extension(&self, extension_ty: &ExtensionTypeId) -> bool {
        self.extensions.iter().any(|extension| &extension.ty == extension_ty)
    }

    fn get_own_extension(&self, extension_ty: &ExtensionTypeId) -> Option<Extension> {
        self.extensions.iter().find(|extension| &extension.ty == extension_ty).cloned()
    }
}

impl NamespacedTypeGetter for EntityInstance {
    fn namespace(&self) -> String {
        self.ty.namespace()
    }

    fn type_name(&self) -> String {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for EntityInstance {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
    }
}

impl Display for EntityInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}__{}", &self.ty, self.id)
    }
}
