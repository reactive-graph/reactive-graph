use tabled::Tabled;

use crate::model::NamespacedTypeGetter;
use crate::table_model::types::property_type::display_property_types_inline;
use crate::table_model::types::property_type::PropertyType;
use crate::table_model::types::property_type::PropertyTypes;

#[derive(Tabled)]
pub(crate) struct Component {
    /// The namespace of the extension.
    pub namespace: String,

    /// The name of the extension.
    pub name: String,

    /// Textual description of the extension.
    // #[tabled(skip)]
    pub description: String,

    /// The property types.
    #[tabled(display_with("display_property_types_inline"))]
    pub properties: Vec<PropertyType>,
}

impl From<crate::model::Component> for Component {
    fn from(component: crate::model::Component) -> Self {
        Component {
            namespace: component.namespace(),
            name: component.type_name(),
            description: component.description,
            properties: PropertyTypes::from(component.properties).0,
        }
    }
}

pub(crate) struct Components(pub(crate) Vec<Component>);

impl From<Vec<crate::model::Component>> for Components {
    fn from(components: Vec<crate::model::Component>) -> Self {
        let c = components.into_iter().map(From::from).collect();
        Components(c)
    }
}
