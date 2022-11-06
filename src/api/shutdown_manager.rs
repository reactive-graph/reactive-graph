use async_trait::async_trait;
use uuid::Uuid;

use crate::api::Lifecycle;

// Ensure stable UUID for the shutdown entity
pub static UUID_SHUTDOWN: Uuid = Uuid::from_u128(0x6ba7b8109e1511d150b400c04fd530c7);
pub static UUID_SHUTDOWN_TRIGGER: Uuid = Uuid::from_u128(0x6ba7b8109e1511d150b400c04fe530c7);

#[async_trait]
pub trait ShutdownManager: Send + Sync + Lifecycle {
    fn do_shutdown(&self);

    fn is_shutdown(&self) -> bool;
}
