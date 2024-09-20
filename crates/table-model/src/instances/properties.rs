use crate::container::TableInlineFormat;
use crate::container::TableInlineFormatSetter;
use crate::container::TableOptions;
use crate::styles::modern_inline::modern_inline;
use serde::Serialize;
use serde_json::Value;
use std::fmt;
use std::fmt::Formatter;
use table_to_html::HtmlTable;
use tabled::settings::Style;
use tabled::Table;
use tabled::Tabled;

#[derive(Clone, Debug, Serialize, Tabled)]
pub struct PropertyInstance {
    /// The name of the property.
    pub name: String,

    /// The value of the property
    pub value: Value,

    #[tabled(skip)]
    #[serde(skip)]
    inline_format: TableInlineFormat,
}

impl PropertyInstance {
    pub fn new(name: String, value: Value) -> Self {
        Self {
            name,
            value,
            inline_format: Default::default(),
        }
    }
}

impl TableInlineFormatSetter for PropertyInstance {
    fn set_table_inline_format(&mut self, table_inline_format: TableInlineFormat) {
        self.inline_format = table_inline_format;
    }
}

pub fn display_property_instances_inline(property_instances: &[PropertyInstance]) -> Table {
    let property_instances = property_instances.to_vec();
    if property_instances.is_empty() {
        return Table::new(["No properties"]).with(modern_inline()).to_owned();
    }

    Table::new(property_instances).with(modern_inline()).to_owned()
}

pub fn display_property_instances_html_inline(property_instances: &Vec<PropertyInstance>) -> String {
    let property_instances = property_instances.to_vec();
    if property_instances.is_empty() {
        return String::new();
    }
    HtmlTable::with_header(Vec::<Vec<String>>::from(Table::builder(&property_instances)))
        .to_string()
        .split_whitespace()
        .collect()
}

#[derive(Clone, Debug)]
pub struct PropertyInstances(pub Vec<PropertyInstance>);

impl From<PropertyInstances> for reactive_graph_graph::PropertyInstances {
    fn from(property_instances: PropertyInstances) -> Self {
        property_instances
            .0
            .into_iter()
            .map(|property_instance| (property_instance.name, property_instance.value))
            .collect()
    }
}

impl From<reactive_graph_graph::PropertyInstances> for PropertyInstances {
    fn from(property_instances: reactive_graph_graph::PropertyInstances) -> Self {
        PropertyInstances(
            property_instances
                .into_iter()
                .map(|(name, value)| PropertyInstance {
                    name,
                    value,
                    inline_format: Default::default(),
                })
                .collect(),
        )
    }
}

impl fmt::Display for PropertyInstances {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f)
    }
}

pub struct PropertyInstancesTableOptions;

impl TableOptions for PropertyInstancesTableOptions {
    fn options(table: &mut Table) -> &mut Table {
        table.with(Style::extended())
    }
}
