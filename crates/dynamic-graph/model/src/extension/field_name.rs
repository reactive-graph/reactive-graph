use reactive_graph_graph::ExtensionTypeId;
use std::str::FromStr;
use std::sync::LazyLock;

pub static EXTENSION_DYNAMIC_GRAPH_FIELD_NAME: LazyLock<ExtensionTypeId> =
    LazyLock::new(|| ExtensionTypeId::from_str("reactive_graph::dynamic_graph::FieldName").unwrap());
