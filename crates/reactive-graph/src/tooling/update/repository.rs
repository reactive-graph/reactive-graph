use crate::tooling::repository::DEFAULT_REPOSITORY_OWNER;
use crate::tooling::repository::Repository;
use std::sync::LazyLock;

#[derive(Debug)]
pub struct ReactiveGraphRepository {}

impl Repository for ReactiveGraphRepository {
    fn repository_owner(&self) -> String {
        DEFAULT_REPOSITORY_OWNER.to_string()
    }

    fn repository_name(&self) -> String {
        "reactive-graph".to_string()
    }
}

pub static REACTIVE_GRAPH_REPOSITORY: LazyLock<Box<dyn Repository>> = LazyLock::new(|| Box::new(ReactiveGraphRepository {}));
