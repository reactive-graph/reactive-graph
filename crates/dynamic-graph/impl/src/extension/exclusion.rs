use serde_json::json;

use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::ExtensionContainer;

pub const EXCLUDE: &str = "exclude";

pub fn is_excluded(extension_container: &impl ExtensionContainer, component_ty: &ComponentTypeId) -> bool {
    false
}
