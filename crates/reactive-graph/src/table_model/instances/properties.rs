use crate::table_model::container::TableOptions;
use crate::table_model::styles::modern_inline::modern_inline;
use serde::Serialize;
use serde_json::Value;
use std::fmt;
use std::fmt::Formatter;
use tabled::settings::Style;
use tabled::Table;
use tabled::Tabled;

#[derive(Clone, Debug, Serialize, Tabled)]
pub struct PropertyInstance {
    /// The name of the property.
    pub name: String,

    /// The value of the property
    pub value: Value,
}

impl PropertyInstance {
    pub fn new(name: String, value: Value) -> Self {
        Self { name, value }
    }
}

pub fn display_property_instances_inline(property_instances: &Vec<PropertyInstance>) -> String {
    if property_instances.is_empty() {
        return String::from("No properties");
    }

    Table::new(property_instances).with(modern_inline()).to_string()
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
        PropertyInstances(property_instances.into_iter().map(|(name, value)| PropertyInstance { name, value }).collect())
    }
}

impl fmt::Display for PropertyInstances {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f)
    }
}

pub(crate) struct PropertyInstancesTableOptions;

impl TableOptions for PropertyInstancesTableOptions {
    fn options(table: &mut Table) -> &mut Table {
        table.with(Style::extended())
    }
}
