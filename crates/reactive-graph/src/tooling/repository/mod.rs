pub mod args;

pub const DEFAULT_REPOSITORY_OWNER: &str = "reactive-graph";

pub trait Repository: Send + Sync {
    fn repository_owner(&self) -> String;
    fn repository_name(&self) -> String;
}
