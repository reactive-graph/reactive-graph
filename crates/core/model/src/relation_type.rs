use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::extension::Extension;
use crate::ComponentOrEntityTypeId;
use crate::ComponentTypeId;
use crate::ExtensionContainer;
use crate::ExtensionTypeId;
use crate::NamespacedTypeGetter;
use crate::PropertyType;
use crate::PropertyTypeContainer;
use crate::RelationTypeId;
use crate::TypeContainer;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;

/// A relation type defines the type of an relation instance.
///
/// The relation type defines the entity types of the outbound and inbound entity instances.
/// Also the relation type defines the properties of the relation instance.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
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
    #[serde(default = "Vec::<ComponentTypeId>::new")]
    pub components: Vec<ComponentTypeId>,

    /// The properties which are defined by the relation type.
    #[serde(default = "Vec::<PropertyType>::new")]
    pub properties: Vec<PropertyType>,

    /// Relation type specific extensions.
    #[serde(default = "Vec::<Extension>::new")]
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
}

impl PropertyTypeContainer for RelationType {
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

impl ExtensionContainer for RelationType {
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
