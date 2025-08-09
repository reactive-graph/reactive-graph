use reactive_graph_graph::EntityTypeId;
use std::sync::LazyLock;

pub static ENTITY_TYPE_COMMAND_NUM_COMMANDS: LazyLock<EntityTypeId> = LazyLock::new(|| EntityTypeId::try_from("reactive_graph::command::NumCommands").unwrap());
