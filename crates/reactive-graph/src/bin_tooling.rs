use crate::shared::completions::handle_completions;
use crate::shared::info::handle_info_command;
#[cfg(target_os = "linux")]
use crate::shared::manpages::handle_man_pages;
use crate::shared::markdown_help::handle_markdown_help;
use crate::tooling::args::ToolingAndSharedArguments;
use crate::tooling::tooling;
use clap::Parser;
use std::alloc::System;

pub mod shared;
pub mod tooling;

#[global_allocator]
static ALLOCATOR: System = System;

fn main() {
    let args = ToolingAndSharedArguments::parse();

    handle_markdown_help::<ToolingAndSharedArguments>(&args.shared.markdown_help);

    #[cfg(target_os = "linux")]
    handle_man_pages::<ToolingAndSharedArguments>(&args.shared.man_pages);

    handle_completions::<ToolingAndSharedArguments>(&args.shared.completions);

    handle_info_command(&args.shared.info);

    tooling(args.tooling)
}
