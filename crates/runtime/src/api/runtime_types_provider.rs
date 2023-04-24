use async_trait::async_trait;

use crate::api::Lifecycle;

#[async_trait]
pub trait RuntimeTypesProvider: Send + Sync + Lifecycle {}
