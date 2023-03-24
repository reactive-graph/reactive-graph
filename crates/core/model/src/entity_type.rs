use serde::Deserialize;
use serde::Serialize;

use crate::extension::Extension;
use crate::ComponentTypeId;
use crate::EntityTypeId;
use crate::ExtensionContainer;
use crate::ExtensionTypeId;
use crate::NamespacedTypeGetter;
use crate::PropertyType;
use crate::PropertyTypeContainer;
use crate::TypeContainer;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;

/// Entity types defines the type of an entity instance.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EntityType {
    /// The type definition contains the namespace and the type name.
    #[serde(flatten)]
    pub ty: EntityTypeId,

    /// Textual description of the entity type.
    #[serde(default = "String::new")]
    pub description: String,

    /// The names of the components of the entity type.
    #[serde(default = "Vec::new")]
    pub components: Vec<ComponentTypeId>,

    /// The properties which are defined by the entity type.
    #[serde(default = "Vec::new")]
    pub properties: Vec<PropertyType>,

    /// Entity type specific extensions.
    #[serde(default = "Vec::new")]
    pub extensions: Vec<Extension>,
}

impl EntityType {
    /// Constructs an entity type from the given namespaced type with the given description, components, properties and extensions.
    pub fn new<T: Into<EntityTypeId>, S: Into<String>>(
        ty: T,
        description: S,
        components: Vec<ComponentTypeId>,
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
        components: Vec<ComponentTypeId>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> EntityType {
        EntityType {
            ty: EntityTypeId::new_from_type(namespace, type_name),
            description: description.into(),
            components,
            properties,
            extensions,
        }
    }
}

impl TypeContainer for EntityType {
    fn is_a(&self, ty: &ComponentTypeId) -> bool {
        self.components.contains(ty)
    }
}

impl PropertyTypeContainer for EntityType {
    fn has_own_property<S: Into<String>>(&self, property_name: S) -> bool {
        let property_name = property_name.into();
        self.properties.iter().any(|p| p.name == property_name)
    }

    fn get_own_property<S: Into<String>>(&self, property_name: S) -> Option<PropertyType> {
        let property_name = property_name.into();
        self.properties.iter().find(|p| p.name == property_name).cloned()
    }

    fn merge_properties(&mut self, properties_to_merge: Vec<PropertyType>) {
        for property_to_merge in properties_to_merge.into_iter() {
            if !self.has_own_property(&property_to_merge.name) {
                self.properties.push(property_to_merge);
            } else if let Some(existing_property) = self.properties.iter_mut().find(|p| p.name == property_to_merge.name) {
                existing_property.description = property_to_merge.description.clone();
                existing_property.data_type = property_to_merge.data_type;
                existing_property.socket_type = property_to_merge.socket_type;
                existing_property.mutability = property_to_merge.mutability;
                existing_property.merge_extensions(property_to_merge.extensions);
            }
        }
    }
}

impl ExtensionContainer for EntityType {
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
            type_id_type: TypeIdType::EntityType,
            namespace: entity_type.namespace(),
            type_name: entity_type.type_name(),
        }
    }
}
