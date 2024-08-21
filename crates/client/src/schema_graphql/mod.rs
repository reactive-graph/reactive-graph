pub mod instances;
pub mod scalar;
pub mod system;
pub mod types;

#[allow(clippy::module_inception)]
pub mod schema {
    cynic::use_schema!("schema_graphql.graphql");
}

#[allow(clippy::upper_case_acronyms)]
type JSON = serde_json::Value;
cynic::impl_scalar!(JSON, schema::JSON);
