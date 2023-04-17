pub mod component;
pub mod data_type;
pub mod extension;
pub mod mutability;
pub mod plugin;
pub mod property_instance;
pub mod property_type;
pub mod scalar;
pub mod socket_type;

pub mod schema {
    cynic::use_schema!("schema.graphql");
}

type JSON = serde_json::Value;
cynic::impl_scalar!(JSON, schema::JSON);
