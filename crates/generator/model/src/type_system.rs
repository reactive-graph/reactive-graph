#[doc(newline)]
pub static TYPE_SYSTEM_COMPONENTS: std::sync::LazyLock<reactive_graph_graph::Components> = std::sync::LazyLock::new(|| reactive_graph_graph::Components::new());
#[doc(newline)]
pub static TYPE_SYSTEM_ENTITY_TYPES: std::sync::LazyLock<reactive_graph_graph::EntityTypes> =
    std::sync::LazyLock::new(|| reactive_graph_graph::EntityTypes::new());
#[doc(newline)]
pub static TYPE_SYSTEM_RELATION_TYPES: std::sync::LazyLock<reactive_graph_graph::RelationTypes> =
    std::sync::LazyLock::new(|| reactive_graph_graph::RelationTypes::new());
#[doc(newline)]
pub static TYPE_SYSTEM_FLOW_TYPES: std::sync::LazyLock<reactive_graph_graph::FlowTypes> = std::sync::LazyLock::new(|| reactive_graph_graph::FlowTypes::new());
#[doc(newline)]
pub static TYPE_SYSTEM: std::sync::LazyLock<reactive_graph_graph::TypeSystem> = std::sync::LazyLock::new(|| {
    reactive_graph_graph::TypeSystem::builder()
        .components(TYPE_SYSTEM_COMPONENTS.clone())
        .entity_types(TYPE_SYSTEM_ENTITY_TYPES.clone())
        .relation_types(TYPE_SYSTEM_RELATION_TYPES.clone())
        .flow_types(TYPE_SYSTEM_FLOW_TYPES.clone())
        .build()
});
pub static TYPE_SYSTEM_ID: &str = "reactive_graph::generator";
