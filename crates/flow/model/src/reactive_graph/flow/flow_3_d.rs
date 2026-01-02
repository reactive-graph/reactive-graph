//! ---------------------------------------------
//! This file was generated automatically.
//! ---------------------------------------------
#![allow(dead_code, unused)]
#![cfg_attr(rustfmt, rustfmt_skip)]

/// Namespace `reactive_graph::flow::Flow3D`
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const FLOW_3_D_NAMESPACE: &str = "reactive_graph::flow::Flow3D";

/// The [type identifier]() of Component `Flow3D`.
///
/// ### Namespace
///
/// The fully qualified namespace is
/// `reactive_graph::flow::Flow3D`
pub static FLOW_3_D: std::sync::LazyLock<reactive_graph_graph::ComponentTypeId> = std::sync::LazyLock::new(||
std::str::FromStr::from_str(FLOW_3_D_NAMESPACE).unwrap());

/// The properties of Component `Flow3D`.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
pub enum Flow3DProperties {
    
    /// ### Property `f3dd`
    ///
    /// The depth (z-axis) in a 3D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    F_3_DD,
    
    /// ### Property `f3dh`
    ///
    /// The height (y-axis) in a 3D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    F_3_DH,
    
    /// ### Property `f3dw`
    ///
    /// The width (x-axis) in a 3D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    F_3_DW,
    
    /// ### Property `f3dx`
    ///
    /// The X position in a 3D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    F_3_DX,
    
    /// ### Property `f3dy`
    ///
    /// The Y position in a 3D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    F_3_DY,
    
    /// ### Property `f3dz`
    ///
    /// The Z position in a 3D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    F_3_DZ,
}
impl Flow3DProperties {
    pub fn len() -> usize {
        6usize
    }
    
    pub fn property_types() -> reactive_graph_graph::PropertyTypes {
        let property_types = reactive_graph_graph::PropertyTypes::new();
        property_types.push(Flow3DProperties::F_3_DD);
        property_types.push(Flow3DProperties::F_3_DH);
        property_types.push(Flow3DProperties::F_3_DW);
        property_types.push(Flow3DProperties::F_3_DX);
        property_types.push(Flow3DProperties::F_3_DY);
        property_types.push(Flow3DProperties::F_3_DZ);
        property_types
    }
}

impl AsRef<str> for Flow3DProperties {
    #[inline]
    fn as_ref(&self) -> &str {
        match *self {
            Flow3DProperties::F_3_DD => "f3dd",
            Flow3DProperties::F_3_DH => "f3dh",
            Flow3DProperties::F_3_DW => "f3dw",
            Flow3DProperties::F_3_DX => "f3dx",
            Flow3DProperties::F_3_DY => "f3dy",
            Flow3DProperties::F_3_DZ => "f3dz",
        }
    }
}

impl From<Flow3DProperties> for &'static str {
    #[inline]
    fn from(properties: Flow3DProperties) -> &'static str {
        match properties {
            Flow3DProperties::F_3_DD => "f3dd",
            Flow3DProperties::F_3_DH => "f3dh",
            Flow3DProperties::F_3_DW => "f3dw",
            Flow3DProperties::F_3_DX => "f3dx",
            Flow3DProperties::F_3_DY => "f3dy",
            Flow3DProperties::F_3_DZ => "f3dz",
        }
    }
}

impl From<Flow3DProperties> for String {
    #[inline]
    fn from(properties: Flow3DProperties) -> String {
        match properties {
            Flow3DProperties::F_3_DD => "f3dd".to_owned(),
            Flow3DProperties::F_3_DH => "f3dh".to_owned(),
            Flow3DProperties::F_3_DW => "f3dw".to_owned(),
            Flow3DProperties::F_3_DX => "f3dx".to_owned(),
            Flow3DProperties::F_3_DY => "f3dy".to_owned(),
            Flow3DProperties::F_3_DZ => "f3dz".to_owned(),
        }
    }
}

impl From<Flow3DProperties> for reactive_graph_graph::PropertyType {
    #[inline]
    fn from(properties: Flow3DProperties) -> reactive_graph_graph::PropertyType {
        match properties {
            Flow3DProperties::F_3_DD => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "f3dd",
                    "The depth (z-axis) in a 3D flow",
                    reactive_graph_graph::DataType::Number,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            Flow3DProperties::F_3_DH => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "f3dh",
                    "The height (y-axis) in a 3D flow",
                    reactive_graph_graph::DataType::Number,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            Flow3DProperties::F_3_DW => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "f3dw",
                    "The width (x-axis) in a 3D flow",
                    reactive_graph_graph::DataType::Number,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            Flow3DProperties::F_3_DX => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "f3dx",
                    "The X position in a 3D flow",
                    reactive_graph_graph::DataType::Number,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            Flow3DProperties::F_3_DY => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "f3dy",
                    "The Y position in a 3D flow",
                    reactive_graph_graph::DataType::Number,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            Flow3DProperties::F_3_DZ => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "f3dz",
                    "The Z position in a 3D flow",
                    reactive_graph_graph::DataType::Number,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
        }
    }
}

