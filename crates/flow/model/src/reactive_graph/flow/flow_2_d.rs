//! ---------------------------------------------
//! This file was generated automatically.
//! ---------------------------------------------
#![allow(dead_code, unused)]
#![cfg_attr(rustfmt, rustfmt_skip)]

/// Namespace `reactive_graph::flow::Flow2D`
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const FLOW_2_D_NAMESPACE: &str = "reactive_graph::flow::Flow2D";

/// The [type identifier]() of Component `Flow2D`.
///
/// ### Namespace
///
/// The fully qualified namespace is
/// `reactive_graph::flow::Flow2D`
pub static FLOW_2_D: std::sync::LazyLock<reactive_graph_graph::ComponentTypeId> = std::sync::LazyLock::new(||
std::str::FromStr::from_str(FLOW_2_D_NAMESPACE).unwrap());

/// The properties of Component `Flow2D`.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
pub enum Flow2DProperties {
    
    /// ### Property `f2dh`
    ///
    /// The height (y-axis) in a 2D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    F_2_DH,
    
    /// ### Property `f2dw`
    ///
    /// The width (x-axis) in a 2D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    F_2_DW,
    
    /// ### Property `f2dx`
    ///
    /// The X position in a 2D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    F_2_DX,
    
    /// ### Property `f2dy`
    ///
    /// The Y position in a 2D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    F_2_DY,
}
impl Flow2DProperties {
    pub fn len() -> usize {
        4usize
    }
    
    pub fn property_types() -> reactive_graph_graph::PropertyTypes {
        let property_types = reactive_graph_graph::PropertyTypes::new();
        property_types.push(Flow2DProperties::F_2_DH);
        property_types.push(Flow2DProperties::F_2_DW);
        property_types.push(Flow2DProperties::F_2_DX);
        property_types.push(Flow2DProperties::F_2_DY);
        property_types
    }
}

impl AsRef<str> for Flow2DProperties {
    #[inline]
    fn as_ref(&self) -> &str {
        match *self {
            Flow2DProperties::F_2_DH => "f2dh",
            Flow2DProperties::F_2_DW => "f2dw",
            Flow2DProperties::F_2_DX => "f2dx",
            Flow2DProperties::F_2_DY => "f2dy",
        }
    }
}

impl From<Flow2DProperties> for &'static str {
    #[inline]
    fn from(properties: Flow2DProperties) -> &'static str {
        match properties {
            Flow2DProperties::F_2_DH => "f2dh",
            Flow2DProperties::F_2_DW => "f2dw",
            Flow2DProperties::F_2_DX => "f2dx",
            Flow2DProperties::F_2_DY => "f2dy",
        }
    }
}

impl From<Flow2DProperties> for String {
    #[inline]
    fn from(properties: Flow2DProperties) -> String {
        match properties {
            Flow2DProperties::F_2_DH => "f2dh".to_owned(),
            Flow2DProperties::F_2_DW => "f2dw".to_owned(),
            Flow2DProperties::F_2_DX => "f2dx".to_owned(),
            Flow2DProperties::F_2_DY => "f2dy".to_owned(),
        }
    }
}

impl From<Flow2DProperties> for reactive_graph_graph::PropertyType {
    #[inline]
    fn from(properties: Flow2DProperties) -> reactive_graph_graph::PropertyType {
        match properties {
            Flow2DProperties::F_2_DH => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "f2dh",
                    "The height (y-axis) in a 2D flow",
                    reactive_graph_graph::DataType::Number,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            Flow2DProperties::F_2_DW => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "f2dw",
                    "The width (x-axis) in a 2D flow",
                    reactive_graph_graph::DataType::Number,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            Flow2DProperties::F_2_DX => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "f2dx",
                    "The X position in a 2D flow",
                    reactive_graph_graph::DataType::Number,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            Flow2DProperties::F_2_DY => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "f2dy",
                    "The Y position in a 2D flow",
                    reactive_graph_graph::DataType::Number,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
        }
    }
}

