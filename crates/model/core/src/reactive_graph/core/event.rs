//! ---------------------------------------------
//! This file was generated automatically.
//! ---------------------------------------------
#![allow(dead_code, unused)]
#![cfg_attr(rustfmt, rustfmt_skip)]

/// Namespace `reactive_graph::core::Event`
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const EVENT_NAMESPACE: &str = "reactive_graph::core::Event";

/// The [type identifier]() of Component `Event`.
///
/// ### Namespace
///
/// The fully qualified namespace is
/// `reactive_graph::core::Event`
pub static EVENT: std::sync::LazyLock<reactive_graph_graph::ComponentTypeId> = std::sync::LazyLock::new(||
std::str::FromStr::from_str(EVENT_NAMESPACE).unwrap());

/// The properties of Component `Event`.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
pub enum EventProperties {
    
    /// ### Property `event`
    ///
    /// On receiving a boolean trigger the action will be executed
    ///
    /// Data Type: `Any`
    ///
    /// Socket Type: `Output`
    ///
    /// Mutability: `Mutable`
    ///
    EVENT,
}
impl EventProperties {
    pub fn len() -> usize {
        1usize
    }
    
    pub fn property_types() -> reactive_graph_graph::PropertyTypes {
        let property_types = reactive_graph_graph::PropertyTypes::new();
        property_types.push(EventProperties::EVENT);
        property_types
    }
}

impl AsRef<str> for EventProperties {
    #[inline]
    fn as_ref(&self) -> &str {
        match *self {
            EventProperties::EVENT => "event",
        }
    }
}

impl From<EventProperties> for &'static str {
    #[inline]
    fn from(properties: EventProperties) -> &'static str {
        match properties {
            EventProperties::EVENT => "event",
        }
    }
}

impl From<EventProperties> for String {
    #[inline]
    fn from(properties: EventProperties) -> String {
        match properties {
            EventProperties::EVENT => "event".to_owned(),
        }
    }
}

impl From<EventProperties> for reactive_graph_graph::PropertyType {
    #[inline]
    fn from(properties: EventProperties) -> reactive_graph_graph::PropertyType {
        match properties {
            EventProperties::EVENT => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "event",
                    "On receiving a boolean trigger the action will be executed",
                    reactive_graph_graph::DataType::Any,
                    reactive_graph_graph::SocketType::Output,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
        }
    }
}

pub struct EventPropertiesIterator(Option<EventProperties>);

impl EventProperties {
    pub fn into_iter() -> EventPropertiesIterator {
        EventPropertiesIterator(None)
    }
}

impl Iterator for EventPropertiesIterator {
    type Item = EventProperties;
    fn next(&mut self) -> Option<Self::Item> {
        self.0 = match self.0 {
            None => Some(EventProperties::EVENT),
            Some(EventProperties::EVENT) => None,
        };
        self.0.clone()
    }
}

impl IntoIterator for EventProperties {
    type Item = EventProperties;
    type IntoIter = EventPropertiesIterator;
    fn into_iter(self) -> Self::IntoIter {
        EventPropertiesIterator(None)
    }
}

impl core::fmt::Display for EventProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match *self {
            EventProperties::EVENT => core::fmt::Display::fmt("event", f),
        }
    }
}

///
pub static EVENT_EXTENSIONS: std::sync::LazyLock<reactive_graph_graph::Extensions> = std::sync::LazyLock::new(||
{ reactive_graph_graph::Extensions::new() });

pub static EVENT_TYPE: std::sync::LazyLock<reactive_graph_graph::Component> = std::sync::LazyLock::new(||
{
    reactive_graph_graph::Component::builder()
        .ty(core::ops::Deref::deref(&EVENT))
        .description("This component spawns events.")
        .properties(EventProperties::property_types())
        .build()
});

/// # Component `Event`
///
/// ## Fully Qualified Namespace
///
/// `reactive_graph::core::Event`
///
/// ## Description
///
/// This component spawns events.
///
/// ### Properties
///
/// - event
///
/// ## JSON Schema
///
/// [https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/core/Event.schema.json]()
///
pub trait Event {
    
    /// ### Property `event`
    ///
    /// On receiving a boolean trigger the action will be executed
    ///
    /// Data Type: `Any`
    ///
    /// Socket Type: `Output`
    ///
    /// Mutability: `Mutable`
    ///
    fn event(&self) -> serde_json::Value;
    
    /// ### Property `event`
    ///
    /// On receiving a boolean trigger the action will be executed
    ///
    /// Data Type: `Any`
    ///
    /// Socket Type: `Output`
    ///
    /// Mutability: `Mutable`
    ///
    fn set_event(&mut self, event: serde_json::Value);
}
