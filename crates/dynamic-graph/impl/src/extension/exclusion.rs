use serde_json::json;

use inexor_rgf_graph::ComponentTypeId;
use inexor_rgf_graph::ExtensionContainer;

pub const EXCLUDE: &str = "exclude";

pub fn is_excluded(extension_container: &impl ExtensionContainer, component_ty: &ComponentTypeId) -> bool {
    false
}
