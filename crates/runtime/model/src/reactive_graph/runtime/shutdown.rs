//! ---------------------------------------------
//! This file was generated automatically.
//! ---------------------------------------------
#![allow(dead_code, unused)]
#![cfg_attr(rustfmt, rustfmt_skip)]

/// Namespace `reactive_graph::runtime::Shutdown`
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const SHUTDOWN_NAMESPACE: &str = "reactive_graph::runtime::Shutdown";

/// The [type identifier]() of EntityType `Shutdown`.
///
/// ### Namespace
///
/// The fully qualified namespace is
/// `reactive_graph::runtime::Shutdown`
pub static SHUTDOWN: std::sync::LazyLock<reactive_graph_graph::EntityTypeId> = std::sync::LazyLock::new(||
std::str::FromStr::from_str(SHUTDOWN_NAMESPACE).unwrap());

/// The properties of EntityType `Shutdown`.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
pub enum ShutdownProperties {
    
    /// ### Property `delay`
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    DELAY,
    
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
impl ShutdownProperties {
    pub fn len() -> usize {
        3usize
    }
    
    pub fn property_types() -> reactive_graph_graph::PropertyTypes {
        let property_types = reactive_graph_graph::PropertyTypes::new();
        property_types.push(ShutdownProperties::DELAY);
        property_types.push(ShutdownProperties::LABEL);
        property_types.push(ShutdownProperties::TRIGGER);
        property_types
    }
}

impl AsRef<str> for ShutdownProperties {
    #[inline]
    fn as_ref(&self) -> &str {
        match *self {
            ShutdownProperties::DELAY => "delay",
            ShutdownProperties::LABEL => "label",
            ShutdownProperties::TRIGGER => "trigger",
        }
    }
}

impl From<ShutdownProperties> for &'static str {
    #[inline]
    fn from(properties: ShutdownProperties) -> &'static str {
        match properties {
            ShutdownProperties::DELAY => "delay",
            ShutdownProperties::LABEL => "label",
            ShutdownProperties::TRIGGER => "trigger",
        }
    }
}

impl From<ShutdownProperties> for String {
    #[inline]
    fn from(properties: ShutdownProperties) -> String {
        match properties {
            ShutdownProperties::DELAY => "delay".to_owned(),
            ShutdownProperties::LABEL => "label".to_owned(),
            ShutdownProperties::TRIGGER => "trigger".to_owned(),
        }
    }
}

impl From<ShutdownProperties> for reactive_graph_graph::PropertyType {
    #[inline]
    fn from(properties: ShutdownProperties) -> reactive_graph_graph::PropertyType {
        match properties {
            ShutdownProperties::DELAY => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "delay",
                    "",
                    reactive_graph_graph::DataType::Number,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            ShutdownProperties::LABEL => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "label",
                    "Hierarchical path with static segments, named parameters and catch-all parameters",
                    reactive_graph_graph::DataType::String,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            ShutdownProperties::TRIGGER => {
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

pub struct ShutdownPropertiesIterator(Option<ShutdownProperties>);

impl ShutdownProperties {
    pub fn into_iter() -> ShutdownPropertiesIterator {
        ShutdownPropertiesIterator(None)
    }
}

impl Iterator for ShutdownPropertiesIterator {
    type Item = ShutdownProperties;
    fn next(&mut self) -> Option<Self::Item> {
        self.0 = match self.0 {
            None => Some(ShutdownProperties::DELAY),
            Some(ShutdownProperties::DELAY) => Some(ShutdownProperties::LABEL),
            Some(ShutdownProperties::LABEL) => Some(ShutdownProperties::TRIGGER),
            Some(ShutdownProperties::TRIGGER) => None,
        };
        self.0.clone()
    }
}

impl IntoIterator for ShutdownProperties {
    type Item = ShutdownProperties;
    type IntoIter = ShutdownPropertiesIterator;
    fn into_iter(self) -> Self::IntoIter {
        ShutdownPropertiesIterator(None)
    }
}

impl core::fmt::Display for ShutdownProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match *self {
            ShutdownProperties::DELAY => core::fmt::Display::fmt("delay", f),
            ShutdownProperties::LABEL => core::fmt::Display::fmt("label", f),
            ShutdownProperties::TRIGGER => core::fmt::Display::fmt("trigger", f),
        }
    }
}

/// ## Components
///
/// | Component                       | Description                                                                                       | Properties                          |
/// |---------------------------------|---------------------------------------------------------------------------------------------------|-------------------------------------|
/// | `reactive_graph::core::Action`  | An action can be triggered                                                                        | <ul compact><li>`trigger`</li></ul> |
/// | `reactive_graph::core::Labeled` | The label is a hierarchical path with static segments, named parameters and catch-all parameters. | <ul compact><li>`label`</li></ul>   |
///
pub static SHUTDOWN_COMPONENTS: std::sync::LazyLock<
    reactive_graph_graph::ComponentTypeIds,
> = std::sync::LazyLock::new(|| {
    reactive_graph_graph::ComponentTypeIds::new()
        .component(
            std::ops::Deref::deref(
                &reactive_graph_model_core::reactive_graph::core::action::ACTION,
            ),
        )
        .component(
            std::ops::Deref::deref(
                &reactive_graph_model_core::reactive_graph::core::labeled::LABELED,
            ),
        )
});

///
pub static SHUTDOWN_EXTENSIONS: std::sync::LazyLock<reactive_graph_graph::Extensions> = std::sync::LazyLock::new(||
{ reactive_graph_graph::Extensions::new() });

pub static SHUTDOWN_TYPE: std::sync::LazyLock<reactive_graph_graph::EntityType> = std::sync::LazyLock::new(||
{
    reactive_graph_graph::EntityType::builder()
        .ty(core::ops::Deref::deref(&SHUTDOWN))
        .description("Shutting down the runtime")
        .components(SHUTDOWN_COMPONENTS.clone())
        .properties(ShutdownProperties::property_types())
        .extensions(SHUTDOWN_EXTENSIONS.clone())
        .build()
});

/// # EntityType `Shutdown`
///
/// ## Fully Qualified Namespace
///
/// `reactive_graph::runtime::Shutdown`
///
/// ## Description
///
/// Shutting down the runtime
///
/// ## Components
///
/// | Component                       | Description                                                                                       | Properties                          |
/// |---------------------------------|---------------------------------------------------------------------------------------------------|-------------------------------------|
/// | `reactive_graph::core::Labeled` | The label is a hierarchical path with static segments, named parameters and catch-all parameters. | <ul compact><li>`label`</li></ul>   |
/// | `reactive_graph::core::Action`  | An action can be triggered                                                                        | <ul compact><li>`trigger`</li></ul> |
///
/// ### Properties
///
/// - delay
///
/// ### Properties from components
///
/// - label
/// - trigger
///
/// ## JSON Schema
///
/// [https://schema.reactive-graph.io/schema/json/dynamic_graph/types/entity/reactive_graph/runtime/Shutdown.schema.json]()
///
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    typed_builder::TypedBuilder
)]
pub struct Shutdown {
    #[builder(default, setter(into))]
    pub id: uuid::Uuid,
    
