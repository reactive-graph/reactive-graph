//! ---------------------------------------------
//! This file was generated automatically.
//! ---------------------------------------------
#![allow(dead_code, unused)]
#![cfg_attr(rustfmt, rustfmt_skip)]

/// Namespace `reactive_graph::core::Labeled`
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const LABELED_NAMESPACE: &str = "reactive_graph::core::Labeled";

/// The [type identifier]() of Component `Labeled`.
///
/// ### Namespace
///
/// The fully qualified namespace is
/// `reactive_graph::core::Labeled`
pub static LABELED: std::sync::LazyLock<reactive_graph_graph::ComponentTypeId> = std::sync::LazyLock::new(||
std::str::FromStr::from_str(LABELED_NAMESPACE).unwrap());

/// The properties of Component `Labeled`.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
pub enum LabeledProperties {
    
    /// ### Property `label`
    ///
    /// Hierarchical path with static segments, named parameters and catch-all parameters
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    LABEL,
}
impl LabeledProperties {
    pub fn len() -> usize {
        1usize
    }
    
    pub fn property_types() -> reactive_graph_graph::PropertyTypes {
        let property_types = reactive_graph_graph::PropertyTypes::new();
        property_types.push(LabeledProperties::LABEL);
        property_types
    }
}

impl AsRef<str> for LabeledProperties {
    #[inline]
    fn as_ref(&self) -> &str {
        match *self {
            LabeledProperties::LABEL => "label",
        }
    }
}

impl From<LabeledProperties> for &'static str {
    #[inline]
    fn from(properties: LabeledProperties) -> &'static str {
        match properties {
            LabeledProperties::LABEL => "label",
        }
    }
}

impl From<LabeledProperties> for String {
    #[inline]
    fn from(properties: LabeledProperties) -> String {
        match properties {
            LabeledProperties::LABEL => "label".to_owned(),
        }
    }
}

impl From<LabeledProperties> for reactive_graph_graph::PropertyType {
    #[inline]
    fn from(properties: LabeledProperties) -> reactive_graph_graph::PropertyType {
        match properties {
            LabeledProperties::LABEL => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "label",
                    "Hierarchical path with static segments, named parameters and catch-all parameters",
                    reactive_graph_graph::DataType::String,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
        }
    }
}

pub struct LabeledPropertiesIterator(Option<LabeledProperties>);

impl LabeledProperties {
    pub fn into_iter() -> LabeledPropertiesIterator {
        LabeledPropertiesIterator(None)
    }
}

impl Iterator for LabeledPropertiesIterator {
    type Item = LabeledProperties;
    fn next(&mut self) -> Option<Self::Item> {
        self.0 = match self.0 {
            None => Some(LabeledProperties::LABEL),
            Some(LabeledProperties::LABEL) => None,
        };
        self.0.clone()
    }
}

impl IntoIterator for LabeledProperties {
    type Item = LabeledProperties;
    type IntoIter = LabeledPropertiesIterator;
    fn into_iter(self) -> Self::IntoIter {
        LabeledPropertiesIterator(None)
    }
}

impl core::fmt::Display for LabeledProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match *self {
            LabeledProperties::LABEL => core::fmt::Display::fmt("label", f),
        }
    }
}

///
pub static LABELED_EXTENSIONS: std::sync::LazyLock<reactive_graph_graph::Extensions> = std::sync::LazyLock::new(||
{ reactive_graph_graph::Extensions::new() });

pub static LABELED_TYPE: std::sync::LazyLock<reactive_graph_graph::Component> = std::sync::LazyLock::new(||
{
    reactive_graph_graph::Component::builder()
        .ty(core::ops::Deref::deref(&LABELED))
        .description(
            "The label is a hierarchical path with static segments, named parameters and catch-all parameters.",
        )
        .properties(LabeledProperties::property_types())
        .build()
});

/// # Component `Labeled`
///
/// ## Fully Qualified Namespace
///
/// `reactive_graph::core::Labeled`
///
/// ## Description
///
/// The label is a hierarchical path with static segments, named parameters and catch-all
/// parameters.
///
/// ### Properties
///
/// - label
///
/// ## JSON Schema
///
/// [https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/core/Labeled.schema.json]()
///
pub trait Labeled {
    
    /// ### Property `label`
    ///
    /// Hierarchical path with static segments, named parameters and catch-all parameters
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn label(&self) -> String;
    
    /// ### Property `label`
    ///
    /// Hierarchical path with static segments, named parameters and catch-all parameters
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn set_label(&mut self, label: String);
}
