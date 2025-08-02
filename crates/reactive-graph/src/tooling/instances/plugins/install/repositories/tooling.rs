use crate::tooling::repository::DEFAULT_REPOSITORY_OWNER;
use crate::tooling::repository::Repository;
use std::sync::LazyLock;

#[derive(Debug)]
pub struct ToolingRepository {}

impl Repository for ToolingRepository {
    fn repository_owner(&self) -> String {
        DEFAULT_REPOSITORY_OWNER.to_string()
    }

    fn repository_name(&self) -> String {
        "tooling".to_string()
    }
}

pub static PLUGINS_REPOSITORY_TOOLING: LazyLock<Box<dyn Repository>> = LazyLock::new(|| Box::new(ToolingRepository {}));
