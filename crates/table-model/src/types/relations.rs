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
pub struct RelationType {
    /// The namespace of the outbound entity type.
    pub outbound_type_namespace: String,

    /// The name of the outbound entity type.
    pub outbound_type_name: String,

    /// The namespace of the relation type.
    pub namespace: String,

    /// The name of the relation type.
    pub name: String,

    /// The namespace of the inbound entity type.
    pub inbound_type_namespace: String,

    /// The name of the inbound entity type.
    pub inbound_type_name: String,

    /// Textual description of the relation type.
    // #[tabled(skip)]
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

fn display_component_type_ids(components: &[ComponentTypeId], relation_type: &RelationType) -> String {
    match relation_type.inline_format {
        TableInlineFormat::Table => display_component_type_ids_inline_str(components),
        TableInlineFormat::Html => display_component_type_ids_html_inline(components),
    }
}

fn display_property_types(properties: &[PropertyType], relation_type: &RelationType) -> String {
    match relation_type.inline_format {
        TableInlineFormat::Table => display_property_types_inline_str(properties),
        TableInlineFormat::Html => display_property_types_html_inline(properties),
    }
}

fn display_extensions(extensions: &[Extension], relation_type: &RelationType) -> String {
    match relation_type.inline_format {
        TableInlineFormat::Table => display_extensions_inline_str(extensions),
        TableInlineFormat::Html => display_extensions_html_inline(extensions),
    }
}

impl TableInlineFormatSetter for RelationType {
    fn set_table_inline_format(&mut self, table_inline_format: TableInlineFormat) {
        self.inline_format = table_inline_format;
    }
}

impl From<reactive_graph_graph::RelationType> for RelationType {
    fn from(relation_type: reactive_graph_graph::RelationType) -> Self {
        RelationType {
            outbound_type_namespace: relation_type.outbound_type.namespace(),
            outbound_type_name: relation_type.outbound_type.type_name(),
            namespace: relation_type.namespace(),
            name: relation_type.type_name(),
            inbound_type_namespace: relation_type.inbound_type.namespace(),
            inbound_type_name: relation_type.inbound_type.type_name(),
            description: relation_type.description,
            components: ComponentTypeIds::from(relation_type.components).0,
            properties: PropertyTypes::from(relation_type.properties).0,
            extensions: Extensions::from(relation_type.extensions).0,
            inline_format: Default::default(),
        }
    }
}

pub struct RelationTypesTableOptions;

impl TableOptions for RelationTypesTableOptions {
    fn options(table: &mut Table) -> &mut Table {
        table.with(Style::extended()).with(
            Modify::new(Segment::new(0.., 0..3))
                .with(Width::increase(22))
                .with(Width::increase(22))
                .with(Width::wrap(40).keep_words(true)),
        )
    }
}
