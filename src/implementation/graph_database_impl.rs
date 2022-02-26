use crate::di::*;
use async_trait::async_trait;
use indradb::MemoryDatastore;
use std::sync::Arc;

use crate::api::GraphDatabase;

#[wrapper]
pub struct InexorDatastore(Arc<MemoryDatastore>);

#[provides]
fn create_datastore() -> InexorDatastore {
    InexorDatastore(Arc::new(MemoryDatastore::default()))
}

#[component]
pub struct GraphDatabaseImpl {
    pub datastore: InexorDatastore,
}

#[async_trait]
#[provides]
impl GraphDatabase for GraphDatabaseImpl {
    fn get_datastore(&self) -> Arc<MemoryDatastore> {
        self.datastore.0.clone()
    }
}
