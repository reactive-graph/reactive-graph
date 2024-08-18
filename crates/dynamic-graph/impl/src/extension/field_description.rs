use serde::Deserialize;
use serde::Serialize;
use serde_json::from_value;

use reactive_graph_dynamic_graph_model::EXTENSION_FIELD_DESCRIPTION;
use reactive_graph_graph::Extension;
use reactive_graph_graph::ExtensionContainer;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DynamicGraphFieldDescriptionExtension {
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

impl From<Extension> for DynamicGraphFieldDescriptionExtension {
    fn from(extension: Extension) -> Self {
        from_value(extension.extension).unwrap_or_default()
    }
}

pub fn get_dynamic_graph_field_descriptions(extension_container: &impl ExtensionContainer) -> DynamicGraphFieldDescriptionExtension {
    match extension_container.get_own_extension(&EXTENSION_FIELD_DESCRIPTION.clone()) {
        Some(field_descriptions) => field_descriptions.into(),
        None => Default::default(),
    }
}
