use serde::Deserialize;
use serde::Serialize;

use crate::extension::Extension;
use crate::ComponentTypeId;
use crate::EntityTypeId;
use crate::ExtensionContainer;
use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::PropertyType;
use crate::RelationTypeId;
use crate::TypeContainer;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ComponentOrEntityTypeId {
    #[serde(rename = "component")]
    Component(ComponentTypeId),
    #[serde(rename = "entity_type")]
    EntityType(EntityTypeId),
}

impl From<ComponentTypeId> for ComponentOrEntityTypeId {
    fn from(ty: ComponentTypeId) -> Self {
        ComponentOrEntityTypeId::Component(ty)
    }
}

impl TryFrom<ComponentOrEntityTypeId> for ComponentTypeId {
    type Error = ();

    fn try_from(ty: ComponentOrEntityTypeId) -> Result<Self, Self::Error> {
        match ty {
            ComponentOrEntityTypeId::Component(ty) => Ok(ty),
            ComponentOrEntityTypeId::EntityType(_) => Err(()),
        }
    }
}

impl From<EntityTypeId> for ComponentOrEntityTypeId {
    fn from(ty: EntityTypeId) -> Self {
        ComponentOrEntityTypeId::EntityType(ty)
    }
}

impl TryFrom<ComponentOrEntityTypeId> for EntityTypeId {
    type Error = ();

    fn try_from(ty: ComponentOrEntityTypeId) -> Result<Self, Self::Error> {
        match ty {
            ComponentOrEntityTypeId::Component(_) => Err(()),
            ComponentOrEntityTypeId::EntityType(ty) => Ok(ty),
        }
    }
}

impl NamespacedTypeGetter for ComponentOrEntityTypeId {
    fn namespace(&self) -> String {
        match self {
            ComponentOrEntityTypeId::Component(ty) => ty.namespace(),
            ComponentOrEntityTypeId::EntityType(ty) => ty.namespace(),
        }
    }

    fn type_name(&self) -> String {
        match self {
            ComponentOrEntityTypeId::Component(ty) => ty.type_name(),
            ComponentOrEntityTypeId::EntityType(ty) => ty.type_name(),
        }
    }
}

impl TypeDefinitionGetter for ComponentOrEntityTypeId {
    fn type_definition(&self) -> TypeDefinition {
        match self {
            ComponentOrEntityTypeId::Component(ty) => ty.type_definition(),
            ComponentOrEntityTypeId::EntityType(ty) => ty.type_definition(),
        }
    }
}

impl From<&ComponentOrEntityTypeId> for TypeDefinition {
    fn from(ty: &ComponentOrEntityTypeId) -> Self {
        match ty {
            ComponentOrEntityTypeId::Component(ty) => ty.type_definition(),
            ComponentOrEntityTypeId::EntityType(ty) => ty.type_definition(),
        }
    }
}

impl From<&ComponentOrEntityTypeId> for NamespacedType {
    fn from(ty: &ComponentOrEntityTypeId) -> Self {
        match ty {
            ComponentOrEntityTypeId::Component(ty) => ty.into(),
            ComponentOrEntityTypeId::EntityType(ty) => ty.into(),
        }
    }
}

/// A relation type defines the type of an relation instance.
///
/// The relation type defines the entity types of the outbound and inbound entity instances.
/// Also the relation type defines the properties of the relation instance.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelationType {
    /// The outbound component or entity type.
    #[serde(rename = "outbound", alias = "outbound")]
    pub outbound_type: ComponentOrEntityTypeId,

    /// The type definition contains the namespace and the type name.
    #[serde(flatten)]
    pub ty: RelationTypeId,

    /// The inbound component or entity type.
    #[serde(rename = "inbound", alias = "inbound")]
    pub inbound_type: ComponentOrEntityTypeId,

    /// Textual description of the relation type.
    #[serde(default = "String::new")]
    pub description: String,

    /// The names of the components of the relation type.
    #[serde(default = "Vec::new")]
    pub components: Vec<ComponentTypeId>,

    /// The properties which are defined by the relation type.
    #[serde(default = "Vec::new")]
    pub properties: Vec<PropertyType>,

    /// Relation type specific extensions.
    #[serde(default = "Vec::new")]
    pub extensions: Vec<Extension>,
}

impl RelationType {
    #[allow(clippy::too_many_arguments)]
    pub fn new<OT: Into<ComponentOrEntityTypeId>, RT: Into<RelationTypeId>, IT: Into<ComponentOrEntityTypeId>, S: Into<String>>(
        outbound_type: OT,
        ty: RT,
        inbound_type: IT,
        description: S,
        components: Vec<ComponentTypeId>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> RelationType {
        let ty = ty.into();
        RelationType {
            outbound_type: outbound_type.into(),
            ty,
            inbound_type: inbound_type.into(),
            description: description.into(),
            components,
            properties,
            extensions,
        }
    }
}

impl TypeContainer for RelationType {
    fn is_a(&self, ty: &ComponentTypeId) -> bool {
        self.components.contains(ty)
    }

    fn has_own_property<S: Into<String>>(&self, property_name: S) -> bool {
        let property_name = property_name.into();
        self.properties.iter().any(|p| p.name == property_name)
    }

    fn get_own_property<S: Into<String>>(&self, property_name: S) -> Option<PropertyType> {
        let property_name = property_name.into();
        self.properties.iter().find(|p| p.name == property_name).cloned()
    }
}

impl ExtensionContainer for RelationType {
    fn has_own_extension<S: Into<String>>(&self, extension_name: S) -> bool {
        let extension_name = extension_name.into();
        self.extensions.iter().any(|extension| extension.name == extension_name)
    }

    fn get_own_extension<S: Into<String>>(&self, extension_name: S) -> Option<Extension> {
        let extension_name = extension_name.into();
        self.extensions.iter().find(|extension| extension.name == extension_name).cloned()
    }
}

impl NamespacedTypeGetter for RelationType {
    fn namespace(&self) -> String {
        self.ty.namespace()
    }

    fn type_name(&self) -> String {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for RelationType {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
    }
}

impl From<&RelationType> for TypeDefinition {
    fn from(relation_type: &RelationType) -> Self {
        TypeDefinition {
            type_id_type: TypeIdType::RelationType,
            namespace: relation_type.namespace(),
            type_name: relation_type.type_name(),
        }
    }
}
