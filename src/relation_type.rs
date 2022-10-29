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
#[derive(Clone, Debug)]
pub struct RelationType {
    /// The name of the outbound entity type.
    pub outbound_type: EntityTypeId,

    /// The type definition contains the namespace and the type name.
    pub ty: RelationTypeId,

    /// The name of the inbound entity type.
    pub inbound_type: EntityTypeId,

    /// Textual description of the relation type.
    pub description: String,

    /// The names of the components of the relation type.
    pub components: Vec<ComponentTypeId>,

    /// The properties which are defined by the relation type.
    pub properties: Vec<PropertyType>,

    /// Relation type specific extensions
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
            ty,
            outbound_type: outbound_type.into(),
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
        let outbound_type = EntityTypeId::new_from_type(&dao.outbound_namespace, &dao.outbound_type_name);
        let ty = RelationTypeId::new_from_type(&dao.namespace, &dao.type_name);
        let inbound_type = EntityTypeId::new_from_type(&dao.inbound_namespace, &dao.inbound_type_name);
        Self {
            outbound_type,
            ty,
            inbound_type,
            description: dao.description.clone(),
            components: dao.components.iter().cloned().filter_map(|c| ComponentTypeId::try_from(&c).ok()).collect(),
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
            inbound_namespace: relation_type.inbound_type.namespace(),
            inbound_type_name: relation_type.inbound_type.type_name(),
            description: relation_type.description.clone(),
            components: relation_type.components.iter().cloned().map(|c| c.type_definition().to_string()).collect(),
            properties: relation_type.properties.clone(),
            extensions: relation_type.extensions.clone(),
        }
    }
}
