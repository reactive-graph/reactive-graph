use serde::Deserialize;
use serde::Serialize;

use crate::ComponentTypeId;
use crate::Extension;
use crate::ExtensionContainer;
use crate::ExtensionTypeId;
use crate::NamespacedTypeGetter;
use crate::PropertyType;
use crate::PropertyTypeContainer;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;

/// A component defines a set of properties to be applied to entity
/// types and relation types.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Component {
    /// The type definition of the component.
    #[serde(flatten)]
    pub ty: ComponentTypeId,

    /// Textual description of the component.
    #[serde(default = "String::new")]
    pub description: String,

    /// The properties which are applied on entity or relation instances.
    #[serde(default = "Vec::new")]
    pub properties: Vec<PropertyType>,

    /// Component specific extensions
    #[serde(default = "Vec::new")]
    pub extensions: Vec<Extension>,
}

impl Component {
    pub fn new<T: Into<ComponentTypeId>, S: Into<String>>(ty: T, description: S, properties: Vec<PropertyType>, extensions: Vec<Extension>) -> Component {
        Component {
            ty: ty.into(),
            description: description.into(),
            properties,
            extensions,
        }
    }

    /// Constructs a new component with the given name and properties
    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S, description: S, properties: Vec<PropertyType>, extensions: Vec<Extension>) -> Component {
        Component {
            ty: ComponentTypeId::new_from_type(namespace, type_name),
            description: description.into(),
            properties,
            extensions,
        }
    }

    /// Constructs a new component with the given name and properties
    pub fn new_without_extensions<T: Into<ComponentTypeId>, S: Into<String>>(ty: T, description: S, properties: Vec<PropertyType>) -> Component {
        Component {
            ty: ty.into(),
            description: description.into(),
            properties,
            extensions: Vec::new(),
        }
    }

    /// Constructs an component with the given name but without properties
    pub fn new_without_properties<T: Into<ComponentTypeId>, S: Into<String>>(ty: T, description: S, extensions: Vec<Extension>) -> Component {
        Component {
            ty: ty.into(),
            description: description.into(),
            properties: Vec::new(),
            extensions,
        }
    }

    /// Returns true, if the component contains a property with the given name.
    pub fn has_property<S: Into<String>>(&self, property_name: S) -> bool {
        let property_name = property_name.into();
        self.properties.iter().any(|p| p.name == property_name)
    }

    /// Returns true, if the component contains an extension with the given type.
    pub fn has_extension(&self, ty: &ExtensionTypeId) -> bool {
        self.extensions.iter().any(|extension| &extension.ty == ty)
    }

    fn get_extension(&self, ty: &ExtensionTypeId) -> Option<Extension> {
        self.extensions.iter().find(|extension| &extension.ty == ty).cloned()
    }
}

impl PropertyTypeContainer for Component {
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

impl ExtensionContainer for Component {
    fn has_own_extension(&self, ty: &ExtensionTypeId) -> bool {
        self.has_extension(ty)
    }

    fn get_own_extension(&self, ty: &ExtensionTypeId) -> Option<Extension> {
        self.get_extension(ty)
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

impl NamespacedTypeGetter for Component {
    fn namespace(&self) -> String {
        self.ty.namespace()
    }

    fn type_name(&self) -> String {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for Component {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
    }
}

impl From<Component> for TypeDefinition {
    fn from(component: Component) -> Self {
        TypeDefinition {
            type_id_type: TypeIdType::Component,
            namespace: component.ty.namespace(),
            type_name: component.ty.type_name(),
        }
    }
}

impl From<&Component> for ComponentTypeId {
    fn from(component: &Component) -> Self {
        component.ty.clone()
    }
}

#[macro_export]
macro_rules! component_model {
    (
        $ident: ident
        $(,
            $accessor_type: tt
            $(
            $accessor_name: ident
            $accessor_data_type: tt
            )?
        )*
        $(,)?
    ) => {
        pub trait $ident: $crate::PropertyInstanceGetter + $crate::PropertyInstanceSetter {
            $(
                $crate::rx_accessor!($accessor_type $($accessor_name $accessor_data_type)?);
            )*
        }
    };
}
