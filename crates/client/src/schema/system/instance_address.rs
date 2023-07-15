#[derive(Clone, Debug, cynic::InputObject)]
#[cynic(schema_path = "schema.graphql", schema_module = "crate::schema::schema")]
pub struct InstanceAddress {
    pub hostname: String,
    pub port: i32,
    pub secure: bool,
}

impl From<crate::model_runtime::InstanceAddress> for InstanceAddress {
    fn from(address: crate::model_runtime::InstanceAddress) -> Self {
        InstanceAddress {
            hostname: address.hostname,
            port: address.port as i32,
            secure: address.secure,
        }
    }
}