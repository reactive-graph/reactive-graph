use crate::cli::args::ClientArgs;
use crate::cli::client;
use clap::Parser;
use std::alloc::System;
use std::process::exit;

mod cli;

#[global_allocator]
static ALLOCATOR: System = System;

#[tokio::main]
async fn main() {
    let client_args = ClientArgs::parse();

    if client_args.markdown_help {
        clap_markdown::print_help_markdown::<ClientArgs>();
        exit(0);
    }

    client(client_args).await
}
