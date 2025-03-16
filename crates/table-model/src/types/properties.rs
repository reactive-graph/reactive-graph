use crate::container::TableInlineFormat;
use crate::container::TableInlineFormatSetter;
use crate::container::TableOptions;
use crate::styles::modern_inline::modern_inline;
use crate::types::data_type::DataType;
use crate::types::extension::Extension;
use crate::types::extension::Extensions;
use crate::types::extension::display_extensions_html_inline;
use crate::types::extension::display_extensions_inline_str;
use crate::types::mutability::Mutability;
use crate::types::socket_type::SocketType;
use std::fmt;
use std::fmt::Formatter;
use table_to_html::HtmlTable;
use tabled::Table;
use tabled::Tabled;
use tabled::settings::Modify;
use tabled::settings::Style;
use tabled::settings::Width;
use tabled::settings::object::Columns;

#[derive(Clone, Debug, Tabled)]
pub struct PropertyType {
    /// The name of the property.
    pub name: String,

    /// Textual description of the property.
    #[tabled(skip)]
    pub description: String,

    /// Specifies the data type of the property.
    pub data_type: DataType,

    /// Specifies the type of socket - either input socket or output socket or none.
    pub socket_type: SocketType,

    /// Specifies if the property is mutable.
    pub mutability: Mutability,

    /// Property specific extensions.
    #[tabled(display("display_extensions", self))]
    #[tabled(skip)]
    pub extensions: Vec<Extension>,

    #[tabled(skip)]
    inline_format: TableInlineFormat,
}

#[allow(dead_code)]
fn display_extensions(extensions: &[Extension], property_type: &PropertyType) -> String {
    match property_type.inline_format {
        TableInlineFormat::Table => display_extensions_inline_str(extensions),
        TableInlineFormat::Html => display_extensions_html_inline(extensions),
    }
}

impl TableInlineFormatSetter for PropertyType {
    fn set_table_inline_format(&mut self, table_inline_format: TableInlineFormat) {
        self.inline_format = table_inline_format;
    }
}

impl From<PropertyType> for reactive_graph_graph::PropertyType {
    fn from(property_type: PropertyType) -> Self {
        reactive_graph_graph::PropertyType {
            name: property_type.name,
            description: property_type.description,
            data_type: property_type.data_type.into(),
            socket_type: property_type.socket_type.into(),
            mutability: property_type.mutability.into(),
            extensions: Extensions(property_type.extensions).into(),
        }
    }
}

impl From<reactive_graph_graph::PropertyType> for PropertyType {
    fn from(property_type: reactive_graph_graph::PropertyType) -> Self {
        PropertyType {
            name: property_type.name,
            description: property_type.description,
            data_type: property_type.data_type.into(),
            socket_type: property_type.socket_type.into(),
            mutability: property_type.mutability.into(),
            extensions: Extensions::from(property_type.extensions).into(),
            inline_format: Default::default(),
        }
    }
}

pub fn display_property_types_inline_str(property_types: &[PropertyType]) -> String {
    if property_types.is_empty() {
        String::new()
    } else {
        display_property_types_inline(property_types).to_string()
    }
}

pub fn display_property_types_inline(property_types: &[PropertyType]) -> Table {
    let property_types = property_types.to_vec();
    Table::new(property_types)
        .with(modern_inline())
        .with(Modify::new(Columns::new(0..1)).with(Width::increase(35)))
        .with(Modify::new(Columns::new(1..2)).with(Width::increase(9)))
        .with(Modify::new(Columns::new(2..3)).with(Width::increase(11)))
        .with(Modify::new(Columns::new(3..4)).with(Width::increase(10)))
        .to_owned()
}

pub fn display_property_types_html_inline(property_types: &[PropertyType]) -> String {
    let property_types = property_types.to_vec();
    if property_types.is_empty() {
        return String::new();
    }
    HtmlTable::with_header(Vec::<Vec<String>>::from(Table::builder(&property_types)))
        .to_string()
        .split_whitespace()
        .collect()
}

#[derive(Clone, Debug)]
pub struct PropertyTypes(pub Vec<PropertyType>);

impl From<PropertyTypes> for reactive_graph_graph::PropertyTypes {
    fn from(property_types: PropertyTypes) -> Self {
        property_types.0.into_iter().map(|property_type| property_type.into()).collect()
    }
}

impl From<reactive_graph_graph::PropertyTypes> for PropertyTypes {
    fn from(property_types: reactive_graph_graph::PropertyTypes) -> Self {
        PropertyTypes(property_types.into_iter().map(|(_property_name, property_type)| property_type.into()).collect())
    }
}

impl fmt::Display for PropertyTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f)
    }
}

pub struct PropertyTypesTableOptions;

impl TableOptions for PropertyTypesTableOptions {
    fn options(table: &mut Table) -> &mut Table {
        table.with(Style::extended())
    }
}
