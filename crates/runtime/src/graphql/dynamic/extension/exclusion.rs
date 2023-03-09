use serde_json::json;

use crate::model::ComponentTypeId;
use crate::model::ExtensionContainer;

pub const EXCLUDE: &str = "exclude";

pub fn is_excluded(extension_container: &impl ExtensionContainer, component_ty: &ComponentTypeId) -> bool {
    false
}