pub struct Flow2DPropertiesIterator(Option<Flow2DProperties>);

impl Flow2DProperties {
    pub fn into_iter() -> Flow2DPropertiesIterator {
        Flow2DPropertiesIterator(None)
    }
}

impl Iterator for Flow2DPropertiesIterator {
    type Item = Flow2DProperties;
    fn next(&mut self) -> Option<Self::Item> {
        self.0 = match self.0 {
            None => Some(Flow2DProperties::F_2_DH),
            Some(Flow2DProperties::F_2_DH) => Some(Flow2DProperties::F_2_DW),
            Some(Flow2DProperties::F_2_DW) => Some(Flow2DProperties::F_2_DX),
            Some(Flow2DProperties::F_2_DX) => Some(Flow2DProperties::F_2_DY),
            Some(Flow2DProperties::F_2_DY) => None,
        };
        self.0.clone()
    }
}

impl IntoIterator for Flow2DProperties {
    type Item = Flow2DProperties;
    type IntoIter = Flow2DPropertiesIterator;
    fn into_iter(self) -> Self::IntoIter {
        Flow2DPropertiesIterator(None)
    }
}

impl core::fmt::Display for Flow2DProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match *self {
            Flow2DProperties::F_2_DH => core::fmt::Display::fmt("f2dh", f),
            Flow2DProperties::F_2_DW => core::fmt::Display::fmt("f2dw", f),
            Flow2DProperties::F_2_DX => core::fmt::Display::fmt("f2dx", f),
            Flow2DProperties::F_2_DY => core::fmt::Display::fmt("f2dy", f),
        }
    }
}

///
pub static FLOW_2_D_EXTENSIONS: std::sync::LazyLock<reactive_graph_graph::Extensions> = std::sync::LazyLock::new(||
{ reactive_graph_graph::Extensions::new() });

pub static FLOW_2_D_TYPE: std::sync::LazyLock<reactive_graph_graph::Component> = std::sync::LazyLock::new(||
{
    reactive_graph_graph::Component::builder()
        .ty(core::ops::Deref::deref(&FLOW_2_D))
        .description(
            "The position (x,y) of the entity or relation on a two dimensional flow.",
        )
        .properties(Flow2DProperties::property_types())
        .build()
});

/// # Component `Flow2D`
///
/// ## Fully Qualified Namespace
///
/// `reactive_graph::flow::Flow2D`
///
/// ## Description
///
/// The position (x,y) of the entity or relation on a two dimensional flow.
///
/// ### Properties
///
/// - f2dh
/// - f2dw
/// - f2dx
/// - f2dy
///
/// ## JSON Schema
///
/// [https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/flow/Flow2D.schema.json]()
///
pub trait Flow2D {
    
    /// ### Property `f2dh`
    ///
    /// The height (y-axis) in a 2D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn f2dh(&self) -> u64;
    
    /// ### Property `f2dh`
    ///
    /// The height (y-axis) in a 2D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn set_f2dh(&mut self, f2dh: u64);
    
    /// ### Property `f2dw`
    ///
    /// The width (x-axis) in a 2D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn f2dw(&self) -> u64;
    
    /// ### Property `f2dw`
    ///
    /// The width (x-axis) in a 2D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn set_f2dw(&mut self, f2dw: u64);
    
    /// ### Property `f2dx`
    ///
    /// The X position in a 2D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn f2dx(&self) -> u64;
    
    /// ### Property `f2dx`
    ///
    /// The X position in a 2D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn set_f2dx(&mut self, f2dx: u64);
    
    /// ### Property `f2dy`
    ///
    /// The Y position in a 2D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn f2dy(&self) -> u64;
    
    /// ### Property `f2dy`
    ///
    /// The Y position in a 2D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn set_f2dy(&mut self, f2dy: u64);
}
