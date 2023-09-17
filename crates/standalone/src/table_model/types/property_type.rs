use std::fmt;
use std::fmt::Formatter;
use tabled::settings::object::Columns;
use tabled::settings::Modify;
use tabled::settings::Width;
use tabled::Table;
use tabled::Tabled;

use crate::table_model::styles::modern_inline::modern_inline;
use crate::table_model::types::data_type::DataType;
use crate::table_model::types::extension::Extension;
use crate::table_model::types::extension::ExtensionDefinition;
use crate::table_model::types::extension::ExtensionDefinitions;
use crate::table_model::types::extension::Extensions;
use crate::table_model::types::mutability::Mutability;
use crate::table_model::types::socket_type::SocketType;

pub struct PropertyTypeDefinition {
    pub data_type: DataType,
    pub description: String,
    pub extensions: Vec<ExtensionDefinition>,
    pub mutability: Mutability,
    pub name: String,
    pub socket_type: SocketType,
}

impl From<inexor_rgf_graph::PropertyType> for PropertyTypeDefinition {
    fn from(property_type: inexor_rgf_graph::PropertyType) -> Self {
        let extensions: ExtensionDefinitions = property_type.extensions.into();
        PropertyTypeDefinition {
            name: property_type.name,
            description: property_type.description,
            data_type: property_type.data_type.into(),
            socket_type: property_type.socket_type.into(),
            mutability: property_type.mutability.into(),
            extensions: extensions.0,
        }
    }
}

pub struct PropertyTypeDefinitions(pub Vec<PropertyTypeDefinition>);

impl From<PropertyTypeDefinitions> for Vec<PropertyTypeDefinition> {
    fn from(property_types: PropertyTypeDefinitions) -> Self {
        property_types.0.into_iter().collect()
    }
}

impl From<Vec<inexor_rgf_graph::PropertyType>> for PropertyTypeDefinitions {
    fn from(property_types: Vec<inexor_rgf_graph::PropertyType>) -> Self {
        PropertyTypeDefinitions(property_types.into_iter().map(|property_type| property_type.into()).collect())
    }
}

#[derive(Clone, Debug, Tabled)]
pub struct PropertyType {
    /// The name of the extension.
    pub name: String,

    /// Textual description of the extension.
    #[tabled(skip)]
    pub description: String,

    /// The extension as JSON representation.
    pub data_type: DataType,

    /// Specifies the type of socket - either input socket or output socket or none
    pub socket_type: SocketType,

    /// Specifies if the property is mutable.
    pub mutability: Mutability,

    /// Property specific extensions
    #[tabled(display_with("display_extensions"))]
    #[tabled(skip)]
    pub extensions: Vec<Extension>,
}

impl From<PropertyType> for inexor_rgf_graph::PropertyType {
    fn from(property_type: PropertyType) -> Self {
        inexor_rgf_graph::PropertyType {
            name: property_type.name,
            description: property_type.description,
            data_type: property_type.data_type.into(),
            socket_type: property_type.socket_type.into(),
            mutability: property_type.mutability.into(),
            extensions: Extensions(property_type.extensions).into(),
        }
    }
}

impl From<inexor_rgf_graph::PropertyType> for PropertyType {
    fn from(property_type: inexor_rgf_graph::PropertyType) -> Self {
        PropertyType {
            name: property_type.name,
            description: property_type.description,
            data_type: property_type.data_type.into(),
            socket_type: property_type.socket_type.into(),
            mutability: property_type.mutability.into(),
            extensions: Extensions::from(property_type.extensions).into(),
        }
    }
}

pub fn display_property_types(property_types: &Vec<PropertyType>) -> String {
    Table::new(property_types).to_string()
}

pub fn display_property_types_inline(property_types: &Vec<PropertyType>) -> String {
    if property_types.is_empty() {
        return String::from("No properties");
    }

    Table::new(property_types)
        .with(modern_inline())
        .with(Modify::new(Columns::new(0..1)).with(Width::increase(35)))
        .with(Modify::new(Columns::new(1..2)).with(Width::increase(9)))
        .with(Modify::new(Columns::new(2..3)).with(Width::increase(11)))
        .with(Modify::new(Columns::new(3..4)).with(Width::increase(10)))
        .to_string()
}

#[derive(Clone, Debug)]
pub struct PropertyTypes(pub Vec<PropertyType>);

impl From<PropertyTypes> for inexor_rgf_graph::PropertyTypes {
    fn from(property_types: PropertyTypes) -> Self {
        property_types.0.into_iter().map(|property_type| property_type.into()).collect()
    }
}

impl From<inexor_rgf_graph::PropertyTypes> for PropertyTypes {
    fn from(property_types: inexor_rgf_graph::PropertyTypes) -> Self {
        PropertyTypes(property_types.into_iter().map(|(property_name, property_type)| property_type.into()).collect())
    }
}

impl fmt::Display for PropertyTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // let x = Table::new(&self.0).to_string();
        // writeln!(f, "{}", Table::new(self.0.clone()).to_string())
        writeln!(f)
    }
}
