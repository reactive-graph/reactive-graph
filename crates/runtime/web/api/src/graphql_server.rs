use crossbeam::channel::Receiver;

use async_trait::async_trait;
use springtime_di::injectable;

use inexor_rgf_lifecycle::Lifecycle;

#[injectable]
#[async_trait]
pub trait GraphQLServer: Send + Sync + Lifecycle {
    async fn serve(&self, stopper: Receiver<()>);
}
