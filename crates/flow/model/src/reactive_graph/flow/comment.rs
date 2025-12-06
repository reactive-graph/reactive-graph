//! ---------------------------------------------
//! This file was generated automatically.
//! ---------------------------------------------
#![allow(dead_code, unused)]
#![cfg_attr(rustfmt, rustfmt_skip)]

/// Namespace `reactive_graph::flow::Comment`
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const COMMENT_NAMESPACE: &str = "reactive_graph::flow::Comment";

/// The [type identifier]() of EntityType `Comment`.
///
/// ### Namespace
///
/// The fully qualified namespace is
/// `reactive_graph::flow::Comment`
pub static COMMENT: std::sync::LazyLock<reactive_graph_graph::EntityTypeId> = std::sync::LazyLock::new(||
std::str::FromStr::from_str(COMMENT_NAMESPACE).unwrap());

/// The properties of EntityType `Comment`.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
pub enum CommentProperties {
    
    /// ### Property `comment`
    ///
    /// Comment
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    COMMENT,
    
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
impl CommentProperties {
    pub fn len() -> usize {
        11usize
    }
    
    pub fn property_types() -> reactive_graph_graph::PropertyTypes {
        let property_types = reactive_graph_graph::PropertyTypes::new();
        property_types.push(CommentProperties::COMMENT);
        property_types.push(CommentProperties::F_2_DH);
        property_types.push(CommentProperties::F_2_DW);
        property_types.push(CommentProperties::F_2_DX);
        property_types.push(CommentProperties::F_2_DY);
        property_types.push(CommentProperties::F_3_DD);
        property_types.push(CommentProperties::F_3_DH);
        property_types.push(CommentProperties::F_3_DW);
        property_types.push(CommentProperties::F_3_DX);
        property_types.push(CommentProperties::F_3_DY);
        property_types.push(CommentProperties::F_3_DZ);
        property_types
    }
}

impl AsRef<str> for CommentProperties {
    #[inline]
    fn as_ref(&self) -> &str {
        match *self {
            CommentProperties::COMMENT => "comment",
            CommentProperties::F_2_DH => "f2dh",
            CommentProperties::F_2_DW => "f2dw",
            CommentProperties::F_2_DX => "f2dx",
            CommentProperties::F_2_DY => "f2dy",
            CommentProperties::F_3_DD => "f3dd",
            CommentProperties::F_3_DH => "f3dh",
            CommentProperties::F_3_DW => "f3dw",
            CommentProperties::F_3_DX => "f3dx",
            CommentProperties::F_3_DY => "f3dy",
            CommentProperties::F_3_DZ => "f3dz",
        }
    }
}

impl From<CommentProperties> for &'static str {
    #[inline]
    fn from(properties: CommentProperties) -> &'static str {
        match properties {
            CommentProperties::COMMENT => "comment",
            CommentProperties::F_2_DH => "f2dh",
            CommentProperties::F_2_DW => "f2dw",
            CommentProperties::F_2_DX => "f2dx",
            CommentProperties::F_2_DY => "f2dy",
            CommentProperties::F_3_DD => "f3dd",
            CommentProperties::F_3_DH => "f3dh",
            CommentProperties::F_3_DW => "f3dw",
            CommentProperties::F_3_DX => "f3dx",
            CommentProperties::F_3_DY => "f3dy",
            CommentProperties::F_3_DZ => "f3dz",
        }
    }
}

impl From<CommentProperties> for String {
    #[inline]
    fn from(properties: CommentProperties) -> String {
        match properties {
            CommentProperties::COMMENT => "comment".to_owned(),
            CommentProperties::F_2_DH => "f2dh".to_owned(),
            CommentProperties::F_2_DW => "f2dw".to_owned(),
            CommentProperties::F_2_DX => "f2dx".to_owned(),
            CommentProperties::F_2_DY => "f2dy".to_owned(),
            CommentProperties::F_3_DD => "f3dd".to_owned(),
            CommentProperties::F_3_DH => "f3dh".to_owned(),
            CommentProperties::F_3_DW => "f3dw".to_owned(),
            CommentProperties::F_3_DX => "f3dx".to_owned(),
            CommentProperties::F_3_DY => "f3dy".to_owned(),
            CommentProperties::F_3_DZ => "f3dz".to_owned(),
        }
    }
}

