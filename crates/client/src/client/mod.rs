use std::fmt::Display;
use std::fmt::Formatter;
use std::sync::Arc;

use cynic::GraphQlError;
use cynic::Operation;
use cynic::QueryBuilder;
use cynic::http::CynicReqwestError;
use cynic::http::ReqwestExt;
use cynic_introspection::IntrospectionQuery;
use cynic_introspection::Schema;
use cynic_introspection::SchemaError;
use reqwest::Client;
use reqwest::Error;
use reqwest::header::InvalidHeaderValue;

use crate::client::instances::Instances;
use crate::client::json_schema::JsonSchema;
use crate::client::plugin::api::Plugins;
use crate::client::runtime::Runtime;
use crate::client::types::Types;
use reactive_graph_remotes_model::InstanceAddress;

pub mod instances;
pub mod json_schema;
pub mod plugin;
pub mod runtime;
pub mod types;

#[derive(Debug)]
pub enum ReactiveGraphClientError {
    InvalidBearer(InvalidHeaderValue),
    ClientCreationError(Error),
}

impl Display for ReactiveGraphClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReactiveGraphClientError::InvalidBearer(e) => {
                writeln!(f, "{e}")
            }
            ReactiveGraphClientError::ClientCreationError(e) => {
                writeln!(f, "{e}")
            }
        }
    }
}

impl std::error::Error for ReactiveGraphClientError {}

#[derive(Debug)]
pub enum ReactiveGraphClientExecutionError {
    FailedToSendRequest(CynicReqwestError),
    FailedToParseResponse(Error),
    GraphQlError(Vec<GraphQlError>),
    IntrospectionQueryError,
    IntrospectionQuerySchemaError(SchemaError),
}

impl From<CynicReqwestError> for ReactiveGraphClientExecutionError {
    fn from(e: CynicReqwestError) -> Self {
        ReactiveGraphClientExecutionError::FailedToSendRequest(e)
    }
}

impl Display for ReactiveGraphClientExecutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReactiveGraphClientExecutionError::FailedToSendRequest(e) => {
                writeln!(f, "Failed to send request:\n{e:?}")
            }
            ReactiveGraphClientExecutionError::FailedToParseResponse(e) => {
                writeln!(f, "Failed to parse response:\n{e:?}")
            }
            ReactiveGraphClientExecutionError::GraphQlError(e) => {
                let graphql_errors: Vec<String> = e.iter().map(|graphql_error| format!("{graphql_error}")).collect();
                writeln!(f, "The response returned errors:\n{}", graphql_errors.join("\n"))
            }
            ReactiveGraphClientExecutionError::IntrospectionQueryError => {
                writeln!(f, "Failed to run introspection query")
            }
            ReactiveGraphClientExecutionError::IntrospectionQuerySchemaError(e) => {
                writeln!(f, "Schema error on introspection query:\n{e:?}")
            }
        }
    }
}

impl std::error::Error for ReactiveGraphClientExecutionError {}

pub struct ReactiveGraphClient {
    remote: InstanceAddress,
    pub client: Client,
}

impl ReactiveGraphClient {
    pub fn new_default() -> Result<Arc<Self>, ReactiveGraphClientError> {
        ReactiveGraphClient::new(InstanceAddress::default())
    }

    pub fn new<A: Into<InstanceAddress>>(remote: A) -> Result<Arc<Self>, ReactiveGraphClientError> {
        let remote = remote.into();
        let mut client_builder = Client::builder()
            .user_agent(remote.user_agent.clone())
            .danger_accept_invalid_certs(remote.danger_accept_invalid_certs.unwrap_or_default())
            .danger_accept_invalid_hostnames(remote.danger_accept_invalid_hostnames.unwrap_or_default());
        if let Some(bearer) = remote.bearer.clone() {
            let header_value = reqwest::header::HeaderValue::from_str(&format!("Bearer {bearer}")).map_err(ReactiveGraphClientError::InvalidBearer)?;
            client_builder = client_builder.default_headers(std::iter::once((reqwest::header::AUTHORIZATION, header_value)).collect());
        }
        let client = client_builder.build().map_err(ReactiveGraphClientError::ClientCreationError)?;
        Ok(Arc::new(Self { remote, client }))
    }

    #[cfg(all(test, feature = "integration-tests"))]
    pub fn new_from_runtime(runtime: Arc<dyn reactive_graph_runtime_api::Runtime>) -> Result<Arc<Self>, ReactiveGraphClientError> {
        let config = runtime.get_config_manager().get_graphql_server_config();
        let address = InstanceAddress::new(config.hostname(), config.port(), config.is_secure());
        ReactiveGraphClient::new(address)
    }

    /// Returns the instance address.
    pub fn remote(&self) -> InstanceAddress {
        self.remote.clone()
    }

    /// Returns the URL of the graphql endpoint of the remote.
    pub fn url_reactive_graph(&self) -> String {
        self.remote.url_reactive_graph()
    }

    /// Returns the URL of the dynamic graph endpoint of the remote.
    pub fn url_dynamic_graph(&self) -> String {
        self.remote.url_dynamic_graph()
    }

