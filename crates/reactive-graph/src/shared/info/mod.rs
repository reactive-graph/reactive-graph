use crate::shared::info::args::InfoArgs;
use crate::shared::info::args::OutputFormatArgs;
use crate::shared::info::commands::InfoCommands;
use crate::shared::info::model::BinInfo;
use std::ops::Deref;
use std::process::exit;
use std::sync::LazyLock;

pub mod args;
pub mod commands;
pub mod model;

pub static VERSION: &str = env!("CARGO_PKG_VERSION");
pub static TARGET_TRIPLE: &str = env!("VERGEN_CARGO_TARGET_TRIPLE");
pub static GIT_TAG: &str = env!("VERGEN_GIT_DESCRIBE");
pub static GIT_COMMIT: &str = env!("VERGEN_GIT_SHA");
pub static RUSTC_CHANNEL: &str = env!("VERGEN_RUSTC_CHANNEL");
pub static RUSTC_VERSION: &str = env!("VERGEN_RUSTC_SEMVER");

pub static BIN_INFO: LazyLock<BinInfo> = LazyLock::new(|| {
    BinInfo::builder()
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
            InfoCommands::Info(args) => match args.output_format.clone().unwrap_or_default() {
                OutputFormatArgs::Default => {
                    println!("{}", BIN_INFO.deref());
                    exit(0);
                }
                OutputFormatArgs::Json => match serde_json::to_string(BIN_INFO.deref()) {
                    Ok(serialized) => {
                        println!("{serialized}");
                        exit(0);
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        exit(1);
                    }
                },
                #[cfg(feature = "toml")]
                OutputFormatArgs::Toml => match toml::to_string(BIN_INFO.deref()) {
                    Ok(serialized) => {
                        println!("{serialized}");
                        exit(0);
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        exit(1);
                    }
                },
            },
        }
    }
}
