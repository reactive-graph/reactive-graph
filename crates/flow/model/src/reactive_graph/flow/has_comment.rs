//! ---------------------------------------------
//! This file was generated automatically.
//! ---------------------------------------------
#![allow(dead_code, unused)]
#![cfg_attr(rustfmt, rustfmt_skip)]

/// Namespace `reactive_graph::flow::HasComment`
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const HAS_COMMENT_NAMESPACE: &str = "reactive_graph::flow::HasComment";

/// The [type identifier]() of RelationType `HasComment`.
///
/// ### Namespace
///
/// The fully qualified namespace is
/// `reactive_graph::flow::HasComment`
pub static HAS_COMMENT: std::sync::LazyLock<reactive_graph_graph::RelationTypeId> = std::sync::LazyLock::new(||
std::str::FromStr::from_str(HAS_COMMENT_NAMESPACE).unwrap());

/// The properties of RelationType `HasComment`.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone)]
pub enum HasCommentProperties {}
impl HasCommentProperties {
    pub fn len() -> usize {
        0usize
    }
    
    pub fn property_types() -> reactive_graph_graph::PropertyTypes {
        let property_types = reactive_graph_graph::PropertyTypes::new();
        property_types
    }
}

impl AsRef<str> for HasCommentProperties {
    #[inline]
    fn as_ref(&self) -> &str {
        match *self {}
    }
}

impl From<HasCommentProperties> for &'static str {
    #[inline]
    fn from(properties: HasCommentProperties) -> &'static str {
        match properties {}
    }
}

impl From<HasCommentProperties> for String {
    #[inline]
    fn from(properties: HasCommentProperties) -> String {
        match properties {}
    }
}

impl From<HasCommentProperties> for reactive_graph_graph::PropertyType {
    #[inline]
    fn from(properties: HasCommentProperties) -> reactive_graph_graph::PropertyType {
        match properties {}
    }
}

pub struct HasCommentPropertiesIterator(Option<HasCommentProperties>);

impl HasCommentProperties {
    pub fn into_iter() -> HasCommentPropertiesIterator {
        HasCommentPropertiesIterator(None)
    }
}

impl Iterator for HasCommentPropertiesIterator {
    type Item = HasCommentProperties;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl IntoIterator for HasCommentProperties {
    type Item = HasCommentProperties;
    type IntoIter = HasCommentPropertiesIterator;
    fn into_iter(self) -> Self::IntoIter {
        HasCommentPropertiesIterator(None)
    }
}

impl core::fmt::Display for HasCommentProperties {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match *self {}
    }
}

///
pub static HAS_COMMENT_COMPONENTS: std::sync::LazyLock<
    reactive_graph_graph::ComponentTypeIds,
> = std::sync::LazyLock::new(|| { reactive_graph_graph::ComponentTypeIds::new() });

///
pub static HAS_COMMENT_EXTENSIONS: std::sync::LazyLock<
    reactive_graph_graph::Extensions,
> = std::sync::LazyLock::new(|| { reactive_graph_graph::Extensions::new() });

pub static HAS_COMMENT_TYPE: std::sync::LazyLock<reactive_graph_graph::RelationType> = std::sync::LazyLock::new(||
{
    reactive_graph_graph::RelationType::builder()
        .outbound_type(
            reactive_graph_graph::InboundOutboundType::EntityType(
                reactive_graph_graph::MatchingInboundOutboundType::Any,
            ),
        )
        .ty(core::ops::Deref::deref(&HAS_COMMENT))
        .inbound_type(
            reactive_graph_graph::InboundOutboundType::EntityType(
                reactive_graph_graph::MatchingInboundOutboundType::NamespacedType(
                    std::ops::Deref::deref(
                            &crate::reactive_graph::flow::comment::COMMENT,
                        )
                        .clone(),
                ),
            ),
        )
        .description("Any entity within a flow can be commented")
        .components(HAS_COMMENT_COMPONENTS.clone())
        .properties(HasCommentProperties::property_types())
        .extensions(HAS_COMMENT_EXTENSIONS.clone())
        .build()
});

/// # RelationType `HasComment`
///
/// ## Fully Qualified Namespace
///
/// `reactive_graph::flow::HasComment`
///
/// ## Description
///
/// Any entity within a flow can be commented
///
/// ## Outbound Entity
///
/// `*`
///
/// ## Inbound Entity
///
/// `reactive_graph::flow::Comment`
///
/// ## JSON Schema
///
/// [https://schema.reactive-graph.io/schema/json/dynamic_graph/types/relation/reactive_graph/flow/HasComment.schema.json]()
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
pub struct HasComment {
    #[builder(setter(into))]
    outbound_id: uuid::Uuid,
    #[builder(setter(into))]
    id: reactive_graph_graph::RelationInstanceTypeId,
    #[builder(setter(into))]
    inbound_id: uuid::Uuid,
    extensions: reactive_graph_graph::Extensions,
}

impl HasComment {
    pub fn new(
        outbound_id: uuid::Uuid,
        id: reactive_graph_graph::InstanceId,
        inbound_id: uuid::Uuid,
    ) -> Self {
        let id = reactive_graph_graph::RelationInstanceTypeId::new(
            std::ops::Deref::deref(&HAS_COMMENT),
            id,
        );
        Self {
            outbound_id,
            id,
            inbound_id,
            extensions: reactive_graph_graph::Extensions::new(),
        }
    }
    
    pub fn new_with_extensions(
        outbound_id: uuid::Uuid,
        id: reactive_graph_graph::InstanceId,
        inbound_id: uuid::Uuid,
        extensions: reactive_graph_graph::Extensions,
    ) -> Self {
        let id = reactive_graph_graph::RelationInstanceTypeId::new(
            std::ops::Deref::deref(&HAS_COMMENT),
            id,
        );
        Self {
            outbound_id,
            id,
            inbound_id,
            extensions,
        }
    }
    
    pub fn outbound_id(&self) -> uuid::Uuid {
        self.outbound_id
    }
    
    pub fn id(&self) -> reactive_graph_graph::RelationInstanceTypeId {
        self.id.clone()
    }
    
    pub fn inbound_id(&self) -> uuid::Uuid {
        self.inbound_id
    }
    
    pub fn properties(&self) -> reactive_graph_graph::PropertyInstances {
        reactive_graph_graph::PropertyInstances::new()
    }
    
    pub fn extensions(&self) -> reactive_graph_graph::Extensions {
        self.extensions.clone()
    }
}

impl From<HasComment> for reactive_graph_graph::RelationInstance {
    fn from(has_comment: HasComment) -> Self {
        reactive_graph_graph::RelationInstance::builder()
            .outbound_id(has_comment.outbound_id())
            .ty(has_comment.id())
            .inbound_id(has_comment.inbound_id())
            .components(HAS_COMMENT_COMPONENTS.clone())
            .properties(has_comment.properties())
            .build()
    }
}

impl TryFrom<reactive_graph_graph::RelationInstance> for HasComment {
    type Error = ();
    fn try_from(
        relation_instance: reactive_graph_graph::RelationInstance,
    ) -> Result<Self, Self::Error> {
        Err(())
    }
}
