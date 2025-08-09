use reactive_graph_graph::EntityTypeId;
use std::sync::LazyLock;

pub static ENTITY_TYPE_GENERIC_FLOW: LazyLock<EntityTypeId> = LazyLock::new(|| EntityTypeId::try_from("reactive_graph::flow::GenericFlow").unwrap());
