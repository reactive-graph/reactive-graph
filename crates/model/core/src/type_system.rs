//! ---------------------------------------------
//! This file was generated automatically.
//! ---------------------------------------------
#![allow(dead_code, unused)]
#![cfg_attr(rustfmt, rustfmt_skip)]

pub static TYPE_SYSTEM_COMPONENTS: std::sync::LazyLock<
    reactive_graph_graph::Components,
> = std::sync::LazyLock::new(|| {
    reactive_graph_graph::Components::new()
        .component(crate::reactive_graph::core::action::ACTION_TYPE.clone())
        .component(crate::reactive_graph::core::event::EVENT_TYPE.clone())
        .component(crate::reactive_graph::core::labeled::LABELED_TYPE.clone())
});

pub static TYPE_SYSTEM_ENTITY_TYPES: std::sync::LazyLock<
    reactive_graph_graph::EntityTypes,
> = std::sync::LazyLock::new(|| { reactive_graph_graph::EntityTypes::new() });

pub static TYPE_SYSTEM_RELATION_TYPES: std::sync::LazyLock<
    reactive_graph_graph::RelationTypes,
> = std::sync::LazyLock::new(|| { reactive_graph_graph::RelationTypes::new() });

pub static TYPE_SYSTEM_FLOW_TYPES: std::sync::LazyLock<
    reactive_graph_graph::FlowTypes,
> = std::sync::LazyLock::new(|| { reactive_graph_graph::FlowTypes::new() });

pub static TYPE_SYSTEM: std::sync::LazyLock<reactive_graph_graph::TypeSystem> = std::sync::LazyLock::new(||
{
    reactive_graph_graph::TypeSystem::builder()
        .components(TYPE_SYSTEM_COMPONENTS.clone())
        .entity_types(TYPE_SYSTEM_ENTITY_TYPES.clone())
        .relation_types(TYPE_SYSTEM_RELATION_TYPES.clone())
        .flow_types(TYPE_SYSTEM_FLOW_TYPES.clone())
        .build()
});
pub static TYPE_SYSTEM_ID: &str = "reactive_graph::core";
pub static TYPE_SYSTEM_NAMESPACE: std::sync::LazyLock<reactive_graph_graph::Namespace> = std::sync::LazyLock::new(||
{ std::str::FromStr::from_str(TYPE_SYSTEM_ID).unwrap() });
