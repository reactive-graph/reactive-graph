use tabled::settings::object::Segment;
use tabled::settings::Modify;
use tabled::settings::Style;
use tabled::settings::Width;
use tabled::Table;
use tabled::Tabled;

use crate::model::NamespacedTypeGetter;
use crate::table_model::container::DefaultTableContainer;
use crate::table_model::container::TableOptions;
use crate::table_model::types::extension::display_extensions_inline;
use crate::table_model::types::extension::Extension;
use crate::table_model::types::extension::Extensions;
use crate::table_model::types::property_type::display_property_types_inline;
use crate::table_model::types::property_type::PropertyType;
use crate::table_model::types::property_type::PropertyTypes;

#[derive(Clone, Debug, Tabled)]
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

    /// The extensions.
    #[tabled(display_with("display_extensions_inline"))]
    pub extensions: Vec<Extension>,
}

impl From<crate::model::Component> for Component {
    fn from(component: crate::model::Component) -> Self {
        Component {
            namespace: component.namespace(),
            name: component.type_name(),
            description: component.description,
            properties: PropertyTypes::from(component.properties).0,
            extensions: Extensions::from(component.extensions).0,
        }
    }
}

pub(crate) type Components = DefaultTableContainer<crate::model::Component, Component, ComponentsTableOptions>;

pub(crate) struct ComponentsTableOptions;

impl TableOptions for ComponentsTableOptions {
    fn options(table: &mut Table) -> &mut Table {
        table.with(Style::extended()).with(
            Modify::new(Segment::new(0.., 0..3))
                .with(Width::increase(22).priority())
                .with(Width::increase(22).priority())
                .with(Width::wrap(40).keep_words()),
        )
    }
}
