use reactive_graph_graph::EntityTypeId;
use std::sync::LazyLock;

pub static ENTITY_TYPE_SYSTEM_EVENT: LazyLock<EntityTypeId> = LazyLock::new(|| EntityTypeId::try_from("reactive_graph::core::SystemEvent").unwrap());