    /// Returns the URL of the runtime endpoint of the remote.
    pub fn url_reactive_graph_runtime(&self) -> String {
        self.remote.url_reactive_graph_runtime()
    }

    /// Returns the URL of the plugins endpoint of the remote.
    pub fn url_reactive_graph_plugins(&self) -> String {
        self.remote.url_reactive_graph_plugins()
    }

    pub async fn introspection_query(&self, url: String) -> Result<Schema, ReactiveGraphClientExecutionError> {
        self.client
            .post(url)
            .run_graphql(IntrospectionQuery::build(()))
            .await
            .map_err(ReactiveGraphClientExecutionError::FailedToSendRequest)?
            .data
            .ok_or(ReactiveGraphClientExecutionError::IntrospectionQueryError)
            .and_then(|data| data.into_schema().map_err(ReactiveGraphClientExecutionError::IntrospectionQuerySchemaError))
    }

    pub async fn introspection_query_reactive_graph(&self) -> Result<Schema, ReactiveGraphClientExecutionError> {
        self.introspection_query(self.url_reactive_graph()).await
    }

    pub async fn introspection_query_dynamic_graph(&self) -> Result<Schema, ReactiveGraphClientExecutionError> {
        self.introspection_query(self.url_dynamic_graph()).await
    }

    pub async fn introspection_query_reactive_graph_runtime(&self) -> Result<Schema, ReactiveGraphClientExecutionError> {
        self.introspection_query(self.url_reactive_graph_runtime()).await
    }

    pub async fn introspection_query_reactive_graph_plugins(&self) -> Result<Schema, ReactiveGraphClientExecutionError> {
        self.introspection_query(self.url_reactive_graph_plugins()).await
    }

    pub fn types(self: &Arc<Self>) -> Types {
        Types::new(self.clone())
    }

    pub fn instances(self: &Arc<Self>) -> Instances {
        Instances::new(self.clone())
    }

    pub fn json_schema(self: &Arc<Self>) -> JsonSchema {
        JsonSchema::new(self.clone())
    }

    pub fn runtime(self: &Arc<Self>) -> Runtime {
        Runtime::new(self.clone())
    }

    pub fn plugins(self: &Arc<Self>) -> Plugins {
        Plugins::new(self.clone())
    }

    /// Runs a typed graphql query and extracts the response data.
    pub async fn execute_graphql<ResponseData, Vars, ResponseType>(
        &self,
        operation: Operation<ResponseData, Vars>,
        extractor: impl FnOnce(ResponseData) -> ResponseType,
    ) -> Result<ResponseType, ReactiveGraphClientExecutionError>
    where
        Vars: serde::Serialize,
        ResponseData: serde::de::DeserializeOwned + 'static,
    {
        self.execute(self.url_reactive_graph(), operation, extractor).await
    }

    /// Runs a typed graphql query and extracts the response data.
    pub async fn execute_dynamic_graph<ResponseData, Vars, ResponseType>(
        &self,
        operation: Operation<ResponseData, Vars>,
        extractor: impl FnOnce(ResponseData) -> ResponseType,
    ) -> Result<ResponseType, ReactiveGraphClientExecutionError>
    where
        Vars: serde::Serialize,
        ResponseData: serde::de::DeserializeOwned + 'static,
    {
        self.execute(self.url_dynamic_graph(), operation, extractor).await
    }

    /// Runs a typed graphql query and extracts the response data.
    pub async fn execute_runtime<ResponseData, Vars, ResponseType>(
        &self,
        operation: Operation<ResponseData, Vars>,
        extractor: impl FnOnce(ResponseData) -> ResponseType,
    ) -> Result<ResponseType, ReactiveGraphClientExecutionError>
    where
        Vars: serde::Serialize,
        ResponseData: serde::de::DeserializeOwned + 'static,
    {
        self.execute(self.url_reactive_graph_runtime(), operation, extractor).await
    }

    /// Runs a typed graphql query and extracts the response data.
    pub async fn execute_plugins<ResponseData, Vars, ResponseType>(
        &self,
        operation: Operation<ResponseData, Vars>,
        extractor: impl FnOnce(ResponseData) -> ResponseType,
    ) -> Result<ResponseType, ReactiveGraphClientExecutionError>
    where
        Vars: serde::Serialize,
        ResponseData: serde::de::DeserializeOwned + 'static,
    {
        self.execute(self.url_reactive_graph_plugins(), operation, extractor).await
    }

    /// Runs a typed graphql query and extracts the response data.
    pub async fn execute<ResponseData, Vars, ResponseType>(
        &self,
        endpoint: String,
        operation: Operation<ResponseData, Vars>,
        extractor: impl FnOnce(ResponseData) -> ResponseType,
    ) -> Result<ResponseType, ReactiveGraphClientExecutionError>
    where
        Vars: serde::Serialize,
        ResponseData: serde::de::DeserializeOwned + 'static,
    {
        let response = self
            // Cynic Client
            .client
            .post(endpoint)
            .run_graphql(operation)
            .await?;
        if let Some(data) = response.data.map(extractor) {
            return Ok(data);
        }
        Err(ReactiveGraphClientExecutionError::GraphQlError(response.errors.unwrap_or(vec![])))
    }
}
