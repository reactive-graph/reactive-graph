use serde::Deserialize;
use serde::Serialize;

use crate::extension::Extension;
use crate::ComponentType;
use crate::EntityTypeType;
use crate::ExtensionContainer;
use crate::NamespacedTypeGetter;
use crate::PropertyType;
use crate::TypeContainer;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeOfType;

/// Entity types defines the type of an entity instance.
#[derive(Clone, Debug)]
pub struct EntityType {
    /// The type definition contains the namespace and the type name.
    pub ty: EntityTypeType,

    /// Textual description of the entity type.
    pub description: String,

    /// The names of the components of the entity type.
    pub components: Vec<ComponentType>,

    /// The properties which are defined by the entity type.
    pub properties: Vec<PropertyType>,

    /// Entity type specific extensions
    pub extensions: Vec<Extension>,
}

impl EntityType {
    /// Constructs an entity type from the given namespaced type with the given description, components, properties and extensions.
    pub fn new<T: Into<EntityTypeType>, S: Into<String>>(
        ty: T,
        description: S,
        components: Vec<ComponentType>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> EntityType {
        EntityType {
            ty: ty.into(),
            description: description.into(),
            components,
            properties,
            extensions,
        }
    }

    pub fn new_from_type<S: Into<String>>(
        namespace: S,
        type_name: S,
        description: S,
        components: Vec<ComponentType>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> EntityType {
        EntityType {
            ty: EntityTypeType::new_from_type(namespace, type_name),
            description: description.into(),
            components,
            properties,
            extensions,
        }
    }
}

impl TypeContainer for EntityType {
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

impl ExtensionContainer for EntityType {
    fn has_own_extension<S: Into<String>>(&self, extension_name: S) -> bool {
        let extension_name = extension_name.into();
        self.extensions.iter().any(|extension| extension.name == extension_name)
    }

    fn get_own_extension<S: Into<String>>(&self, extension_name: S) -> Option<Extension> {
        let extension_name = extension_name.into();
        self.extensions.iter().find(|extension| extension.name == extension_name).cloned()
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

impl From<&EntityType> for TypeDefinition {
    fn from(entity_type: &EntityType) -> Self {
        TypeDefinition {
            type_type: TypeOfType::EntityType,
            namespace: entity_type.namespace(),
            type_name: entity_type.type_name(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct EntityTypeDao {
    /// The namespace the entity type belongs to.
    #[serde(default = "String::new")]
    pub namespace: String,

    /// The name of the entity type.
    ///
    /// The name is the unique identifier for entity types.
    #[serde(alias = "name")]
    pub type_name: String,

    /// Textual description of the entity type.
    #[serde(default = "String::new")]
    pub description: String,

    /// The names of the components of the entity type.
    #[serde(default = "Vec::new")]
    pub components: Vec<String>,

    /// The properties which are defined by the entity type.
    #[serde(default = "Vec::new")]
    pub properties: Vec<PropertyType>,

    /// Entity type specific extensions
    #[serde(default = "Vec::new")]
    pub extensions: Vec<Extension>,
}

impl From<&EntityTypeDao> for EntityType {
    fn from(dao: &EntityTypeDao) -> Self {
        Self {
            ty: EntityTypeType::new_from_type(&dao.namespace, &dao.type_name),
            description: dao.description.clone(),
            components: dao.components.iter().cloned().filter_map(|c| ComponentType::try_from(&c).ok()).collect(),
            properties: dao.properties.clone(),
            extensions: dao.extensions.clone(),
        }
    }
}

impl From<&EntityType> for EntityTypeDao {
    fn from(entity_type: &EntityType) -> Self {
        EntityTypeDao {
            namespace: entity_type.namespace(),
            type_name: entity_type.type_name(),
            description: entity_type.description.clone(),
            components: entity_type.components.iter().cloned().map(|c| c.type_definition().to_string()).collect(),
            properties: entity_type.properties.clone(),
            extensions: entity_type.extensions.clone(),
        }
    }
}
