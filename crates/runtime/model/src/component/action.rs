use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::properties;
use std::sync::LazyLock;

properties!(ActionProperties, (TRIGGER, "trigger", false));

pub static COMPONENT_ACTION: LazyLock<ComponentTypeId> = LazyLock::new(|| ComponentTypeId::try_from("reactive_graph::core::Action").unwrap());
