use std::fmt::Display;
use std::fmt::Formatter;
use std::sync::Arc;

use crate::client::system::System;
use cynic::http::CynicReqwestError;
use reqwest::Client;
use reqwest::Error;

use crate::client::types::Types;
use crate::config::builder::InexorClientConfigBuilder;
use crate::config::InexorClientConfig;

pub mod system;
pub mod types;

#[derive(Debug)]
pub struct InexorRgfClientError;

#[derive(Debug)]
pub enum InexorRgfClientExecutionError {
    FailedToSendRequest(CynicReqwestError),
    FailedToParseResponse(Error),
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
        }
    }
}

pub struct InexorRgfClient {
    config: InexorClientConfig,
    pub client: Client,
}

impl InexorRgfClient {
    pub fn new() -> Result<Arc<Self>, InexorRgfClientError> {
        InexorRgfClient::new_from_config(InexorClientConfig::default())
    }

    pub fn from_builder(builder: InexorClientConfigBuilder) -> Result<Arc<Self>, InexorRgfClientError> {
        InexorRgfClient::new_from_config(builder.build())
    }

    pub fn new_from_config(config: InexorClientConfig) -> Result<Arc<Self>, InexorRgfClientError> {
        let mut client_builder = Client::builder().user_agent(config.user_agent.clone());
        if let Some(bearer) = config.bearer.clone() {
            let header_value = reqwest::header::HeaderValue::from_str(&format!("Bearer {}", bearer)).map_err(|_| InexorRgfClientError {})?;
            client_builder = client_builder.default_headers(std::iter::once((reqwest::header::AUTHORIZATION, header_value)).collect());
        }
        let client = client_builder.build().map_err(|_| InexorRgfClientError {})?;
        Ok(Arc::new(Self { config, client }))
    }

    pub fn url(&self) -> String {
        format!("{}://{}:{}{}", self.protocol(), self.config.hostname, self.config.port, self.config.endpoint)
    }

    pub fn protocol(&self) -> String {
        if self.config.secure {
            "https".to_string()
        } else {
            "http".to_string()
        }
    }

    pub fn types(self: &Arc<Self>) -> Types {
        Types::new(self.clone())
    }

    pub fn system(self: &Arc<Self>) -> System {
        System::new(self.clone())
    }
}
