use reactive_graph_graph::ExtensionTypeId;
use std::sync::LazyLock;

pub static EXTENSION_DYNAMIC_GRAPH_FIELD_DESCRIPTION: LazyLock<ExtensionTypeId> =
    LazyLock::new(|| ExtensionTypeId::try_from("reactive_graph::dynamic_graph::FieldDescription").unwrap());
