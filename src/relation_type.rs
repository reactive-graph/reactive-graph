use serde::Deserialize;
use serde::Serialize;

use crate::extension::Extension;
use crate::ComponentTypeId;
use crate::EntityTypeId;
use crate::ExtensionContainer;
use crate::NamespacedTypeGetter;
use crate::PropertyType;
use crate::RelationTypeId;
use crate::TypeContainer;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;

/// A relation type defines the type of an relation instance.
///
/// The relation type defines the entity types of the outbound and inbound entity instances.
/// Also the relation type defines the properties of the relation instance.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelationType {
    /// The name of the outbound entity type.
    #[serde(rename = "outbound", alias = "outbound")]
    pub outbound_type: EntityTypeId,

    /// The type definition contains the namespace and the type name.
    #[serde(flatten)]
    pub ty: RelationTypeId,

    /// The name of the inbound entity type.
    #[serde(rename = "inbound", alias = "inbound")]
    pub inbound_type: EntityTypeId,

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
    pub fn new<OT: Into<EntityTypeId>, RT: Into<RelationTypeId>, IT: Into<EntityTypeId>, S: Into<String>>(
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

    #[allow(clippy::too_many_arguments)]
    pub fn new_from_type<S: Into<String>>(
        namespace: S,
        outbound_type: S,
        type_name: S,
        inbound_type: S,
        description: S,
        components: Vec<ComponentTypeId>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> RelationType {
        let namespace = namespace.into();
        let outbound_type = outbound_type.into();
        let type_name = type_name.into();
        let inbound_type = inbound_type.into();
        let outbound_type = EntityTypeId::new_from_type(&namespace, &outbound_type);
        let ty = RelationTypeId::new_from_type(&namespace, &type_name);
        let inbound_type = EntityTypeId::new_from_type(&namespace, &inbound_type);
        RelationType {
            outbound_type,
            ty,
            inbound_type,
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
