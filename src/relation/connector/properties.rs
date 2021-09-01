use indradb::NamedProperty;
use serde_json::json;
use strum_macros::{AsRefStr, IntoStaticStr, ToString};

use crate::property::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, ToString)]
pub enum ConnectorProperties {
    #[strum(serialize = "outbound_property_name")]
    OUTBOUND_PROPERTY_NAME,
    #[strum(serialize = "inbound_property_name")]
    INBOUND_PROPERTY_NAME,
}

impl ConnectorProperties {
    pub fn default_value(&self) -> String {
        match self {
            ConnectorProperties::OUTBOUND_PROPERTY_NAME => String::from(""),
            ConnectorProperties::INBOUND_PROPERTY_NAME => String::from(""),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(ConnectorProperties::OUTBOUND_PROPERTY_NAME),
            NamedProperty::from(ConnectorProperties::INBOUND_PROPERTY_NAME),
        ]
    }
}

impl From<ConnectorProperties> for NamedProperty {
    fn from(p: ConnectorProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: json!(p.default_value())
        }
    }
}
