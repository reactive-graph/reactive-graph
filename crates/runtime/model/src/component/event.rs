use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::properties;
use std::sync::LazyLock;

properties!(EventProperties, (EVENT, "event", ""));

pub static COMPONENT_EVENT: LazyLock<ComponentTypeId> = LazyLock::new(|| ComponentTypeId::try_from("reactive_graph::core::Event").unwrap());
