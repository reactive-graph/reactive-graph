use reactive_graph_graph::ExtensionTypeId;
use std::str::FromStr;
use std::sync::LazyLock;

pub static EXTENSION_FLOW_UUID_TYPE_EXTENSION: LazyLock<ExtensionTypeId> =
    LazyLock::new(|| ExtensionTypeId::from_str("reactive_graph::flow::UuidTypeExtension").unwrap());
