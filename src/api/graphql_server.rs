use std::sync::mpsc::Receiver;

use async_trait::async_trait;
use serde_json::Error;

use crate::api::Lifecycle;
use crate::graphql::InexorSchema;

#[async_trait]
pub trait GraphQLServer: Send + Sync + Lifecycle {
    fn get_schema(&self) -> InexorSchema;

    async fn query(&self, request: String) -> Result<String, Error>;

    fn query_thread(&self, request: String);

    fn serve(&self, stopper: Receiver<()>);
}
