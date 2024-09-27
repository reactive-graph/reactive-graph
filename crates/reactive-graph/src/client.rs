use crate::cli::args::ClientArgs;
use crate::cli::client;
#[cfg(target_os = "linux")]
use crate::completions::install_shell_completions;
use crate::completions::print_shell_completions;
#[cfg(target_os = "linux")]
use crate::manpages::install_man_pages;
#[cfg(target_os = "linux")]
use crate::manpages::print_man_pages;
use clap::CommandFactory;
use clap::Parser;
use std::alloc::System;
use std::process::exit;

mod cli;

mod completions;

#[cfg(target_os = "linux")]
mod manpages;

#[global_allocator]
static ALLOCATOR: System = System;

#[tokio::main]
async fn main() {
    let client_args = ClientArgs::parse();

    if client_args.markdown_help {
        clap_markdown::print_help_markdown::<ClientArgs>();
        exit(0);
    }

    // Print man pages
    #[cfg(target_os = "linux")]
    if client_args.print_man_pages {
        if let Err(e) = print_man_pages(ClientArgs::command()) {
            eprintln!("Failed to print man pages: {e}");
            exit(1);
        };
        exit(0);
    }

    // Install man pages
    #[cfg(target_os = "linux")]
    if client_args.install_man_pages {
        if let Err(e) = install_man_pages(ClientArgs::command()) {
            eprintln!("Failed to install man pages: {e}");
            exit(1);
        }
        exit(0);
    }

    // Print shell completions
    if let Some(completions) = client_args.print_shell_completions {
        let mut cmd = ClientArgs::command();
        print_shell_completions(completions, &mut cmd);
        exit(0);
    }

    // Install shell completions
    #[cfg(target_os = "linux")]
    if let Some(completions) = client_args.install_shell_completions {
        let mut cmd = ClientArgs::command();
        if let Err(e) = install_shell_completions(completions, completions, &mut cmd) {
            eprintln!("Failed to install shell completions: {e}");
            exit(1);
        }
        exit(0);
    }

    client(client_args).await
}
