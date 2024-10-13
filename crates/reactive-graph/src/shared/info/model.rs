use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;
use typed_builder::TypedBuilder;

#[derive(Serialize, TypedBuilder)]
pub struct BinInfo {
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
}

impl Display for BinInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VERSION: {}\nTARGET_TRIPLE: {}\nGIT_TAG: {}\nGIT_COMMIT: {}\nRUSTC_CHANNEL: {}\nRUSTC_VERSION: {}\n",
            &self.version, &self.target_triple, &self.git_tag, &self.git_commit, &self.rustc_channel, &self.rustc_version
        )
    }
}
