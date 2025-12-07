//! ---------------------------------------------
//! This file was generated automatically.
//! ---------------------------------------------
#![allow(dead_code, unused)]
#![cfg_attr(rustfmt, rustfmt_skip)]

/// Namespace `reactive_graph::meta::Describable`
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const DESCRIBABLE_NAMESPACE: &str = "reactive_graph::meta::Describable";

/// The [type identifier]() of Component `Describable`.
///
/// ### Namespace
///
/// The fully qualified namespace is
/// `reactive_graph::meta::Describable`
pub static DESCRIBABLE: std::sync::LazyLock<reactive_graph_graph::ComponentTypeId> = std::sync::LazyLock::new(||
std::str::FromStr::from_str(DESCRIBABLE_NAMESPACE).unwrap());

/// The properties of Component `Describable`.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
pub enum DescribableProperties {
    
    /// ### Property `description`
    ///
    /// The description of an entity or a relation
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    DESCRIPTION,
}
impl DescribableProperties {
    pub fn len() -> usize {
        1usize
    }
    
    pub fn property_types() -> reactive_graph_graph::PropertyTypes {
        let property_types = reactive_graph_graph::PropertyTypes::new();
        property_types.push(DescribableProperties::DESCRIPTION);
        property_types
    }
}

impl AsRef<str> for DescribableProperties {
    #[inline]
    fn as_ref(&self) -> &str {
        match *self {
            DescribableProperties::DESCRIPTION => "description",
        }
    }
}

impl From<DescribableProperties> for &'static str {
    #[inline]
    fn from(properties: DescribableProperties) -> &'static str {
        match properties {
            DescribableProperties::DESCRIPTION => "description",
        }
    }
}

impl From<DescribableProperties> for String {
    #[inline]
    fn from(properties: DescribableProperties) -> String {
        match properties {
            DescribableProperties::DESCRIPTION => "description".to_owned(),
        }
    }
}

impl From<DescribableProperties> for reactive_graph_graph::PropertyType {
    #[inline]
    fn from(properties: DescribableProperties) -> reactive_graph_graph::PropertyType {
        match properties {
            DescribableProperties::DESCRIPTION => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "description",
                    "The description of an entity or a relation",
                    reactive_graph_graph::DataType::String,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
        }
    }
}

pub struct DescribablePropertiesIterator(Option<DescribableProperties>);

impl DescribableProperties {
    pub fn into_iter() -> DescribablePropertiesIterator {
        DescribablePropertiesIterator(None)
    }
}

impl Iterator for DescribablePropertiesIterator {
    type Item = DescribableProperties;
    fn next(&mut self) -> Option<Self::Item> {
        self.0 = match self.0 {
            None => Some(DescribableProperties::DESCRIPTION),
            Some(DescribableProperties::DESCRIPTION) => None,
        };
        self.0.clone()
    }
}

impl IntoIterator for DescribableProperties {
    type Item = DescribableProperties;
    type IntoIter = DescribablePropertiesIterator;
    fn into_iter(self) -> Self::IntoIter {
        DescribablePropertiesIterator(None)
    }
}

impl core::fmt::Display for DescribableProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match *self {
            DescribableProperties::DESCRIPTION => {
                core::fmt::Display::fmt("description", f)
            }
        }
    }
}

///
pub static DESCRIBABLE_EXTENSIONS: std::sync::LazyLock<
    reactive_graph_graph::Extensions,
> = std::sync::LazyLock::new(|| { reactive_graph_graph::Extensions::new() });

pub static DESCRIBABLE_TYPE: std::sync::LazyLock<reactive_graph_graph::Component> = std::sync::LazyLock::new(||
{
    reactive_graph_graph::Component::builder()
        .ty(core::ops::Deref::deref(&DESCRIBABLE))
        .description("The entity or relation has a description.")
        .properties(DescribableProperties::property_types())
        .build()
});

/// # Component `Describable`
///
/// ## Fully Qualified Namespace
///
/// `reactive_graph::meta::Describable`
///
/// ## Description
///
/// The entity or relation has a description.
///
/// ### Properties
///
/// - description
///
/// ## JSON Schema
///
/// [https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/meta/Describable.schema.json]()
///
pub trait Describable {
    
    /// ### Property `description`
    ///
    /// The description of an entity or a relation
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn description(&self) -> String;
    
    /// ### Property `description`
    ///
    /// The description of an entity or a relation
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn set_description(&mut self, description: String);
}
