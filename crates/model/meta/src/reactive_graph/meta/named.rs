//! ---------------------------------------------
//! This file was generated automatically.
//! ---------------------------------------------
#![allow(dead_code, unused)]
#![cfg_attr(rustfmt, rustfmt_skip)]

/// Namespace `reactive_graph::meta::Named`
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const NAMED_NAMESPACE: &str = "reactive_graph::meta::Named";

/// The [type identifier]() of Component `Named`.
///
/// ### Namespace
///
/// The fully qualified namespace is
/// `reactive_graph::meta::Named`
pub static NAMED: std::sync::LazyLock<reactive_graph_graph::ComponentTypeId> = std::sync::LazyLock::new(||
std::str::FromStr::from_str(NAMED_NAMESPACE).unwrap());

/// The properties of Component `Named`.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
pub enum NamedProperties {
    
    /// ### Property `name`
    ///
    /// The name of an entity or a relation
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    NAME,
}
impl NamedProperties {
    pub fn len() -> usize {
        1usize
    }
    
    pub fn property_types() -> reactive_graph_graph::PropertyTypes {
        let property_types = reactive_graph_graph::PropertyTypes::new();
        property_types.push(NamedProperties::NAME);
        property_types
    }
}

impl AsRef<str> for NamedProperties {
    #[inline]
    fn as_ref(&self) -> &str {
        match *self {
            NamedProperties::NAME => "name",
        }
    }
}

impl From<NamedProperties> for &'static str {
    #[inline]
    fn from(properties: NamedProperties) -> &'static str {
        match properties {
            NamedProperties::NAME => "name",
        }
    }
}

impl From<NamedProperties> for String {
    #[inline]
    fn from(properties: NamedProperties) -> String {
        match properties {
            NamedProperties::NAME => "name".to_owned(),
        }
    }
}

impl From<NamedProperties> for reactive_graph_graph::PropertyType {
    #[inline]
    fn from(properties: NamedProperties) -> reactive_graph_graph::PropertyType {
        match properties {
            NamedProperties::NAME => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "name",
                    "The name of an entity or a relation",
                    reactive_graph_graph::DataType::String,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
        }
    }
}

pub struct NamedPropertiesIterator(Option<NamedProperties>);

impl NamedProperties {
    pub fn into_iter() -> NamedPropertiesIterator {
        NamedPropertiesIterator(None)
    }
}

impl Iterator for NamedPropertiesIterator {
    type Item = NamedProperties;
    fn next(&mut self) -> Option<Self::Item> {
        self.0 = match self.0 {
            None => Some(NamedProperties::NAME),
            Some(NamedProperties::NAME) => None,
        };
        self.0.clone()
    }
}

impl IntoIterator for NamedProperties {
    type Item = NamedProperties;
    type IntoIter = NamedPropertiesIterator;
    fn into_iter(self) -> Self::IntoIter {
        NamedPropertiesIterator(None)
    }
}

impl core::fmt::Display for NamedProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match *self {
            NamedProperties::NAME => core::fmt::Display::fmt("name", f),
        }
    }
}

///
pub static NAMED_EXTENSIONS: std::sync::LazyLock<reactive_graph_graph::Extensions> = std::sync::LazyLock::new(||
{ reactive_graph_graph::Extensions::new() });

pub static NAMED_TYPE: std::sync::LazyLock<reactive_graph_graph::Component> = std::sync::LazyLock::new(||
{
    reactive_graph_graph::Component::builder()
        .ty(core::ops::Deref::deref(&NAMED))
        .description("The entity or relation has a name.")
        .properties(NamedProperties::property_types())
        .build()
});

/// # Component `Named`
///
/// ## Fully Qualified Namespace
///
/// `reactive_graph::meta::Named`
///
/// ## Description
///
/// The entity or relation has a name.
///
/// ### Properties
///
/// - name
///
/// ## JSON Schema
///
/// [https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/meta/Named.schema.json]()
///
pub trait Named {
    
    /// ### Property `name`
    ///
    /// The name of an entity or a relation
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn name(&self) -> String;
    
    /// ### Property `name`
    ///
    /// The name of an entity or a relation
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn set_name(&mut self, name: String);
}
