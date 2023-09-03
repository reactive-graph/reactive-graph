use crossbeam::channel::Receiver;

use async_trait::async_trait;

use crate::api::Lifecycle;

#[async_trait]
pub trait GraphQLServer: Send + Sync + Lifecycle {
    async fn serve(&self, stopper: Receiver<()>);
}