pub struct Flow3DPropertiesIterator(Option<Flow3DProperties>);

impl Flow3DProperties {
    pub fn into_iter() -> Flow3DPropertiesIterator {
        Flow3DPropertiesIterator(None)
    }
}

impl Iterator for Flow3DPropertiesIterator {
    type Item = Flow3DProperties;
    fn next(&mut self) -> Option<Self::Item> {
        self.0 = match self.0 {
            None => Some(Flow3DProperties::F_3_DD),
            Some(Flow3DProperties::F_3_DD) => Some(Flow3DProperties::F_3_DH),
            Some(Flow3DProperties::F_3_DH) => Some(Flow3DProperties::F_3_DW),
            Some(Flow3DProperties::F_3_DW) => Some(Flow3DProperties::F_3_DX),
            Some(Flow3DProperties::F_3_DX) => Some(Flow3DProperties::F_3_DY),
            Some(Flow3DProperties::F_3_DY) => Some(Flow3DProperties::F_3_DZ),
            Some(Flow3DProperties::F_3_DZ) => None,
        };
        self.0.clone()
    }
}

impl IntoIterator for Flow3DProperties {
    type Item = Flow3DProperties;
    type IntoIter = Flow3DPropertiesIterator;
    fn into_iter(self) -> Self::IntoIter {
        Flow3DPropertiesIterator(None)
    }
}

impl core::fmt::Display for Flow3DProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match *self {
            Flow3DProperties::F_3_DD => core::fmt::Display::fmt("f3dd", f),
            Flow3DProperties::F_3_DH => core::fmt::Display::fmt("f3dh", f),
            Flow3DProperties::F_3_DW => core::fmt::Display::fmt("f3dw", f),
            Flow3DProperties::F_3_DX => core::fmt::Display::fmt("f3dx", f),
            Flow3DProperties::F_3_DY => core::fmt::Display::fmt("f3dy", f),
            Flow3DProperties::F_3_DZ => core::fmt::Display::fmt("f3dz", f),
        }
    }
}

///
pub static FLOW_3_D_EXTENSIONS: std::sync::LazyLock<reactive_graph_graph::Extensions> = std::sync::LazyLock::new(||
{ reactive_graph_graph::Extensions::new() });

pub static FLOW_3_D_TYPE: std::sync::LazyLock<reactive_graph_graph::Component> = std::sync::LazyLock::new(||
{
    reactive_graph_graph::Component::builder()
        .ty(core::ops::Deref::deref(&FLOW_3_D))
        .description(
            "The position (x,y,z) of the entity or relation on a three dimensional flow (in-game visual scripting).",
        )
        .properties(Flow3DProperties::property_types())
        .build()
});

/// # Component `Flow3D`
///
/// ## Fully Qualified Namespace
///
/// `reactive_graph::flow::Flow3D`
///
/// ## Description
///
/// The position (x,y,z) of the entity or relation on a three dimensional flow (in-game visual
/// scripting).
///
/// ### Properties
///
/// - f3dd
/// - f3dh
/// - f3dw
/// - f3dx
/// - f3dy
/// - f3dz
///
/// ## JSON Schema
///
/// [https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/flow/Flow3D.schema.json]()
///
pub trait Flow3D {
    
    /// ### Property `f3dd`
    ///
    /// The depth (z-axis) in a 3D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn f3dd(&self) -> u64;
    
    /// ### Property `f3dd`
    ///
    /// The depth (z-axis) in a 3D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn set_f3dd(&mut self, f3dd: u64);
    
    /// ### Property `f3dh`
    ///
    /// The height (y-axis) in a 3D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn f3dh(&self) -> u64;
    
    /// ### Property `f3dh`
    ///
    /// The height (y-axis) in a 3D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn set_f3dh(&mut self, f3dh: u64);
    
    /// ### Property `f3dw`
    ///
    /// The width (x-axis) in a 3D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn f3dw(&self) -> u64;
    
    /// ### Property `f3dw`
    ///
    /// The width (x-axis) in a 3D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn set_f3dw(&mut self, f3dw: u64);
    
    /// ### Property `f3dx`
    ///
    /// The X position in a 3D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn f3dx(&self) -> u64;
    
    /// ### Property `f3dx`
    ///
    /// The X position in a 3D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn set_f3dx(&mut self, f3dx: u64);
    
    /// ### Property `f3dy`
    ///
    /// The Y position in a 3D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn f3dy(&self) -> u64;
    
    /// ### Property `f3dy`
    ///
    /// The Y position in a 3D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn set_f3dy(&mut self, f3dy: u64);
    
    /// ### Property `f3dz`
    ///
    /// The Z position in a 3D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn f3dz(&self) -> u64;
    
    /// ### Property `f3dz`
    ///
    /// The Z position in a 3D flow
    ///
    /// Data Type: `Number`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    fn set_f3dz(&mut self, f3dz: u64);
}
