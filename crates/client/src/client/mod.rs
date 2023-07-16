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

use crate::client::system::System;
use crate::client::types::Types;
use crate::config::InstanceAddress;

pub mod system;
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
                writeln!(f, "{}", e)
            }
            InexorRgfClientExecutionError::FailedToParseResponse(e) => {
                writeln!(f, "{}", e)
            }
            InexorRgfClientExecutionError::GraphQlError(e) => {
                let graphql_errors: Vec<String> = e.iter().map(|graphql_error| format!("{}", graphql_error)).collect();
                writeln!(f, "{}", graphql_errors.join("\n"))
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

    /// Returns the URL of the remote
    pub fn url(&self) -> String {
        self.remote.url()
    }

    pub fn types(self: &Arc<Self>) -> Types {
        Types::new(self.clone())
    }

    pub fn system(self: &Arc<Self>) -> System {
        System::new(self.clone())
    }

    /// Runs a typed graphql query and extracts the response data.
    pub async fn run_graphql<ResponseData, Vars, ResponseType>(
        &self,
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
            .post(self.url())
            .run_graphql(operation)
            .await?;
        if let Some(data) = response.data.map(extractor) {
            return Ok(data);
        }
        Err(InexorRgfClientExecutionError::GraphQlError(response.errors.unwrap_or(vec![])))
    }
}
