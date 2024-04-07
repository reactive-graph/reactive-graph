use serde::Deserialize;
use serde::Serialize;
use serde_json::from_value;

use reactive_graph_dynamic_graph_model::EXTENSION_FIELD_NAME;
use reactive_graph_graph::Extension;
use reactive_graph_graph::ExtensionContainer;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DynamicGraphFieldNameExtension {
    #[serde(default)]
    pub from_outbound_entity_to_relation: Option<String>,

    #[serde(default)]
    pub from_relation_to_outbound_entity: Option<String>,

    #[serde(default)]
    pub from_inbound_entity_to_relation: Option<String>,

    #[serde(default)]
    pub from_relation_to_inbound_entity: Option<String>,

    #[serde(default)]
    pub from_outbound_entity_to_inbound_entity: Option<String>,

    #[serde(default)]
    pub from_inbound_entity_to_outbound_entity: Option<String>,
}

impl From<Extension> for DynamicGraphFieldNameExtension {
    fn from(extension: Extension) -> Self {
        match from_value(extension.extension) {
            Ok(field_names) => field_names,
            Err(_) => Default::default(),
        }
    }
}

pub fn get_dynamic_graph_field_names(extension_container: &impl ExtensionContainer) -> DynamicGraphFieldNameExtension {
    match extension_container.get_own_extension(&EXTENSION_FIELD_NAME.clone()) {
        Some(field_names) => field_names.into(),
        None => Default::default(),
    }
}
