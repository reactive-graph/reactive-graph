//! ---------------------------------------------
//! This file was generated automatically.
//! ---------------------------------------------
#![allow(dead_code, unused)]
#![cfg_attr(rustfmt, rustfmt_skip)]

/// Namespace `reactive_graph::flow::SimpleFlow`
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const SIMPLE_FLOW_NAMESPACE: &str = "reactive_graph::flow::SimpleFlow";

/// The [type identifier]() of EntityType `SimpleFlow`.
///
/// ### Namespace
///
/// The fully qualified namespace is
/// `reactive_graph::flow::SimpleFlow`
pub static SIMPLE_FLOW: std::sync::LazyLock<reactive_graph_graph::EntityTypeId> = std::sync::LazyLock::new(||
std::str::FromStr::from_str(SIMPLE_FLOW_NAMESPACE).unwrap());

/// The properties of EntityType `SimpleFlow`.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
pub enum SimpleFlowProperties {
    
    /// ### Property `input`
    ///
    /// Flow input
    ///
    /// Data Type: `Object`
    ///
    /// Socket Type: `Input`
    ///
    /// Mutability: `Mutable`
    ///
    INPUT,
    
    /// ### Property `output`
    ///
    /// Flow output
    ///
    /// Data Type: `Object`
    ///
    /// Socket Type: `Output`
    ///
    /// Mutability: `Mutable`
    ///
    OUTPUT,
}
impl SimpleFlowProperties {
    pub fn len() -> usize {
        2usize
    }
    
    pub fn property_types() -> reactive_graph_graph::PropertyTypes {
        let property_types = reactive_graph_graph::PropertyTypes::new();
        property_types.push(SimpleFlowProperties::INPUT);
        property_types.push(SimpleFlowProperties::OUTPUT);
        property_types
    }
}

impl AsRef<str> for SimpleFlowProperties {
    #[inline]
    fn as_ref(&self) -> &str {
        match *self {
            SimpleFlowProperties::INPUT => "input",
            SimpleFlowProperties::OUTPUT => "output",
        }
    }
}

impl From<SimpleFlowProperties> for &'static str {
    #[inline]
    fn from(properties: SimpleFlowProperties) -> &'static str {
        match properties {
            SimpleFlowProperties::INPUT => "input",
            SimpleFlowProperties::OUTPUT => "output",
        }
    }
}

impl From<SimpleFlowProperties> for String {
    #[inline]
    fn from(properties: SimpleFlowProperties) -> String {
        match properties {
            SimpleFlowProperties::INPUT => "input".to_owned(),
            SimpleFlowProperties::OUTPUT => "output".to_owned(),
        }
    }
}

impl From<SimpleFlowProperties> for reactive_graph_graph::PropertyType {
    #[inline]
    fn from(properties: SimpleFlowProperties) -> reactive_graph_graph::PropertyType {
        match properties {
            SimpleFlowProperties::INPUT => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "input",
                    "Flow input",
                    reactive_graph_graph::DataType::Object,
                    reactive_graph_graph::SocketType::Input,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            SimpleFlowProperties::OUTPUT => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "output",
                    "Flow output",
                    reactive_graph_graph::DataType::Object,
                    reactive_graph_graph::SocketType::Output,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
        }
    }
}

pub struct SimpleFlowPropertiesIterator(Option<SimpleFlowProperties>);

impl SimpleFlowProperties {
    pub fn into_iter() -> SimpleFlowPropertiesIterator {
        SimpleFlowPropertiesIterator(None)
    }
}

impl Iterator for SimpleFlowPropertiesIterator {
    type Item = SimpleFlowProperties;
    fn next(&mut self) -> Option<Self::Item> {
        self.0 = match self.0 {
            None => Some(SimpleFlowProperties::INPUT),
            Some(SimpleFlowProperties::INPUT) => Some(SimpleFlowProperties::OUTPUT),
            Some(SimpleFlowProperties::OUTPUT) => None,
        };
        self.0.clone()
    }
}

impl IntoIterator for SimpleFlowProperties {
    type Item = SimpleFlowProperties;
    type IntoIter = SimpleFlowPropertiesIterator;
    fn into_iter(self) -> Self::IntoIter {
        SimpleFlowPropertiesIterator(None)
    }
}

impl core::fmt::Display for SimpleFlowProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match *self {
            SimpleFlowProperties::INPUT => core::fmt::Display::fmt("input", f),
            SimpleFlowProperties::OUTPUT => core::fmt::Display::fmt("output", f),
        }
    }
}

///
pub static SIMPLE_FLOW_COMPONENTS: std::sync::LazyLock<
    reactive_graph_graph::ComponentTypeIds,
> = std::sync::LazyLock::new(|| { reactive_graph_graph::ComponentTypeIds::new() });

///
pub static SIMPLE_FLOW_EXTENSIONS: std::sync::LazyLock<
    reactive_graph_graph::Extensions,
> = std::sync::LazyLock::new(|| { reactive_graph_graph::Extensions::new() });

