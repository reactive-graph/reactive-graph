use std::fmt;
use std::fmt::Formatter;
use tabled::settings::object::Rows;
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

impl From<crate::model::PropertyType> for PropertyTypeDefinition {
    fn from(property_type: crate::model::PropertyType) -> Self {
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

impl From<Vec<crate::model::PropertyType>> for PropertyTypeDefinitions {
    fn from(property_types: Vec<crate::model::PropertyType>) -> Self {
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

impl From<PropertyType> for crate::model::PropertyType {
    fn from(property_type: PropertyType) -> Self {
        crate::model::PropertyType {
            name: property_type.name,
            description: property_type.description,
            data_type: property_type.data_type.into(),
            socket_type: property_type.socket_type.into(),
            mutability: property_type.mutability.into(),
            extensions: Extensions(property_type.extensions).into(),
        }
    }
}

impl From<crate::model::PropertyType> for PropertyType {
    fn from(property_type: crate::model::PropertyType) -> Self {
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
        // .with(Style::modern().remove_top().remove_left().remove_bottom().remove_right())
        .with(Modify::new(Rows::new(1..)).with(Width::truncate(15).suffix("...")).with(Width::increase(15)))
        .to_string()
}

#[derive(Clone, Debug)]
pub struct PropertyTypes(pub Vec<PropertyType>);

impl From<PropertyTypes> for Vec<crate::model::PropertyType> {
    fn from(property_types: PropertyTypes) -> Self {
        property_types.0.into_iter().map(|property_type| property_type.into()).collect()
    }
}

impl From<Vec<crate::model::PropertyType>> for PropertyTypes {
    fn from(property_types: Vec<crate::model::PropertyType>) -> Self {
        PropertyTypes(property_types.into_iter().map(|property_type| property_type.into()).collect())
    }
}

impl fmt::Display for PropertyTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // let x = Table::new(&self.0).to_string();
        // writeln!(f, "{}", Table::new(self.0.clone()).to_string())
        writeln!(f)
    }
}
