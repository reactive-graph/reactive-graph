use reactive_graph_graph::ExtensionTypeId;
use std::str::FromStr;
use std::sync::LazyLock;

pub static EXTENSION_DIVERGENT: LazyLock<ExtensionTypeId> = LazyLock::new(|| ExtensionTypeId::from_str("reactive_graph::core::Divergent").unwrap());
