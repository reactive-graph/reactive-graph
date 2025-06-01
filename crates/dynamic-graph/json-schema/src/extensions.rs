use reactive_graph_graph::ExtensionTypeId;
use std::sync::LazyLock;

pub static EXTENSION_JSON_SCHEMA_FORMAT: LazyLock<ExtensionTypeId> = LazyLock::new(|| ExtensionTypeId::new_from_type("json_schema", "format"));
