use std::fmt;
use std::fmt::Formatter;
use tabled::settings::object::Rows;
use tabled::settings::Modify;
use tabled::settings::Width;
use tabled::Table;
use tabled::Tabled;

use crate::schema::data_type::DataType;
use crate::schema::extension::Extension;
use crate::schema::extension::ExtensionDefinition;
use crate::schema::extension::ExtensionDefinitions;
use crate::schema::extension::Extensions;
use crate::schema::mutability::Mutability;
use crate::schema::socket_type::SocketType;
use crate::table::modern_inline;

#[derive(cynic::InputObject, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "crate::schema::schema")]
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

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "crate::schema::schema")]
#[derive(Tabled)]
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
    // #[tabled(display_with("display_extensions"))]
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

impl fmt::Display for PropertyTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // let x = Table::new(&self.0).to_string();
        // writeln!(f, "{}", Table::new(self.0.clone()).to_string())
        writeln!(f, "")
    }
}
