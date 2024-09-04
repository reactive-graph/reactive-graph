use tabled::settings::object::Segment;
use tabled::settings::Modify;
use tabled::settings::Style;
use tabled::settings::Width;
use tabled::Table;
use tabled::Tabled;

use crate::container::TableOptions;
use crate::types::component::display_component_type_ids_inline;
use crate::types::component::ComponentTypeId;
use crate::types::component::ComponentTypeIds;
use crate::types::extension::display_extensions_inline;
use crate::types::extension::Extension;
use crate::types::extension::Extensions;
use crate::types::property_type::display_property_types_inline;
use crate::types::property_type::PropertyType;
use crate::types::property_type::PropertyTypes;
use reactive_graph_graph::NamespacedTypeGetter;

#[derive(Clone, Debug, Tabled)]
pub struct EntityType {
    /// The namespace of the extension.
    pub namespace: String,

    /// The name of the extension.
    pub name: String,

    /// Textual description of the extension.
    // #[tabled(skip)]
    pub description: String,

    /// The components.
    #[tabled(display_with("display_component_type_ids_inline"))]
    pub components: Vec<ComponentTypeId>,

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
            components: ComponentTypeIds::from(entity_type.components).0,
            properties: PropertyTypes::from(entity_type.properties).0,
            extensions: Extensions::from(entity_type.extensions).0,
        }
    }
}

pub struct EntityTypesTableOptions;

impl TableOptions for EntityTypesTableOptions {
    fn options(table: &mut Table) -> &mut Table {
        table.with(Style::extended()).with(
            Modify::new(Segment::new(0.., 0..3))
                .with(Width::increase(22))
                .with(Width::increase(22))
                .with(Width::wrap(40).keep_words(true)),
        )
    }
}
