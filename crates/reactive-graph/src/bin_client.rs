use crate::client::args::ClientAndSharedArguments;
use crate::client::client;
use crate::shared::completions::handle_completions;
use crate::shared::info::handle_info_command;
#[cfg(target_os = "linux")]
use crate::shared::manpages::handle_man_pages;
use crate::shared::markdown_help::handle_markdown_help;
use clap::Parser;
use std::alloc::System;

pub mod client;
pub mod shared;

#[global_allocator]
static ALLOCATOR: System = System;

fn main() {
    let args = ClientAndSharedArguments::parse();

    handle_markdown_help::<ClientAndSharedArguments>(&args.shared.markdown_help);

    #[cfg(target_os = "linux")]
    handle_man_pages::<ClientAndSharedArguments>(&args.shared.man_pages);

    handle_completions::<ClientAndSharedArguments>(&args.shared.completions);

    handle_info_command(&args.shared.info);

    client(args.client)
}
