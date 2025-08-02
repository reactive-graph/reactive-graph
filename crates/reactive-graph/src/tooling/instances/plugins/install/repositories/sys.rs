use crate::tooling::repository::DEFAULT_REPOSITORY_OWNER;
use crate::tooling::repository::Repository;
use std::sync::LazyLock;

#[derive(Debug)]
pub struct SysRepository {}

impl Repository for SysRepository {
    fn repository_owner(&self) -> String {
        DEFAULT_REPOSITORY_OWNER.to_string()
    }

    fn repository_name(&self) -> String {
        "sys".to_string()
    }
}

pub static PLUGINS_REPOSITORY_SYS: LazyLock<Box<dyn Repository>> = LazyLock::new(|| Box::new(SysRepository {}));
