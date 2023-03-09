use std::sync::mpsc::Receiver;

use async_trait::async_trait;

use crate::api::Lifecycle;

#[async_trait]
pub trait GraphQLServer: Send + Sync + Lifecycle {
    fn serve(&self, stopper: Receiver<()>);
}
