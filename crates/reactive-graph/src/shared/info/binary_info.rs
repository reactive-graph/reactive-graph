use serde::Serialize;
use tabled::Tabled;
use typed_builder::TypedBuilder;

#[derive(Serialize, TypedBuilder, Tabled)]
pub struct BinaryInfo {
    #[builder(setter(into))]
    version: String,
    #[builder(setter(into))]
    target_triple: String,
    #[builder(setter(into))]
    git_tag: String,
    #[builder(setter(into))]
    git_commit: String,
    #[builder(setter(into))]
    rustc_channel: String,
    #[builder(setter(into))]
    rustc_version: String,
    #[builder(setter(into))]
    rustc_host_triple: String,
    #[builder(setter(into))]
    rustc_commit_date: String,
}
