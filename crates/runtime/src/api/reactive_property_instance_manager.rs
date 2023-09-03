use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait ReactivePropertyManager: Send + Sync {
    fn connect(&self, outbound_uuid: Uuid, inbound_uuid: Uuid);

    // TODO: automatically disconnect if ReactiveProperty is destroyed
    fn disconnect(&self, outbound_uuid: Uuid, inbound_uuid: Uuid);
}
