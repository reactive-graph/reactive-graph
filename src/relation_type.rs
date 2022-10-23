use serde::Deserialize;
use serde::Serialize;

use crate::extension::Extension;
use crate::ComponentType;
use crate::EntityTypeType;
use crate::ExtensionContainer;
use crate::NamespacedTypeGetter;
use crate::PropertyType;
use crate::RelationTypeType;
use crate::TypeContainer;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeOfType;

/// A relation type defines the type of an relation instance.
///
/// The relation type defines the entity types of the outbound and inbound entity instances.
/// Also the relation type defines the properties of the relation instance.
#[derive(Clone, Debug)]
pub struct RelationType {
    /// The name of the outbound entity type.
    pub outbound_type: EntityTypeType,

    /// The type definition contains the namespace and the type name.
    pub ty: RelationTypeType,

    /// The instance type name is unique between two entity instances and is set for a
    /// concrete relation instance.
    /// TODO: RelationInstanceType
    pub instance_type_name: String,

    /// The name of the inbound entity type.
    pub inbound_type: EntityTypeType,

    /// Textual description of the relation type.
    pub description: String,

    /// The names of the components of the relation type.
    pub components: Vec<ComponentType>,

    /// The properties which are defined by the relation type.
    pub properties: Vec<PropertyType>,

    /// Relation type specific extensions
    pub extensions: Vec<Extension>,
}

impl RelationType {
    #[allow(clippy::too_many_arguments)]
    pub fn new<OT: Into<EntityTypeType>, RT: Into<RelationTypeType>, IT: Into<EntityTypeType>, S: Into<String>>(
        outbound_type: OT,
        ty: RT,
        inbound_type: IT,
        description: S,
        components: Vec<ComponentType>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> RelationType {
        let ty = ty.into();
        let type_name = ty.type_name();
        RelationType {
            ty,
            outbound_type: outbound_type.into(),
            instance_type_name: type_name,
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
        components: Vec<ComponentType>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> RelationType {
        let namespace = namespace.into();
        let outbound_type = outbound_type.into();
        let type_name = type_name.into();
        let inbound_type = inbound_type.into();
        let outbound_type = EntityTypeType::new_from_type(&namespace, &outbound_type);
        let ty = RelationTypeType::new_from_type(&namespace, &type_name);
        let inbound_type = EntityTypeType::new_from_type(&namespace, &inbound_type);
        RelationType {
            ty,
            outbound_type,
            instance_type_name: type_name,
            inbound_type,
            description: description.into(),
            components,
            properties,
            extensions,
        }
    }
}

impl TypeContainer for RelationType {
    fn is_a(&self, ty: &ComponentType) -> bool {
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
            type_type: TypeOfType::RelationType,
            namespace: relation_type.namespace(),
            type_name: relation_type.type_name(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelationTypeDao {
    /// The namespace the outbound entity type.
    #[serde(default = "String::new")]
    pub outbound_namespace: String,

    /// The type name of the outbound entity type.
    pub outbound_type_name: String,

    /// The namespace the relation type.
    #[serde(default = "String::new")]
    pub namespace: String,

    /// The type name of the relation type.
    #[serde(alias = "name")]
    pub type_name: String,

    /// The instance type name is unique between two entity instances and is set for a
    /// concrete relation instance.
    #[serde(default = "String::new")]
    pub instance_type_name: String,

    /// The namespace the inbound entity type.
    #[serde(default = "String::new")]
    pub inbound_namespace: String,

    /// The type name of the inbound entity type.
    pub inbound_type_name: String,

    /// Textual description of the relation type.
    #[serde(default = "String::new")]
    pub description: String,

    /// The names of the components of the relation type.
    #[serde(default = "Vec::new")]
    pub components: Vec<String>,

    /// The properties which are defined by the relation type.
    #[serde(default = "Vec::new")]
    pub properties: Vec<PropertyType>,

    /// Relation type specific extensions
    #[serde(default = "Vec::new")]
    pub extensions: Vec<Extension>,
}

impl From<&RelationTypeDao> for RelationType {
    fn from(dao: &RelationTypeDao) -> Self {
        let outbound_type = EntityTypeType::new_from_type(&dao.outbound_namespace, &dao.outbound_type_name);
        let ty = RelationTypeType::new_from_type(&dao.namespace, &dao.type_name);
        let inbound_type = EntityTypeType::new_from_type(&dao.inbound_namespace, &dao.inbound_type_name);
        Self {
            outbound_type,
            ty,
            instance_type_name: dao.instance_type_name.clone(),
            inbound_type,
            description: dao.description.clone(),
            components: dao.components.iter().cloned().filter_map(|c| ComponentType::try_from(&c).ok()).collect(),
            properties: dao.properties.clone(),
            extensions: dao.extensions.clone(),
        }
    }
}

impl From<&RelationType> for RelationTypeDao {
    fn from(relation_type: &RelationType) -> Self {
        RelationTypeDao {
            outbound_namespace: relation_type.outbound_type.namespace(),
            outbound_type_name: relation_type.outbound_type.type_name(),
            namespace: relation_type.namespace(),
            type_name: relation_type.type_name(),
            instance_type_name: relation_type.instance_type_name.clone(),
            inbound_namespace: relation_type.inbound_type.namespace(),
            inbound_type_name: relation_type.inbound_type.type_name(),
            description: relation_type.description.clone(),
            components: relation_type.components.iter().cloned().map(|c| c.type_definition().to_string()).collect(),
            properties: relation_type.properties.clone(),
            extensions: relation_type.extensions.clone(),
        }
    }
}
