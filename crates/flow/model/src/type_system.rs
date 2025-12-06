pub static TYPE_SYSTEM_COMPONENTS: std::sync::LazyLock<reactive_graph_graph::Components> = std::sync::LazyLock::new(|| {
    reactive_graph_graph::Components::new()
        .component(crate::reactive_graph::flow::flow_2_d::FLOW_2_D_TYPE.clone())
        .component(crate::reactive_graph::flow::flow_3_d::FLOW_3_D_TYPE.clone())
});

pub static TYPE_SYSTEM_ENTITY_TYPES: std::sync::LazyLock<reactive_graph_graph::EntityTypes> = std::sync::LazyLock::new(|| {
    reactive_graph_graph::EntityTypes::new()
        .entity(crate::reactive_graph::flow::comment::COMMENT_TYPE.clone())
        .entity(crate::reactive_graph::flow::simple_flow::SIMPLE_FLOW_TYPE.clone())
});

pub static TYPE_SYSTEM_RELATION_TYPES: std::sync::LazyLock<reactive_graph_graph::RelationTypes> =
    std::sync::LazyLock::new(|| reactive_graph_graph::RelationTypes::new().relation(crate::reactive_graph::flow::has_comment::HAS_COMMENT_TYPE.clone()));

pub static TYPE_SYSTEM_FLOW_TYPES: std::sync::LazyLock<reactive_graph_graph::FlowTypes> = std::sync::LazyLock::new(|| reactive_graph_graph::FlowTypes::new());

pub static TYPE_SYSTEM: std::sync::LazyLock<reactive_graph_graph::TypeSystem> = std::sync::LazyLock::new(|| {
    reactive_graph_graph::TypeSystem::builder()
        .components(TYPE_SYSTEM_COMPONENTS.clone())
        .entity_types(TYPE_SYSTEM_ENTITY_TYPES.clone())
        .relation_types(TYPE_SYSTEM_RELATION_TYPES.clone())
        .flow_types(TYPE_SYSTEM_FLOW_TYPES.clone())
        .build()
});
pub static TYPE_SYSTEM_ID: &str = "reactive_graph::flow";
pub static TYPE_SYSTEM_NAMESPACE: std::sync::LazyLock<reactive_graph_graph::Namespace> =
    std::sync::LazyLock::new(|| std::str::FromStr::from_str(TYPE_SYSTEM_ID).unwrap());
