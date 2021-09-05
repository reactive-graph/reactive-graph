use async_trait::async_trait;
use indradb::{Datastore, MemoryDatastore};
use waiter_di::*;

use crate::api::GraphDatabase;

#[wrapper]
pub struct InexorDatastore(MemoryDatastore);

#[provides]
fn create_external_type_dependency() -> InexorDatastore {
    InexorDatastore(MemoryDatastore::default())
}

#[component]
pub struct GraphDatabaseImpl {
    pub datastore: InexorDatastore,
}

#[async_trait]
#[provides]
impl GraphDatabase for GraphDatabaseImpl {
    fn get_transaction(&self) -> indradb::Result<indradb::MemoryTransaction> {
        self.datastore.0.transaction()
    }
}
