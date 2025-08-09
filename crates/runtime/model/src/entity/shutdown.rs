use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::properties;
use std::sync::LazyLock;

properties!(ShutdownProperties, (DELAY, "delay", 0));

pub static ENTITY_TYPE_SHUTDOWN: LazyLock<EntityTypeId> = LazyLock::new(|| EntityTypeId::try_from("reactive_graph::core::Shutdown").unwrap());
