//! ---------------------------------------------
//! This file was generated automatically.
//! ---------------------------------------------
#![allow(dead_code, unused)]
#![cfg_attr(rustfmt, rustfmt_skip)]

/// Namespace `reactive_graph::type_system::TypeSystemEvent`
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const TYPE_SYSTEM_EVENT_NAMESPACE: &str = "reactive_graph::type_system::TypeSystemEvent";

/// The [type identifier]() of EntityType `TypeSystemEvent`.
///
/// ### Namespace
///
/// The fully qualified namespace is
/// `reactive_graph::type_system::TypeSystemEvent`
pub static TYPE_SYSTEM_EVENT: std::sync::LazyLock<reactive_graph_graph::EntityTypeId> = std::sync::LazyLock::new(||
std::str::FromStr::from_str(TYPE_SYSTEM_EVENT_NAMESPACE).unwrap());

/// The properties of EntityType `TypeSystemEvent`.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
pub enum TypeSystemEventProperties {
    
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
impl TypeSystemEventProperties {
    pub fn len() -> usize {
        2usize
    }
    
    pub fn property_types() -> reactive_graph_graph::PropertyTypes {
        let property_types = reactive_graph_graph::PropertyTypes::new();
        property_types.push(TypeSystemEventProperties::EVENT);
        property_types.push(TypeSystemEventProperties::LABEL);
        property_types
    }
}

impl AsRef<str> for TypeSystemEventProperties {
    #[inline]
    fn as_ref(&self) -> &str {
        match *self {
            TypeSystemEventProperties::EVENT => "event",
            TypeSystemEventProperties::LABEL => "label",
        }
    }
}

impl From<TypeSystemEventProperties> for &'static str {
    #[inline]
    fn from(properties: TypeSystemEventProperties) -> &'static str {
        match properties {
            TypeSystemEventProperties::EVENT => "event",
            TypeSystemEventProperties::LABEL => "label",
        }
    }
}

impl From<TypeSystemEventProperties> for String {
    #[inline]
    fn from(properties: TypeSystemEventProperties) -> String {
        match properties {
            TypeSystemEventProperties::EVENT => "event".to_owned(),
            TypeSystemEventProperties::LABEL => "label".to_owned(),
        }
    }
}

