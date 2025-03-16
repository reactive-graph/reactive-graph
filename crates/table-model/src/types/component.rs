use table_to_html::HtmlTable;
use tabled::Table;
use tabled::Tabled;
use tabled::settings::Modify;
use tabled::settings::Style;
use tabled::settings::Width;
use tabled::settings::object::Columns;
use tabled::settings::object::Segment;

use crate::container::DefaultTableContainer;
use crate::container::TableInlineFormat;
use crate::container::TableInlineFormatSetter;
use crate::container::TableOptions;
use crate::styles::modern_inline::modern_inline;
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
pub struct Component {
    /// The namespace of the component.
    pub namespace: String,

    /// The name of the component.
    pub name: String,

    /// Textual description of the component.
    // #[tabled(skip)]
    pub description: String,

    /// The property types.
    #[tabled(display("display_property_types", self))]
    pub properties: Vec<PropertyType>,

    /// The extensions.
    #[tabled(display("display_extensions", self))]
    pub extensions: Vec<Extension>,

    #[tabled(skip)]
    inline_format: TableInlineFormat,
}

fn display_property_types(properties: &[PropertyType], component: &Component) -> String {
    match component.inline_format {
        TableInlineFormat::Table => display_property_types_inline_str(properties),
        TableInlineFormat::Html => display_property_types_html_inline(properties),
    }
}

fn display_extensions(extensions: &[Extension], component: &Component) -> String {
    match component.inline_format {
        TableInlineFormat::Table => display_extensions_inline_str(extensions),
        TableInlineFormat::Html => display_extensions_html_inline(extensions),
    }
}

impl TableInlineFormatSetter for Component {
    fn set_table_inline_format(&mut self, table_inline_format: TableInlineFormat) {
        self.inline_format = table_inline_format;
    }
}

impl From<reactive_graph_graph::Component> for Component {
    fn from(component: reactive_graph_graph::Component) -> Self {
        Component {
            namespace: component.namespace(),
            name: component.type_name(),
            description: component.description,
            properties: PropertyTypes::from(component.properties).0,
            extensions: Extensions::from(component.extensions).0,
            inline_format: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Tabled)]
pub struct ComponentTypeId {
    /// The namespace of the component.
    pub namespace: String,

    /// The name of the component.
    pub name: String,

    #[tabled(skip)]
    inline_format: TableInlineFormat,
}

impl TableInlineFormatSetter for ComponentTypeId {
    fn set_table_inline_format(&mut self, table_inline_format: TableInlineFormat) {
        self.inline_format = table_inline_format;
    }
}

impl From<reactive_graph_graph::ComponentTypeId> for ComponentTypeId {
    fn from(ty: reactive_graph_graph::ComponentTypeId) -> Self {
        ComponentTypeId {
            namespace: ty.namespace(),
            name: ty.type_name(),
            inline_format: Default::default(),
        }
    }
}

pub fn display_component_type_ids_inline_str(tys: &[ComponentTypeId]) -> String {
    if tys.is_empty() {
        String::new()
    } else {
        display_component_type_ids_inline(tys).to_string()
    }
}

pub fn display_component_type_ids_inline(tys: &[ComponentTypeId]) -> Table {
    let tys = tys.to_vec();
    Table::new(tys)
        .with(modern_inline())
        .with(Modify::new(Columns::new(0..1)).with(Width::increase(15)))
        .with(Modify::new(Columns::new(1..2)).with(Width::increase(15)))
        .to_owned()
}

pub fn display_component_type_ids_html_inline(tys: &[ComponentTypeId]) -> String {
    let tys = tys.to_vec();
    if tys.is_empty() {
        return String::new();
    }
    HtmlTable::with_header(Vec::<Vec<String>>::from(Table::builder(&tys)))
        .to_string()
        .split_whitespace()
        .collect()
}

#[derive(Clone, Debug, Default)]
pub struct ComponentTypeIds(pub Vec<ComponentTypeId>);

impl From<reactive_graph_graph::ComponentTypeIds> for ComponentTypeIds {
    fn from(tys: reactive_graph_graph::ComponentTypeIds) -> Self {
        ComponentTypeIds(tys.into_iter().map(|ty| ty.into()).collect())
    }
}

pub type ComponentTableContainer = DefaultTableContainer<reactive_graph_graph::Component, Component, ComponentTypeIdsTableOptions>;

pub struct ComponentsTableOptions;

impl TableOptions for ComponentsTableOptions {
    fn options(table: &mut Table) -> &mut Table {
        table.with(Style::extended()).with(
            Modify::new(Segment::new(0.., 0..3))
                .with(Width::increase(22))
                .with(Width::increase(22))
                .with(Width::wrap(40).keep_words(true)),
        )
    }
}

pub type ComponentTypeIdTableContainer = DefaultTableContainer<reactive_graph_graph::ComponentTypeId, ComponentTypeId, ComponentTypeIdsTableOptions>;

pub struct ComponentTypeIdsTableOptions;

impl TableOptions for ComponentTypeIdsTableOptions {
    fn options(table: &mut Table) -> &mut Table {
        table.with(Style::extended())
    }
}
