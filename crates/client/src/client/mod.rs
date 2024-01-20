use std::fmt::Display;
use std::fmt::Formatter;
use std::sync::Arc;

use cynic::http::CynicReqwestError;
use cynic::http::ReqwestExt;
use cynic::GraphQlError;
use cynic::Operation;
use reqwest::header::InvalidHeaderValue;
use reqwest::Client;
use reqwest::Error;

use crate::client::plugin::api::Plugins;
use crate::client::runtime::Runtime;
use crate::client::types::Types;
use inexor_rgf_remotes_model::InstanceAddress;

pub mod plugin;
pub mod runtime;
pub mod types;

#[derive(Debug)]
pub enum InexorRgfClientError {
    InvalidBearer(InvalidHeaderValue),
    ClientCreationError(Error),
}

impl Display for InexorRgfClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InexorRgfClientError::InvalidBearer(e) => {
                writeln!(f, "{}", e)
            }
            InexorRgfClientError::ClientCreationError(e) => {
                writeln!(f, "{}", e)
            }
        }
    }
}

impl std::error::Error for InexorRgfClientError {}

#[derive(Debug)]
pub enum InexorRgfClientExecutionError {
    FailedToSendRequest(CynicReqwestError),
    FailedToParseResponse(Error),
    GraphQlError(Vec<GraphQlError>),
}

impl From<CynicReqwestError> for InexorRgfClientExecutionError {
    fn from(e: CynicReqwestError) -> Self {
        InexorRgfClientExecutionError::FailedToSendRequest(e)
    }
}

impl Display for InexorRgfClientExecutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InexorRgfClientExecutionError::FailedToSendRequest(e) => {
                writeln!(f, "Failed to send request:\n{e:?}")
            }
            InexorRgfClientExecutionError::FailedToParseResponse(e) => {
                writeln!(f, "Failed to parse response:\n{e:?}")
            }
            InexorRgfClientExecutionError::GraphQlError(e) => {
                let graphql_errors: Vec<String> = e.iter().map(|graphql_error| format!("{}", graphql_error)).collect();
                writeln!(f, "The response returned errors:\n{}", graphql_errors.join("\n"))
            }
        }
    }
}

impl std::error::Error for InexorRgfClientExecutionError {}

pub struct InexorRgfClient {
    remote: InstanceAddress,
    pub client: Client,
}

impl InexorRgfClient {
    pub fn new_default() -> Result<Arc<Self>, InexorRgfClientError> {
        InexorRgfClient::new(InstanceAddress::default())
    }

    pub fn new<A: Into<InstanceAddress>>(remote: A) -> Result<Arc<Self>, InexorRgfClientError> {
        let remote = remote.into();
        let mut client_builder = Client::builder().user_agent(remote.user_agent.clone());
        if let Some(bearer) = remote.bearer.clone() {
            let header_value = reqwest::header::HeaderValue::from_str(&format!("Bearer {}", bearer)).map_err(InexorRgfClientError::InvalidBearer)?;
            client_builder = client_builder.default_headers(std::iter::once((reqwest::header::AUTHORIZATION, header_value)).collect());
        }
        let client = client_builder.build().map_err(InexorRgfClientError::ClientCreationError)?;
        Ok(Arc::new(Self { remote, client }))
    }

    /// Returns the instance address.
    pub fn remote(&self) -> InstanceAddress {
        self.remote.clone()
    }

    /// Returns the URL of the graphql endpoint of the remote.
    pub fn url_graphql(&self) -> String {
        self.remote.url_graphql()
    }

    /// Returns the URL of the dynamic graph endpoint of the remote.
    pub fn url_dynamic_graph(&self) -> String {
        self.remote.url_dynamic_graph()
    }

    /// Returns the URL of the runtime endpoint of the remote.
    pub fn url_runtime(&self) -> String {
        self.remote.url_runtime()
    }

    /// Returns the URL of the plugins endpoint of the remote.
    pub fn url_plugin(&self) -> String {
        self.remote.url_plugin()
    }

    pub fn types(self: &Arc<Self>) -> Types {
        Types::new(self.clone())
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
    ) -> Result<ResponseType, InexorRgfClientExecutionError>
    where
        Vars: serde::Serialize,
        ResponseData: serde::de::DeserializeOwned + 'static,
    {
        self.execute(self.url_graphql(), operation, extractor).await
    }

    /// Runs a typed graphql query and extracts the response data.
    pub async fn execute_dynamic_graph<ResponseData, Vars, ResponseType>(
        &self,
        operation: Operation<ResponseData, Vars>,
        extractor: impl FnOnce(ResponseData) -> ResponseType,
    ) -> Result<ResponseType, InexorRgfClientExecutionError>
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
    ) -> Result<ResponseType, InexorRgfClientExecutionError>
    where
        Vars: serde::Serialize,
        ResponseData: serde::de::DeserializeOwned + 'static,
    {
        self.execute(self.url_runtime(), operation, extractor).await
    }

    /// Runs a typed graphql query and extracts the response data.
    pub async fn execute_plugins<ResponseData, Vars, ResponseType>(
        &self,
        operation: Operation<ResponseData, Vars>,
        extractor: impl FnOnce(ResponseData) -> ResponseType,
    ) -> Result<ResponseType, InexorRgfClientExecutionError>
    where
        Vars: serde::Serialize,
        ResponseData: serde::de::DeserializeOwned + 'static,
    {
        self.execute(self.url_plugin(), operation, extractor).await
    }

    /// Runs a typed graphql query and extracts the response data.
    pub async fn execute<ResponseData, Vars, ResponseType>(
        &self,
        endpoint: String,
        operation: Operation<ResponseData, Vars>,
        extractor: impl FnOnce(ResponseData) -> ResponseType,
    ) -> Result<ResponseType, InexorRgfClientExecutionError>
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
        Err(InexorRgfClientExecutionError::GraphQlError(response.errors.unwrap_or(vec![])))
    }
}
