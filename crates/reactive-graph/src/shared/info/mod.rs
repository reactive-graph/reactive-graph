use crate::shared::info::args::InfoArgs;
use crate::shared::info::binary_info::BinaryInfo;
use crate::shared::info::commands::InfoCommands;
use crate::shared::output_format::RenderTable;
use std::ops::Deref;
use std::sync::LazyLock;

pub mod args;
pub mod binary_info;
pub mod commands;

pub static VERSION: &str = env!("CARGO_PKG_VERSION");
pub static TARGET_TRIPLE: &str = env!("VERGEN_CARGO_TARGET_TRIPLE");
pub static GIT_TAG: &str = env!("VERGEN_GIT_DESCRIBE");
pub static GIT_COMMIT: &str = env!("VERGEN_GIT_SHA");
pub static RUSTC_CHANNEL: &str = env!("VERGEN_RUSTC_CHANNEL");
pub static RUSTC_VERSION: &str = env!("VERGEN_RUSTC_SEMVER");

pub static BINARY_INFO: LazyLock<BinaryInfo> = LazyLock::new(|| {
    BinaryInfo::builder()
        .version(VERSION)
        .target_triple(TARGET_TRIPLE)
        .git_tag(GIT_TAG)
        .git_commit(GIT_COMMIT)
        .rustc_version(RUSTC_VERSION)
        .rustc_channel(RUSTC_CHANNEL)
        .build()
});

pub fn handle_info_command(args: &InfoArgs) {
    #[allow(clippy::match_single_binding)]
    if let Some(commands) = &args.commands {
        match commands {
            InfoCommands::Info(args) => vec![BINARY_INFO.deref()].print_table_and_exit(&args.output_format),
        }
    }
}
