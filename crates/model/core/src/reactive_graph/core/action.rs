//! ---------------------------------------------
//! This file was generated automatically.
//! ---------------------------------------------
#![allow(dead_code, unused)]
#![cfg_attr(rustfmt, rustfmt_skip)]

/// Namespace `reactive_graph::core::Action`
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const ACTION_NAMESPACE: &str = "reactive_graph::core::Action";

/// The [type identifier]() of Component `Action`.
///
/// ### Namespace
///
/// The fully qualified namespace is
/// `reactive_graph::core::Action`
pub static ACTION: std::sync::LazyLock<reactive_graph_graph::ComponentTypeId> = std::sync::LazyLock::new(||
std::str::FromStr::from_str(ACTION_NAMESPACE).unwrap());

/// The properties of Component `Action`.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
pub enum ActionProperties {
    
    /// ### Property `trigger`
    ///
    /// On receiving a boolean trigger the action will be executed
    ///
    /// Data Type: `Bool`
    ///
    /// Socket Type: `Input`
    ///
    /// Mutability: `Mutable`
    ///
    TRIGGER,
}
impl ActionProperties {
    pub fn len() -> usize {
        1usize
    }
    
    pub fn property_types() -> reactive_graph_graph::PropertyTypes {
        let property_types = reactive_graph_graph::PropertyTypes::new();
        property_types.push(ActionProperties::TRIGGER);
        property_types
    }
}

impl AsRef<str> for ActionProperties {
    #[inline]
    fn as_ref(&self) -> &str {
        match *self {
            ActionProperties::TRIGGER => "trigger",
        }
    }
}

impl From<ActionProperties> for &'static str {
    #[inline]
    fn from(properties: ActionProperties) -> &'static str {
        match properties {
            ActionProperties::TRIGGER => "trigger",
        }
    }
}

impl From<ActionProperties> for String {
    #[inline]
    fn from(properties: ActionProperties) -> String {
        match properties {
            ActionProperties::TRIGGER => "trigger".to_owned(),
        }
    }
}

impl From<ActionProperties> for reactive_graph_graph::PropertyType {
    #[inline]
    fn from(properties: ActionProperties) -> reactive_graph_graph::PropertyType {
        match properties {
            ActionProperties::TRIGGER => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "trigger",
                    "On receiving a boolean trigger the action will be executed",
                    reactive_graph_graph::DataType::Bool,
                    reactive_graph_graph::SocketType::Input,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
        }
    }
}

pub struct ActionPropertiesIterator(Option<ActionProperties>);

impl ActionProperties {
    pub fn into_iter() -> ActionPropertiesIterator {
        ActionPropertiesIterator(None)
    }
}

impl Iterator for ActionPropertiesIterator {
    type Item = ActionProperties;
    fn next(&mut self) -> Option<Self::Item> {
        self.0 = match self.0 {
            None => Some(ActionProperties::TRIGGER),
            Some(ActionProperties::TRIGGER) => None,
        };
        self.0.clone()
    }
}

impl IntoIterator for ActionProperties {
    type Item = ActionProperties;
    type IntoIter = ActionPropertiesIterator;
    fn into_iter(self) -> Self::IntoIter {
        ActionPropertiesIterator(None)
    }
}

impl core::fmt::Display for ActionProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match *self {
            ActionProperties::TRIGGER => core::fmt::Display::fmt("trigger", f),
        }
    }
}

///
pub static ACTION_EXTENSIONS: std::sync::LazyLock<reactive_graph_graph::Extensions> = std::sync::LazyLock::new(||
{ reactive_graph_graph::Extensions::new() });

pub static ACTION_TYPE: std::sync::LazyLock<reactive_graph_graph::Component> = std::sync::LazyLock::new(||
{
    reactive_graph_graph::Component::builder()
        .ty(core::ops::Deref::deref(&ACTION))
        .description("An action can be triggered")
        .properties(ActionProperties::property_types())
        .build()
});

/// # Component `Action`
///
/// ## Fully Qualified Namespace
///
/// `reactive_graph::core::Action`
///
/// ## Description
///
/// An action can be triggered
///
/// ### Properties
///
/// - trigger
///
/// ## JSON Schema
///
/// [https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/core/Action.schema.json]()
///
pub trait Action {
    
    /// ### Property `trigger`
    ///
    /// On receiving a boolean trigger the action will be executed
    ///
    /// Data Type: `Bool`
    ///
    /// Socket Type: `Input`
    ///
    /// Mutability: `Mutable`
    ///
    fn trigger(&self) -> bool;
    
    /// ### Property `trigger`
    ///
    /// On receiving a boolean trigger the action will be executed
    ///
    /// Data Type: `Bool`
    ///
    /// Socket Type: `Input`
    ///
    /// Mutability: `Mutable`
    ///
    fn set_trigger(&mut self, trigger: bool);
}