pub static SIMPLE_FLOW_TYPE: std::sync::LazyLock<reactive_graph_graph::EntityType> = std::sync::LazyLock::new(||
{
    reactive_graph_graph::EntityType::builder()
        .ty(core::ops::Deref::deref(&SIMPLE_FLOW))
        .description("A simple generic flow with a single input and a single output")
        .components(SIMPLE_FLOW_COMPONENTS.clone())
        .properties(SimpleFlowProperties::property_types())
        .extensions(SIMPLE_FLOW_EXTENSIONS.clone())
        .build()
});

/// # EntityType `SimpleFlow`
///
/// ## Fully Qualified Namespace
///
/// `reactive_graph::flow::SimpleFlow`
///
/// ## Description
///
/// A simple generic flow with a single input and a single output
///
/// ### Properties
///
/// - input
/// - output
///
/// ## JSON Schema
///
/// [https://schema.reactive-graph.io/schema/json/dynamic_graph/types/entity/reactive_graph/flow/SimpleFlow.schema.json]()
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
pub struct SimpleFlow {
    #[builder(default, setter(into))]
    pub id: uuid::Uuid,
    
    /// ### Property `input`
    ///
    /// Flow input
    ///
    /// Data Type: `Object`
    ///
    /// Socket Type: `Input`
    ///
    /// Mutability: `Mutable`
    ///
    #[builder(setter(into))]
    pub input: serde_json::Map<String, serde_json::Value>,
    
    /// ### Property `output`
    ///
    /// Flow output
    ///
    /// Data Type: `Object`
    ///
    /// Socket Type: `Output`
    ///
    /// Mutability: `Mutable`
    ///
    #[builder(setter(into))]
    pub output: serde_json::Map<String, serde_json::Value>,
    #[builder(default, setter(into))]
    pub extensions: reactive_graph_graph::Extensions,
}

impl SimpleFlow {
    pub fn new(
        input: serde_json::Map<String, serde_json::Value>,
        output: serde_json::Map<String, serde_json::Value>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            input,
            output,
            extensions: reactive_graph_graph::Extensions::new(),
        }
    }
    pub fn new_with_id(
        id: uuid::Uuid,
        input: serde_json::Map<String, serde_json::Value>,
        output: serde_json::Map<String, serde_json::Value>,
    ) -> Self {
        Self {
            id,
            input,
            output,
            extensions: reactive_graph_graph::Extensions::new(),
        }
    }
    pub fn new_with_extensions(
        id: uuid::Uuid,
        input: serde_json::Map<String, serde_json::Value>,
        output: serde_json::Map<String, serde_json::Value>,
        extensions: reactive_graph_graph::Extensions,
    ) -> Self {
        Self {
            id,
            input,
            output,
            extensions,
        }
    }
    
    pub fn id(&self) -> uuid::Uuid {
        self.id
    }
    
    /// ### Property `input`
    ///
    /// Flow input
    ///
    /// Data Type: `Object`
    ///
    /// Socket Type: `Input`
    ///
    /// Mutability: `Mutable`
    ///
    pub fn input(&self) -> serde_json::Map<String, serde_json::Value> {
        self.input.clone()
    }
    
    /// ### Property `input`
    ///
    /// Flow input
    ///
    /// Data Type: `Object`
    ///
    /// Socket Type: `Input`
    ///
    /// Mutability: `Mutable`
    ///
    pub fn set_input(&mut self, input: serde_json::Map<String, serde_json::Value>) {
        self.input = input;
    }
    
    /// ### Property `output`
    ///
    /// Flow output
    ///
    /// Data Type: `Object`
    ///
    /// Socket Type: `Output`
    ///
    /// Mutability: `Mutable`
    ///
    pub fn output(&self) -> serde_json::Map<String, serde_json::Value> {
        self.output.clone()
    }
    
    /// ### Property `output`
    ///
    /// Flow output
    ///
    /// Data Type: `Object`
    ///
    /// Socket Type: `Output`
    ///
    /// Mutability: `Mutable`
    ///
    pub fn set_output(&mut self, output: serde_json::Map<String, serde_json::Value>) {
        self.output = output;
    }
    
    pub fn properties(&self) -> reactive_graph_graph::PropertyInstances {
        reactive_graph_graph::PropertyInstances::new()
            .property(SimpleFlowProperties::INPUT, self.input.clone())
            .property(SimpleFlowProperties::OUTPUT, self.output.clone())
    }
    
    pub fn extensions(&self) -> reactive_graph_graph::Extensions {
        self.extensions.clone()
    }
}

impl From<SimpleFlow> for reactive_graph_graph::EntityInstance {
    fn from(simple_flow: SimpleFlow) -> Self {
        reactive_graph_graph::EntityInstance::builder()
            .ty(std::ops::Deref::deref(&SIMPLE_FLOW))
            .id(simple_flow.id())
            .components(SIMPLE_FLOW_COMPONENTS.clone())
            .properties(simple_flow.properties())
            .build()
    }
}

impl TryFrom<reactive_graph_graph::EntityInstance> for SimpleFlow {
    type Error = ();
    fn try_from(
        entity_instance: reactive_graph_graph::EntityInstance,
    ) -> Result<Self, Self::Error> {
        Err(())
    }
}
