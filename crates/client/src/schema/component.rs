use tabled::Tabled;

use crate::model::NamespacedTypeGetter;
use crate::schema::extension::Extension;
use crate::schema::extension::Extensions;
use crate::schema::property_type::display_property_types_inline;
use crate::schema::property_type::PropertyType;
use crate::schema::property_type::PropertyTypes;

#[derive(cynic::InputObject, Clone, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "crate::schema::schema")]
#[derive(Tabled)]
pub struct ComponentTypeId {
    pub name: String,
    pub namespace: String,
}

impl From<crate::model::ComponentTypeId> for ComponentTypeId {
    fn from(ty: crate::model::ComponentTypeId) -> Self {
        ComponentTypeId {
            name: ty.type_name(),
            namespace: ty.namespace(),
        }
    }
}

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "crate::schema::schema")]
#[derive(Tabled)]
pub struct Component {
    /// The namespace of the extension.
    pub namespace: String,

    /// The name of the extension.
    pub name: String,

    /// Textual description of the extension.
    #[tabled(skip)]
    pub description: String,

    /// The property types.
    #[tabled(display_with("display_property_types_inline"))]
    pub properties: Vec<PropertyType>,

    /// The extensions.
    #[tabled(skip)]
    pub extensions: Vec<Extension>,
}

impl From<Component> for crate::model::Component {
    fn from(component: Component) -> Self {
        let ty = crate::model::ComponentTypeId::new_from_type(component.namespace, component.name);
        crate::model::Component {
            ty,
            description: component.description,
            properties: PropertyTypes(component.properties).into(),
            extensions: Extensions(component.extensions).into(),
        }
    }
}