    /// ### Property `delay`
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    pub delay: u64,
    
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
    #[builder(setter(into))]
    pub label: String,
    
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
    pub trigger: bool,
    #[builder(default, setter(into))]
    pub extensions: reactive_graph_graph::Extensions,
}

impl Shutdown {
    pub fn new(delay: u64, label: String, trigger: bool) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            delay,
            label,
            trigger,
            extensions: reactive_graph_graph::Extensions::new(),
        }
    }
    pub fn new_with_id(
        id: uuid::Uuid,
        delay: u64,
        label: String,
        trigger: bool,
    ) -> Self {
        Self {
            id,
            delay,
            label,
            trigger,
            extensions: reactive_graph_graph::Extensions::new(),
        }
    }
    pub fn new_with_extensions(
        id: uuid::Uuid,
        delay: u64,
        label: String,
        trigger: bool,
        extensions: reactive_graph_graph::Extensions,
    ) -> Self {
        Self {
            id,
            delay,
            label,
            trigger,
            extensions,
        }
    }
    
    pub fn id(&self) -> uuid::Uuid {
        self.id
    }
    
    /// ### Property `delay`
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    pub fn delay(&self) -> u64 {
        self.delay
    }
    
    /// ### Property `delay`
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    pub fn set_delay(&mut self, delay: u64) {
        self.delay = delay;
    }
    
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
    pub fn label(&self) -> String {
        self.label.clone()
    }
    
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
    pub fn set_label(&mut self, label: String) {
        self.label = label;
    }
    
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
    pub fn trigger(&self) -> bool {
        self.trigger
    }
    
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
    pub fn set_trigger(&mut self, trigger: bool) {
        self.trigger = trigger;
    }
    
    pub fn properties(&self) -> reactive_graph_graph::PropertyInstances {
        reactive_graph_graph::PropertyInstances::new()
            .property(ShutdownProperties::DELAY, self.delay.clone())
            .property(ShutdownProperties::LABEL, self.label.clone())
            .property(ShutdownProperties::TRIGGER, self.trigger.clone())
    }
    
    pub fn extensions(&self) -> reactive_graph_graph::Extensions {
        self.extensions.clone()
    }
}

impl From<Shutdown> for reactive_graph_graph::EntityInstance {
    fn from(shutdown: Shutdown) -> Self {
        reactive_graph_graph::EntityInstance::builder()
            .ty(std::ops::Deref::deref(&SHUTDOWN))
            .id(shutdown.id())
            .components(SHUTDOWN_COMPONENTS.clone())
            .properties(shutdown.properties())
            .build()
    }
}

impl TryFrom<reactive_graph_graph::EntityInstance> for Shutdown {
    type Error = ();
    fn try_from(
        entity_instance: reactive_graph_graph::EntityInstance,
    ) -> Result<Self, Self::Error> {
        Err(())
    }
}
impl reactive_graph_model_core::reactive_graph::core::labeled::Labeled for Shutdown {
    
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
    fn label(&self) -> String {
        self.label.clone()
    }
    
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
    fn set_label(&mut self, label: String) {
        self.label = label;
    }
}
impl reactive_graph_model_core::reactive_graph::core::action::Action for Shutdown {
    
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
    fn trigger(&self) -> bool {
        self.trigger
    }
    
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
    fn set_trigger(&mut self, trigger: bool) {
        self.trigger = trigger;
    }
}
