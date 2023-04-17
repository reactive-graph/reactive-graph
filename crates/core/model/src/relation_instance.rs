use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;

use indradb::EdgeKey;
use indradb::EdgeProperties;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;
use uuid::Uuid;

use crate::Extension;
use crate::ExtensionContainer;
use crate::ExtensionTypeId;
use crate::MutablePropertyInstanceSetter;
use crate::NamespacedTypeGetter;
use crate::PropertyInstanceGetter;
use crate::RelationInstanceTypeId;
use crate::RelationTypeId;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;

/// Relation instances are edges from an outbound entity instance to an
/// inbound entity instance.
///
/// The relation instance is of a relation type. The relation type defines
/// the entity types of the outbound entity instance and the inbound entity
/// instance. Furthermore the relation type defines which properties
/// (name, data type, socket type) a relation instance have to have.
///
/// In contrast to the relation type, the relation instance stores values/
/// documents in it's properties.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelationInstance {
    /// The id of the outbound vertex.
    pub outbound_id: Uuid,

    /// The type definition of the relation instance type.
    #[serde(flatten)]
    pub ty: RelationInstanceTypeId,

    /// The id of the inbound vertex.
    pub inbound_id: Uuid,

    /// Textual description of the relation instance.
    #[serde(default = "String::new")]
    pub description: String,

    /// The properties of then relation instance.
    ///
    /// Each property is represented by it's name (String) and it's value. The value is
    /// a representation of a JSON. Therefore the value can be boolean, number, string,
    /// array or an object. For more information about the data types please look at
    /// <https://docs.serde.rs/serde_json/value/enum.Value.html>
    #[serde(default = "HashMap::new")]
    pub properties: HashMap<String, Value>,

    /// Relation instance specific extensions.
    #[serde(default = "Vec::new")]
    pub extensions: Vec<Extension>,
}

impl RelationInstance {
    /// Constructs a new relation instance with the given outbound_id, type, inbound_id and properties
    pub fn new<T: Into<RelationInstanceTypeId>>(outbound_id: Uuid, ty: T, inbound_id: Uuid, properties: HashMap<String, Value>) -> RelationInstance {
        RelationInstance {
            outbound_id,
            ty: ty.into(),
            inbound_id,
            description: String::new(),
            properties,
            extensions: Vec::new(),
        }
    }

    /// Constructs a new relation instance with the given outbound_id, type, inbound_id and properties
    pub fn new_from_type_unique_id<S: Into<String>>(
        namespace: S,
        outbound_id: Uuid,
        type_name: S,
        inbound_id: Uuid,
        properties: HashMap<String, Value>,
    ) -> RelationInstance {
        RelationInstance {
            outbound_id,
            ty: RelationInstanceTypeId::new_from_type_unique_id(namespace, type_name),
            inbound_id,
            description: String::new(),
            properties,
            extensions: Vec::new(),
        }
    }

    /// Constructs a new relation instance with the given outbound_id, type, inbound_id and properties
    pub fn new_from_type_unique_for_instance_id<S: Into<String>>(
        namespace: S,
        outbound_id: Uuid,
        type_name: S,
        instance_id: S,
        inbound_id: Uuid,
        properties: HashMap<String, Value>,
    ) -> RelationInstance {
        RelationInstance {
            outbound_id,
            ty: RelationInstanceTypeId::new_from_type_unique_for_instance_id(namespace, type_name, instance_id),
            inbound_id,
            description: String::new(),
            properties,
            extensions: Vec::new(),
        }
    }

    /// Constructs a new relation instance with the given outbound_id, type, inbound_id and properties
    pub fn new_from_type_with_random_instance_id<S: Into<String>>(
        namespace: S,
        outbound_id: Uuid,
        type_name: S,
        inbound_id: Uuid,
        properties: HashMap<String, Value>,
    ) -> RelationInstance {
        RelationInstance {
            outbound_id,
            ty: RelationInstanceTypeId::new_from_type_with_random_instance_id(namespace, type_name),
            inbound_id,
            description: String::new(),
            properties,
            extensions: Vec::new(),
        }
    }

    /// Constructs a new relation instance with the given outbound_id, type, inbound_id but without properties
    pub fn new_without_properties<T: Into<RelationInstanceTypeId>>(outbound_id: Uuid, ty: T, inbound_id: Uuid) -> RelationInstance {
        RelationInstance {
            outbound_id,
            ty: ty.into(),
            inbound_id,
            description: String::new(),
            properties: HashMap::new(),
            extensions: Vec::new(),
        }
    }

    /// Returns the inner relation type id.
    pub fn relation_type_id(&self) -> RelationTypeId {
        self.ty.relation_type_id()
    }

    /// Returns the relation instance type id.
    pub fn instance_id(&self) -> String {
        self.ty.instance_id()
    }

    /// Returns the edge key of the relation instance.
    pub fn get_key(&self) -> EdgeKey {
        EdgeKey::new(self.outbound_id, self.type_id(), self.inbound_id)
    }
}

impl TryFrom<EdgeProperties> for RelationInstance {
    type Error = ();

    fn try_from(properties: EdgeProperties) -> Result<Self, Self::Error> {
        let ty = RelationInstanceTypeId::try_from(&properties.edge.key.t)?;
        Ok(RelationInstance {
            outbound_id: properties.edge.key.outbound_id,
            ty,
            inbound_id: properties.edge.key.inbound_id,
            description: String::new(),
            properties: properties.props.iter().map(|p| (p.name.to_string(), p.value.clone())).collect(),
            extensions: Vec::new(),
        })
    }
}

impl PropertyInstanceGetter for RelationInstance {
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

impl MutablePropertyInstanceSetter for RelationInstance {
    fn set<S: Into<String>>(&mut self, property_name: S, value: Value) {
        if let Some(property_value) = self.properties.get_mut(&property_name.into()) {
            *property_value = value
        }
    }
}

impl ExtensionContainer for RelationInstance {
    fn has_own_extension(&self, extension_ty: &ExtensionTypeId) -> bool {
        self.extensions.iter().any(|extension| &extension.ty == extension_ty)
    }

    fn get_own_extension(&self, extension_ty: &ExtensionTypeId) -> Option<Extension> {
        self.extensions.iter().find(|extension| &extension.ty == extension_ty).cloned()
    }

    fn merge_extensions(&mut self, extensions_to_merge: Vec<Extension>) {
        for extension_to_merge in extensions_to_merge {
            if !self.has_own_extension(&extension_to_merge.ty) {
                self.extensions.push(extension_to_merge);
            } else if let Some(existing_extension) = self.extensions.iter_mut().find(|e| e.ty == extension_to_merge.ty) {
                existing_extension.description = extension_to_merge.description.clone();
                existing_extension.extension = extension_to_merge.extension.clone();
            }
        }
    }
}

impl NamespacedTypeGetter for RelationInstance {
    fn namespace(&self) -> String {
        self.ty.namespace()
    }

    fn type_name(&self) -> String {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for RelationInstance {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
    }
}

impl Display for RelationInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}--[{}]-->{}", self.outbound_id, &self.ty, self.inbound_id)
    }
}
