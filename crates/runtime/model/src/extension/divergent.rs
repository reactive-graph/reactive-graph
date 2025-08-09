use reactive_graph_graph::ExtensionTypeId;
use std::sync::LazyLock;

pub static EXTENSION_DIVERGENT: LazyLock<ExtensionTypeId> = LazyLock::new(|| ExtensionTypeId::try_from("reactive_graph::core::Divergent").unwrap());
