use serde_json::json;

use crate::model::ComponentTypeId;
use crate::model::ExtensionContainer;
use crate::model_runtime::EXTENSION_DIVERGENT;

pub fn is_divergent(extension_container: &impl ExtensionContainer, component_ty: &ComponentTypeId) -> bool {
    match extension_container.get_own_extension(&EXTENSION_DIVERGENT.clone()) {
        Some(divergent) => {
            let component_ty_s = json!(component_ty.to_string());
            divergent.extension.as_array().map(|d| d.contains(&component_ty_s)).unwrap_or(false)
        }
        None => false,
    }
}