impl From<TypeSystemEventProperties> for reactive_graph_graph::PropertyType {
    #[inline]
    fn from(
        properties: TypeSystemEventProperties,
    ) -> reactive_graph_graph::PropertyType {
        match properties {
            TypeSystemEventProperties::EVENT => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "event",
                    "On receiving a boolean trigger the action will be executed",
                    reactive_graph_graph::DataType::Any,
                    reactive_graph_graph::SocketType::Output,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            TypeSystemEventProperties::LABEL => {
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

pub struct TypeSystemEventPropertiesIterator(Option<TypeSystemEventProperties>);

impl TypeSystemEventProperties {
    pub fn into_iter() -> TypeSystemEventPropertiesIterator {
        TypeSystemEventPropertiesIterator(None)
    }
}

impl Iterator for TypeSystemEventPropertiesIterator {
    type Item = TypeSystemEventProperties;
    fn next(&mut self) -> Option<Self::Item> {
        self.0 = match self.0 {
            None => Some(TypeSystemEventProperties::EVENT),
            Some(TypeSystemEventProperties::EVENT) => {
                Some(TypeSystemEventProperties::LABEL)
            }
            Some(TypeSystemEventProperties::LABEL) => None,
        };
        self.0.clone()
    }
}

impl IntoIterator for TypeSystemEventProperties {
    type Item = TypeSystemEventProperties;
    type IntoIter = TypeSystemEventPropertiesIterator;
    fn into_iter(self) -> Self::IntoIter {
        TypeSystemEventPropertiesIterator(None)
    }
}

impl core::fmt::Display for TypeSystemEventProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match *self {
            TypeSystemEventProperties::EVENT => core::fmt::Display::fmt("event", f),
            TypeSystemEventProperties::LABEL => core::fmt::Display::fmt("label", f),
        }
    }
}

/// ## Components
///
/// | Component                       | Description                                                                                       | Properties                        |
/// |---------------------------------|---------------------------------------------------------------------------------------------------|-----------------------------------|
/// | `reactive_graph::core::Labeled` | The label is a hierarchical path with static segments, named parameters and catch-all parameters. | <ul compact><li>`label`</li></ul> |
/// | `reactive_graph::core::Event`   | This component spawns events.                                                                     | <ul compact><li>`event`</li></ul> |
///
pub static TYPE_SYSTEM_EVENT_COMPONENTS: std::sync::LazyLock<
    reactive_graph_graph::ComponentTypeIds,
> = std::sync::LazyLock::new(|| {
    reactive_graph_graph::ComponentTypeIds::new()
        .component(
            std::ops::Deref::deref(
                &reactive_graph_model_core::reactive_graph::core::event::EVENT,
            ),
        )
        .component(
            std::ops::Deref::deref(
                &reactive_graph_model_core::reactive_graph::core::labeled::LABELED,
            ),
        )
});

///
pub static TYPE_SYSTEM_EVENT_EXTENSIONS: std::sync::LazyLock<
    reactive_graph_graph::Extensions,
> = std::sync::LazyLock::new(|| { reactive_graph_graph::Extensions::new() });

pub static TYPE_SYSTEM_EVENT_TYPE: std::sync::LazyLock<
    reactive_graph_graph::EntityType,
> = std::sync::LazyLock::new(|| {
    reactive_graph_graph::EntityType::builder()
        .ty(core::ops::Deref::deref(&TYPE_SYSTEM_EVENT))
        .description("Events of the type system")
        .components(TYPE_SYSTEM_EVENT_COMPONENTS.clone())
        .properties(TypeSystemEventProperties::property_types())
        .extensions(TYPE_SYSTEM_EVENT_EXTENSIONS.clone())
        .build()
});

/// # EntityType `TypeSystemEvent`
///
/// ## Fully Qualified Namespace
///
/// `reactive_graph::type_system::TypeSystemEvent`
///
/// ## Description
///
/// Events of the type system
///
/// ## Components
///
/// | Component                       | Description                                                                                       | Properties                        |
/// |---------------------------------|---------------------------------------------------------------------------------------------------|-----------------------------------|
/// | `reactive_graph::core::Event`   | This component spawns events.                                                                     | <ul compact><li>`event`</li></ul> |
/// | `reactive_graph::core::Labeled` | The label is a hierarchical path with static segments, named parameters and catch-all parameters. | <ul compact><li>`label`</li></ul> |
///
/// ### Properties from components
///
/// - event
/// - label
///
/// ## JSON Schema
///
/// [https://schema.reactive-graph.io/schema/json/dynamic_graph/types/entity/reactive_graph/type_system/TypeSystemEvent.schema.json]()
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
pub struct TypeSystemEvent {
    #[builder(default, setter(into))]
    pub id: uuid::Uuid,
    
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
    pub event: serde_json::Value,
    
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
    #[builder(default, setter(into))]
    pub extensions: reactive_graph_graph::Extensions,
}

impl TypeSystemEvent {
    pub fn new(event: serde_json::Value, label: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            event,
            label,
            extensions: reactive_graph_graph::Extensions::new(),
        }
    }
    pub fn new_with_id(id: uuid::Uuid, event: serde_json::Value, label: String) -> Self {
        Self {
            id,
            event,
            label,
            extensions: reactive_graph_graph::Extensions::new(),
        }
    }
    pub fn new_with_extensions(
        id: uuid::Uuid,
        event: serde_json::Value,
        label: String,
        extensions: reactive_graph_graph::Extensions,
    ) -> Self {
        Self {
            id,
            event,
            label,
            extensions,
        }
    }
    
    pub fn id(&self) -> uuid::Uuid {
        self.id
    }
    
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
    pub fn event(&self) -> serde_json::Value {
        self.event.clone()
    }
    
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
    pub fn set_event(&mut self, event: serde_json::Value) {
        self.event = event;
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
    
    pub fn properties(&self) -> reactive_graph_graph::PropertyInstances {
        reactive_graph_graph::PropertyInstances::new()
            .property(TypeSystemEventProperties::EVENT, self.event.clone())
            .property(TypeSystemEventProperties::LABEL, self.label.clone())
    }
    
    pub fn extensions(&self) -> reactive_graph_graph::Extensions {
        self.extensions.clone()
    }
}

impl From<TypeSystemEvent> for reactive_graph_graph::EntityInstance {
    fn from(type_system_event: TypeSystemEvent) -> Self {
        reactive_graph_graph::EntityInstance::builder()
            .ty(std::ops::Deref::deref(&TYPE_SYSTEM_EVENT))
            .id(type_system_event.id())
            .components(TYPE_SYSTEM_EVENT_COMPONENTS.clone())
            .properties(type_system_event.properties())
            .build()
    }
}

impl TryFrom<reactive_graph_graph::EntityInstance> for TypeSystemEvent {
    type Error = ();
    fn try_from(
        entity_instance: reactive_graph_graph::EntityInstance,
    ) -> Result<Self, Self::Error> {
        Err(())
    }
}
impl reactive_graph_model_core::reactive_graph::core::labeled::Labeled
for TypeSystemEvent {
    
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
impl reactive_graph_model_core::reactive_graph::core::event::Event for TypeSystemEvent {
    
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
    fn event(&self) -> serde_json::Value {
        self.event.clone()
    }
    
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
    fn set_event(&mut self, event: serde_json::Value) {
        self.event = event;
    }
}
