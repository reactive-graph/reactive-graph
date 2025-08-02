use crate::tooling::repository::DEFAULT_REPOSITORY_OWNER;
use crate::tooling::repository::Repository;
use std::sync::LazyLock;

#[derive(Debug)]
pub struct NetRepository {}

impl Repository for NetRepository {
    fn repository_owner(&self) -> String {
        DEFAULT_REPOSITORY_OWNER.to_string()
    }

    fn repository_name(&self) -> String {
        "net".to_string()
    }
}

pub static PLUGINS_REPOSITORY_NET: LazyLock<Box<dyn Repository>> = LazyLock::new(|| Box::new(NetRepository {}));