impl From<CommentProperties> for reactive_graph_graph::PropertyType {
    #[inline]
    fn from(properties: CommentProperties) -> reactive_graph_graph::PropertyType {
        match properties {
            CommentProperties::COMMENT => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "comment",
                    "Comment",
                    reactive_graph_graph::DataType::String,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            CommentProperties::F_2_DH => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "f2dh",
                    "The height (y-axis) in a 2D flow",
                    reactive_graph_graph::DataType::Number,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            CommentProperties::F_2_DW => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "f2dw",
                    "The width (x-axis) in a 2D flow",
                    reactive_graph_graph::DataType::Number,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            CommentProperties::F_2_DX => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "f2dx",
                    "The X position in a 2D flow",
                    reactive_graph_graph::DataType::Number,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            CommentProperties::F_2_DY => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "f2dy",
                    "The Y position in a 2D flow",
                    reactive_graph_graph::DataType::Number,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            CommentProperties::F_3_DD => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "f3dd",
                    "The depth (z-axis) in a 3D flow",
                    reactive_graph_graph::DataType::Number,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            CommentProperties::F_3_DH => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "f3dh",
                    "The height (y-axis) in a 3D flow",
                    reactive_graph_graph::DataType::Number,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            CommentProperties::F_3_DW => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "f3dw",
                    "The width (x-axis) in a 3D flow",
                    reactive_graph_graph::DataType::Number,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            CommentProperties::F_3_DX => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "f3dx",
                    "The X position in a 3D flow",
                    reactive_graph_graph::DataType::Number,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            CommentProperties::F_3_DY => {
                reactive_graph_graph::PropertyType::new_with_all(
                    "f3dy",
                    "The Y position in a 3D flow",
                    reactive_graph_graph::DataType::Number,
                    reactive_graph_graph::SocketType::None,
                    reactive_graph_graph::Mutability::Mutable,
                    reactive_graph_graph::Extensions::new(),
                )
            }
            CommentProperties::F_3_DZ => {
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

pub struct CommentPropertiesIterator(Option<CommentProperties>);

impl CommentProperties {
    pub fn into_iter() -> CommentPropertiesIterator {
        CommentPropertiesIterator(None)
    }
}

impl Iterator for CommentPropertiesIterator {
    type Item = CommentProperties;
    fn next(&mut self) -> Option<Self::Item> {
        self.0 = match self.0 {
            None => Some(CommentProperties::COMMENT),
            Some(CommentProperties::COMMENT) => Some(CommentProperties::F_2_DH),
            Some(CommentProperties::F_2_DH) => Some(CommentProperties::F_2_DW),
            Some(CommentProperties::F_2_DW) => Some(CommentProperties::F_2_DX),
            Some(CommentProperties::F_2_DX) => Some(CommentProperties::F_2_DY),
            Some(CommentProperties::F_2_DY) => Some(CommentProperties::F_3_DD),
            Some(CommentProperties::F_3_DD) => Some(CommentProperties::F_3_DH),
            Some(CommentProperties::F_3_DH) => Some(CommentProperties::F_3_DW),
            Some(CommentProperties::F_3_DW) => Some(CommentProperties::F_3_DX),
            Some(CommentProperties::F_3_DX) => Some(CommentProperties::F_3_DY),
            Some(CommentProperties::F_3_DY) => Some(CommentProperties::F_3_DZ),
            Some(CommentProperties::F_3_DZ) => None,
        };
        self.0.clone()
    }
}

impl IntoIterator for CommentProperties {
    type Item = CommentProperties;
    type IntoIter = CommentPropertiesIterator;
    fn into_iter(self) -> Self::IntoIter {
        CommentPropertiesIterator(None)
    }
}

impl core::fmt::Display for CommentProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match *self {
            CommentProperties::COMMENT => core::fmt::Display::fmt("comment", f),
            CommentProperties::F_2_DH => core::fmt::Display::fmt("f2dh", f),
            CommentProperties::F_2_DW => core::fmt::Display::fmt("f2dw", f),
            CommentProperties::F_2_DX => core::fmt::Display::fmt("f2dx", f),
            CommentProperties::F_2_DY => core::fmt::Display::fmt("f2dy", f),
            CommentProperties::F_3_DD => core::fmt::Display::fmt("f3dd", f),
            CommentProperties::F_3_DH => core::fmt::Display::fmt("f3dh", f),
            CommentProperties::F_3_DW => core::fmt::Display::fmt("f3dw", f),
            CommentProperties::F_3_DX => core::fmt::Display::fmt("f3dx", f),
            CommentProperties::F_3_DY => core::fmt::Display::fmt("f3dy", f),
            CommentProperties::F_3_DZ => core::fmt::Display::fmt("f3dz", f),
        }
    }
}

/// ## Components
///
/// | Component                      | Description                                                                                            | Properties                                                                                                  |
/// |--------------------------------|--------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------|
/// | `reactive_graph::flow::Flow2D` | The position (x,y) of the entity or relation on a two dimensional flow.                                | <ul compact><li>`f2dh`</li><li>`f2dw`</li><li>`f2dx`</li><li>`f2dy`</li></ul>                               |
/// | `reactive_graph::flow::Flow3D` | The position (x,y,z) of the entity or relation on a three dimensional flow (in-game visual scripting). | <ul compact><li>`f3dd`</li><li>`f3dh`</li><li>`f3dw`</li><li>`f3dx`</li><li>`f3dy`</li><li>`f3dz`</li></ul> |
///
pub static COMMENT_COMPONENTS: std::sync::LazyLock<
    reactive_graph_graph::ComponentTypeIds,
> = std::sync::LazyLock::new(|| {
    reactive_graph_graph::ComponentTypeIds::new()
        .component(
            std::ops::Deref::deref(&crate::reactive_graph::flow::flow_2_d::FLOW_2_D),
        )
        .component(
            std::ops::Deref::deref(&crate::reactive_graph::flow::flow_3_d::FLOW_3_D),
        )
});

///
pub static COMMENT_EXTENSIONS: std::sync::LazyLock<reactive_graph_graph::Extensions> = std::sync::LazyLock::new(||
{ reactive_graph_graph::Extensions::new() });

pub static COMMENT_TYPE: std::sync::LazyLock<reactive_graph_graph::EntityType> = std::sync::LazyLock::new(||
{
    reactive_graph_graph::EntityType::builder()
        .ty(core::ops::Deref::deref(&COMMENT))
        .description("A simple comment")
        .components(COMMENT_COMPONENTS.clone())
        .properties(CommentProperties::property_types())
        .extensions(COMMENT_EXTENSIONS.clone())
        .build()
});

/// # EntityType `Comment`
///
/// ## Fully Qualified Namespace
///
/// `reactive_graph::flow::Comment`
///
/// ## Description
///
/// A simple comment
///
/// ## Components
///
/// | Component                      | Description                                                                                            | Properties                                                                                                  |
/// |--------------------------------|--------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------|
/// | `reactive_graph::flow::Flow3D` | The position (x,y,z) of the entity or relation on a three dimensional flow (in-game visual scripting). | <ul compact><li>`f3dw`</li><li>`f3dz`</li><li>`f3dx`</li><li>`f3dd`</li><li>`f3dh`</li><li>`f3dy`</li></ul> |
/// | `reactive_graph::flow::Flow2D` | The position (x,y) of the entity or relation on a two dimensional flow.                                | <ul compact><li>`f2dh`</li><li>`f2dw`</li><li>`f2dx`</li><li>`f2dy`</li></ul>                               |
///
/// ### Properties
///
/// - comment
///
/// ### Properties from components
///
/// - f2dh
/// - f2dw
/// - f2dx
/// - f2dy
/// - f3dd
/// - f3dh
/// - f3dw
/// - f3dx
/// - f3dy
/// - f3dz
///
/// ## JSON Schema
///
/// [https://schema.reactive-graph.io/schema/json/dynamic_graph/types/entity/reactive_graph/flow/Comment.schema.json]()
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
pub struct Comment {
    #[builder(default, setter(into))]
    pub id: uuid::Uuid,
    
    /// ### Property `comment`
    ///
    /// Comment
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    #[builder(setter(into))]
    pub comment: String,
    
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
    pub f2dh: u64,
    
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
    pub f2dw: u64,
    
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
    pub f2dx: u64,
    
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
    pub f2dy: u64,
    
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
    pub f3dd: u64,
    
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
    pub f3dh: u64,
    
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
    pub f3dw: u64,
    
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
    pub f3dx: u64,
    
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
    pub f3dy: u64,
    
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
    pub f3dz: u64,
    #[builder(default, setter(into))]
    pub extensions: reactive_graph_graph::Extensions,
}

impl Comment {
    pub fn new(
        comment: String,
        f2dh: u64,
        f2dw: u64,
        f2dx: u64,
        f2dy: u64,
        f3dd: u64,
        f3dh: u64,
        f3dw: u64,
        f3dx: u64,
        f3dy: u64,
        f3dz: u64,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            comment,
            f2dh,
            f2dw,
            f2dx,
            f2dy,
            f3dd,
            f3dh,
            f3dw,
            f3dx,
            f3dy,
            f3dz,
            extensions: reactive_graph_graph::Extensions::new(),
        }
    }
    pub fn new_with_id(
        id: uuid::Uuid,
        comment: String,
        f2dh: u64,
        f2dw: u64,
        f2dx: u64,
        f2dy: u64,
        f3dd: u64,
        f3dh: u64,
        f3dw: u64,
        f3dx: u64,
        f3dy: u64,
        f3dz: u64,
    ) -> Self {
        Self {
            id,
            comment,
            f2dh,
            f2dw,
            f2dx,
            f2dy,
            f3dd,
            f3dh,
            f3dw,
            f3dx,
            f3dy,
            f3dz,
            extensions: reactive_graph_graph::Extensions::new(),
        }
    }
    pub fn new_with_extensions(
        id: uuid::Uuid,
        comment: String,
        f2dh: u64,
        f2dw: u64,
        f2dx: u64,
        f2dy: u64,
        f3dd: u64,
        f3dh: u64,
        f3dw: u64,
        f3dx: u64,
        f3dy: u64,
        f3dz: u64,
        extensions: reactive_graph_graph::Extensions,
    ) -> Self {
        Self {
            id,
            comment,
            f2dh,
            f2dw,
            f2dx,
            f2dy,
            f3dd,
            f3dh,
            f3dw,
            f3dx,
            f3dy,
            f3dz,
            extensions,
        }
    }
    
    pub fn id(&self) -> uuid::Uuid {
        self.id
    }
    
    /// ### Property `comment`
    ///
    /// Comment
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    pub fn comment(&self) -> String {
        self.comment.clone()
    }
    
    /// ### Property `comment`
    ///
    /// Comment
    ///
    /// Data Type: `String`
    ///
    /// Socket Type: `None`
    ///
    /// Mutability: `Mutable`
    ///
    pub fn set_comment(&mut self, comment: String) {
        self.comment = comment;
    }
    
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
    pub fn f2dh(&self) -> u64 {
        self.f2dh
    }
    
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
    pub fn set_f2dh(&mut self, f2dh: u64) {
        self.f2dh = f2dh;
    }
    
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
    pub fn f2dw(&self) -> u64 {
        self.f2dw
    }
    
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
    pub fn set_f2dw(&mut self, f2dw: u64) {
        self.f2dw = f2dw;
    }
    
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
    pub fn f2dx(&self) -> u64 {
        self.f2dx
    }
    
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
    pub fn set_f2dx(&mut self, f2dx: u64) {
        self.f2dx = f2dx;
    }
    
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
    pub fn f2dy(&self) -> u64 {
        self.f2dy
    }
    
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
    pub fn set_f2dy(&mut self, f2dy: u64) {
        self.f2dy = f2dy;
    }
    
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
    pub fn f3dd(&self) -> u64 {
        self.f3dd
    }
    
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
    pub fn set_f3dd(&mut self, f3dd: u64) {
        self.f3dd = f3dd;
    }
    
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
    pub fn f3dh(&self) -> u64 {
        self.f3dh
    }
    
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
    pub fn set_f3dh(&mut self, f3dh: u64) {
        self.f3dh = f3dh;
    }
    
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
    pub fn f3dw(&self) -> u64 {
        self.f3dw
    }
    
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
    pub fn set_f3dw(&mut self, f3dw: u64) {
        self.f3dw = f3dw;
    }
    
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
    pub fn f3dx(&self) -> u64 {
        self.f3dx
    }
    
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
    pub fn set_f3dx(&mut self, f3dx: u64) {
        self.f3dx = f3dx;
    }
    
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
    pub fn f3dy(&self) -> u64 {
        self.f3dy
    }
    
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
    pub fn set_f3dy(&mut self, f3dy: u64) {
        self.f3dy = f3dy;
    }
    
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
    pub fn f3dz(&self) -> u64 {
        self.f3dz
    }
    
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
    pub fn set_f3dz(&mut self, f3dz: u64) {
        self.f3dz = f3dz;
    }
    
    pub fn properties(&self) -> reactive_graph_graph::PropertyInstances {
        reactive_graph_graph::PropertyInstances::new()
            .property(CommentProperties::COMMENT, self.comment.clone())
            .property(CommentProperties::F_2_DH, self.f2dh.clone())
            .property(CommentProperties::F_2_DW, self.f2dw.clone())
            .property(CommentProperties::F_2_DX, self.f2dx.clone())
            .property(CommentProperties::F_2_DY, self.f2dy.clone())
            .property(CommentProperties::F_3_DD, self.f3dd.clone())
            .property(CommentProperties::F_3_DH, self.f3dh.clone())
            .property(CommentProperties::F_3_DW, self.f3dw.clone())
            .property(CommentProperties::F_3_DX, self.f3dx.clone())
            .property(CommentProperties::F_3_DY, self.f3dy.clone())
            .property(CommentProperties::F_3_DZ, self.f3dz.clone())
    }
    
    pub fn extensions(&self) -> reactive_graph_graph::Extensions {
        self.extensions.clone()
    }
}

impl From<Comment> for reactive_graph_graph::EntityInstance {
    fn from(comment: Comment) -> Self {
        reactive_graph_graph::EntityInstance::builder()
            .ty(std::ops::Deref::deref(&COMMENT))
            .id(comment.id())
            .components(COMMENT_COMPONENTS.clone())
            .properties(comment.properties())
            .build()
    }
}

impl TryFrom<reactive_graph_graph::EntityInstance> for Comment {
    type Error = ();
    fn try_from(
        entity_instance: reactive_graph_graph::EntityInstance,
    ) -> Result<Self, Self::Error> {
        Err(())
    }
}
impl crate::reactive_graph::flow::flow_3_d::Flow3D for Comment {
    
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
    fn f3dd(&self) -> u64 {
        self.f3dd
    }
    
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
    fn set_f3dd(&mut self, f3dd: u64) {
        self.f3dd = f3dd;
    }
    
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
    fn f3dh(&self) -> u64 {
        self.f3dh
    }
    
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
    fn set_f3dh(&mut self, f3dh: u64) {
        self.f3dh = f3dh;
    }
    
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
    fn f3dw(&self) -> u64 {
        self.f3dw
    }
    
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
    fn set_f3dw(&mut self, f3dw: u64) {
        self.f3dw = f3dw;
    }
    
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
    fn f3dx(&self) -> u64 {
        self.f3dx
    }
    
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
    fn set_f3dx(&mut self, f3dx: u64) {
        self.f3dx = f3dx;
    }
    
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
    fn f3dy(&self) -> u64 {
        self.f3dy
    }
    
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
    fn set_f3dy(&mut self, f3dy: u64) {
        self.f3dy = f3dy;
    }
    
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
    fn f3dz(&self) -> u64 {
        self.f3dz
    }
    
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
    fn set_f3dz(&mut self, f3dz: u64) {
        self.f3dz = f3dz;
    }
}
impl crate::reactive_graph::flow::flow_2_d::Flow2D for Comment {
    
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
    fn f2dh(&self) -> u64 {
        self.f2dh
    }
    
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
    fn set_f2dh(&mut self, f2dh: u64) {
        self.f2dh = f2dh;
    }
    
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
    fn f2dw(&self) -> u64 {
        self.f2dw
    }
    
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
    fn set_f2dw(&mut self, f2dw: u64) {
        self.f2dw = f2dw;
    }
    
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
    fn f2dx(&self) -> u64 {
        self.f2dx
    }
    
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
    fn set_f2dx(&mut self, f2dx: u64) {
        self.f2dx = f2dx;
    }
    
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
    fn f2dy(&self) -> u64 {
        self.f2dy
    }
    
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
    fn set_f2dy(&mut self, f2dy: u64) {
        self.f2dy = f2dy;
    }
}
