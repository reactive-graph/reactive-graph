use tabled::Table;
use tabled::Tabled;
use tabled::settings::Modify;
use tabled::settings::Style;
use tabled::settings::Width;
use tabled::settings::object::Segment;

use crate::container::TableInlineFormat;
use crate::container::TableInlineFormatSetter;
use crate::container::TableOptions;
use crate::types::component::ComponentTypeId;
use crate::types::component::ComponentTypeIds;
use crate::types::component::display_component_type_ids_html_inline;
use crate::types::component::display_component_type_ids_inline_str;
use crate::types::extension::Extension;
use crate::types::extension::Extensions;
use crate::types::extension::display_extensions_html_inline;
use crate::types::extension::display_extensions_inline_str;
use crate::types::properties::PropertyType;
use crate::types::properties::PropertyTypes;
use crate::types::properties::display_property_types_html_inline;
use crate::types::properties::display_property_types_inline_str;
use reactive_graph_graph::NamespacedTypeGetter;

#[derive(Clone, Debug, Tabled)]
pub struct EntityType {
    /// The namespace of the entity type.
    pub namespace: String,

    /// The name of the entity type.
    pub name: String,

    /// Textual description of the entity type.
    pub description: String,

    /// The components.
    #[tabled(display("display_component_type_ids", self))]
    pub components: Vec<ComponentTypeId>,

    /// The property types.
    #[tabled(display("display_property_types", self))]
    pub properties: Vec<PropertyType>,

    /// The extensions.
    #[tabled(display("display_extensions", self))]
    pub extensions: Vec<Extension>,

    #[tabled(skip)]
    inline_format: TableInlineFormat,
}

fn display_component_type_ids(components: &[ComponentTypeId], entity_type: &EntityType) -> String {
    match entity_type.inline_format {
        TableInlineFormat::Table => display_component_type_ids_inline_str(components),
        TableInlineFormat::Html => display_component_type_ids_html_inline(components),
    }
}

fn display_property_types(properties: &[PropertyType], entity_type: &EntityType) -> String {
    match entity_type.inline_format {
        TableInlineFormat::Table => display_property_types_inline_str(properties),
        TableInlineFormat::Html => display_property_types_html_inline(properties),
    }
}

fn display_extensions(extensions: &[Extension], entity_type: &EntityType) -> String {
    // println!("{:?}", &self.inline_format);
    match entity_type.inline_format {
        TableInlineFormat::Table => display_extensions_inline_str(extensions),
        TableInlineFormat::Html => display_extensions_html_inline(extensions),
    }
}

impl TableInlineFormatSetter for EntityType {
    fn set_table_inline_format(&mut self, table_inline_format: TableInlineFormat) {
        self.inline_format = table_inline_format;
    }
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
            inline_format: Default::default(),
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
