use tabled::settings::object::Segment;
use tabled::settings::Modify;
use tabled::settings::Style;
use tabled::settings::Width;
use tabled::Table;
use tabled::Tabled;

use crate::table_model::container::DefaultTableContainer;
use crate::table_model::container::TableOptions;
use crate::table_model::types::extension::display_extensions_inline;
use crate::table_model::types::extension::Extension;
use crate::table_model::types::extension::Extensions;
use crate::table_model::types::property_type::display_property_types_inline;
use crate::table_model::types::property_type::PropertyType;
use crate::table_model::types::property_type::PropertyTypes;
use reactive_graph_graph::NamespacedTypeGetter;

#[derive(Clone, Debug, Tabled)]
pub(crate) struct EntityType {
    /// The namespace of the extension.
    pub namespace: String,

    /// The name of the extension.
    pub name: String,

    /// Textual description of the extension.
    // #[tabled(skip)]
    pub description: String,

    // /// The components.
    // #[tabled(display_with("display_components_inline"))]
    // pub components: Vec<ComponentTypeId>,
    /// The property types.
    #[tabled(display_with("display_property_types_inline"))]
    pub properties: Vec<PropertyType>,

    /// The extensions.
    #[tabled(display_with("display_extensions_inline"))]
    pub extensions: Vec<Extension>,
}

impl From<reactive_graph_graph::EntityType> for EntityType {
    fn from(entity_type: reactive_graph_graph::EntityType) -> Self {
        EntityType {
            namespace: entity_type.namespace(),
            name: entity_type.type_name(),
            description: entity_type.description,
            // components
            properties: PropertyTypes::from(entity_type.properties).0,
            extensions: Extensions::from(entity_type.extensions).0,
        }
    }
}

pub(crate) type EntityTypes = DefaultTableContainer<reactive_graph_graph::EntityType, EntityType, EntityTypesTableOptions>;

pub(crate) struct EntityTypesTableOptions;

impl TableOptions for EntityTypesTableOptions {
    fn options(table: &mut Table) -> &mut Table {
        table.with(Style::extended()).with(
            Modify::new(Segment::new(0.., 0..3))
                .with(Width::increase(22).priority())
                .with(Width::increase(22).priority())
                .with(Width::wrap(40).keep_words()),
        )
    }
}
