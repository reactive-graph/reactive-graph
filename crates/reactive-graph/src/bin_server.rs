use crate::server::args::ServerAndSharedArguments;
#[cfg(target_os = "linux")]
use crate::server::daemon::daemonize;
use crate::server::server;
use crate::shared::completions::handle_completions;
use crate::shared::info::handle_info_command;
#[cfg(target_os = "linux")]
use crate::shared::manpages::handle_man_pages;
use crate::shared::markdown_help::handle_markdown_help;
use clap::Parser;
use std::alloc::System;

pub mod server;
pub mod shared;

#[global_allocator]
static ALLOCATOR: System = System;

fn main() {
    let args = ServerAndSharedArguments::parse();

    handle_markdown_help::<ServerAndSharedArguments>(&args.shared.markdown_help);

    #[cfg(target_os = "linux")]
    handle_man_pages::<ServerAndSharedArguments>(&args.shared.man_pages);

    handle_completions::<ServerAndSharedArguments>(&args.shared.completions);

    handle_info_command(&args.shared.info);

    if let Some(commands) = &args.server.commands {
        #[allow(unreachable_patterns, clippy::collapsible_match)]
        match commands {
            #[cfg(target_os = "linux")]
            crate::server::commands::ServerCommands::Daemon(args) => daemonize(args),
            _ => {}
        }
    }

    server(args.server)
}
