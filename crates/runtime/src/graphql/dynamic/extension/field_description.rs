use serde::Deserialize;
use serde::Serialize;
use serde_json::from_value;

use crate::model::Extension;
use crate::model::ExtensionContainer;
use crate::model_dynamic_graph::EXTENSION_FIELD_DESCRIPTION;

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
        match from_value(extension.extension) {
            Ok(field_descriptions) => field_descriptions,
            Err(_) => Default::default(),
        }
    }
}

pub fn get_dynamic_graph_field_descriptions(extension_container: &impl ExtensionContainer) -> DynamicGraphFieldDescriptionExtension {
    match extension_container.get_own_extension(&EXTENSION_FIELD_DESCRIPTION.clone()) {
        Some(field_descriptions) => field_descriptions.into(),
        None => Default::default(),
    }
}
