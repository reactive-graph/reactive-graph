use async_trait::async_trait;
use indradb::MemoryDatastore;
use std::sync::Arc;

#[async_trait]
pub trait GraphDatabase: Send + Sync {
    fn get_datastore(&self) -> Arc<MemoryDatastore>;
}
