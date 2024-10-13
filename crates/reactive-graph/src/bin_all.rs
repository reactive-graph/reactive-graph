pub mod all;
#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "server")]
pub mod server;
pub mod shared;
#[cfg(feature = "tooling")]
pub mod tooling;

use crate::all::args::AllInOneArguments;
#[cfg(feature = "client")]
use crate::client::client;
#[cfg(feature = "server")]
use crate::server::commands::ServerCommands;
#[cfg(all(feature = "server", target_os = "linux"))]
use crate::server::daemon::daemonize;
#[cfg(feature = "server")]
use crate::server::server;
use crate::shared::completions::handle_completions;
use crate::shared::info::handle_info_command;
#[cfg(target_os = "linux")]
use crate::shared::manpages::handle_man_pages;
use crate::shared::markdown_help::handle_markdown_help;
#[cfg(feature = "tooling")]
use crate::tooling::tooling;
use clap::Parser;
use std::alloc::System;

#[global_allocator]
static ALLOCATOR: System = System;

fn main() {
    let args = AllInOneArguments::parse();

    handle_markdown_help::<AllInOneArguments>(&args.shared.markdown_help);

    #[cfg(target_os = "linux")]
    handle_man_pages::<AllInOneArguments>(&args.shared.man_pages);

    handle_completions::<AllInOneArguments>(&args.shared.completions);

    handle_info_command(&args.shared.info);

    #[cfg(feature = "tooling")]
    if let Some(_commands) = &args.tooling.commands {
        return tooling(args.tooling);
    }

    #[cfg(feature = "client")]
    if let Some(_commands) = &args.client.commands {
        return client(args.client);
    }

    #[cfg(feature = "server")]
    if let Some(commands) = &args.server.commands {
        #[allow(unreachable_patterns)]
        match commands {
            #[cfg(target_os = "linux")]
            ServerCommands::Daemon(args) => daemonize(args),
            _ => {}
        }
        return server(args.server);
    }

    #[cfg(feature = "server")]
    return server(args.server);

    #[cfg(all(not(feature = "server"), feature = "client"))]
    return client(args.client);
}
