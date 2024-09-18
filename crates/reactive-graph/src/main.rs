use std::alloc::System;
use std::process::exit;

use clap::Parser;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::Appender;
use log4rs::config::Root;
use log4rs::Config;

#[cfg(feature = "client")]
use crate::cli::client;
#[cfg(feature = "server")]
use crate::server::cli_args::CliArguments;
#[cfg(feature = "server")]
use crate::server::cli_args::Commands;
#[cfg(feature = "server")]
use crate::server::server;

#[cfg(target_os = "linux")]
use daemonize_me::Daemon;
#[cfg(target_os = "linux")]
use daemonize_me::Group;
#[cfg(target_os = "linux")]
use daemonize_me::User;
#[cfg(target_os = "linux")]
use std::any::Any;
#[cfg(target_os = "linux")]
use std::fs::File;

#[cfg(feature = "client")]
mod cli;
#[cfg(feature = "server")]
mod server;

#[global_allocator]
static ALLOCATOR: System = System;

fn main() {
    #[cfg(feature = "server")]
    {}
    let cli_args = CliArguments::parse();

    // Export CLI help as markdown
    if cli_args.markdown_help {
        clap_markdown::print_help_markdown::<CliArguments>();
        exit(0);
    }

    // Initialize daemon
    #[cfg(target_os = "linux")]
    {
        if cli_args.daemon {
            let daemon = Daemon::new()
                .work_dir(cli_args.daemon_working_directory.clone().unwrap_or(String::from(".")))
                .setup_post_fork_parent_hook(post_fork_parent)
                .setup_post_fork_child_hook(post_fork_child)
                .setup_post_init_hook(after_init, None);
            let daemon = match cli_args.daemon_name {
                Some(ref daemon_name) => daemon.name(daemon_name.as_ref()),
                None => daemon,
            };
            let daemon = match cli_args.daemon_pid {
                Some(ref daemon_pid) => daemon.pid_file(daemon_pid, Some(false)),
                None => daemon,
            };
            let daemon = match cli_args.daemon_stdout {
                Some(ref daemon_stdout) => match File::create(daemon_stdout) {
                    Ok(stdout) => daemon.stdout(stdout),
                    Err(_) => daemon,
                },
                None => daemon,
            };
            let daemon = match cli_args.daemon_stderr {
                Some(ref daemon_stderr) => match File::create(daemon_stderr) {
                    Ok(stderr) => daemon.stderr(stderr),
                    Err(_) => daemon,
                },
                None => daemon,
            };
            let daemon = if let (Some(ref daemon_user), Some(ref daemon_group)) = (cli_args.daemon_user.clone(), cli_args.daemon_group.clone()) {
                if let (Ok(daemon_user), Ok(daemon_group)) = (User::try_from(daemon_user), Group::try_from(daemon_group)) {
                    daemon.user(daemon_user).group(daemon_group)
                } else {
                    daemon
                }
            } else {
                daemon
            };

            let daemon = daemon.start();
            match daemon {
                Ok(_) => println!("Process has been daemonized with success"),
                Err(e) => {
                    eprintln!("Failed to run as daemon: {}", e);
                    exit(-1);
                }
            }
        }
    }

    // Initialize logging
    if !cli_args.quiet.unwrap_or(false) {
        let logging_config_location = cli_args.logging_config.clone().unwrap_or(String::from("./config/logging.toml"));

        if let Err(error) = log4rs::init_file(&logging_config_location, Default::default()) {
            eprintln!("Failed to configure logger using config file {}: {}", &logging_config_location, error);
            let stdout = ConsoleAppender::builder().build();
            let Ok(config) = Config::builder()
                .appender(Appender::builder().build("stdout", Box::new(stdout)))
                .build(Root::builder().appender("stdout").build(LevelFilter::Info))
            else {
                eprintln!("Failed to create fallback logger! Exiting with error");
                exit(1);
            };
            if let Err(error) = log4rs::init_config(config) {
                eprintln!("Failed to configure logger: {}", error);
            }
        }
    }

    tokio_main(cli_args)
}

#[tokio::main]
async fn tokio_main(cli_args: CliArguments) {
    match cli_args.commands {
        Some(commands) => match commands {
            #[cfg(feature = "client")]
            Commands::Client(args) => client(args).await,
        },
        None => {
            #[cfg(feature = "server")]
            {
                server(cli_args).await
            }
        }
    }
}

#[cfg(target_os = "linux")]
fn post_fork_parent(_parent_pid: i32, _child_pid: i32) -> ! {
    // println!("Exiting parent process. Parent PID: {}, Child PID {}", parent_pid, child_pid);
    exit(0);
}

#[cfg(target_os = "linux")]
fn post_fork_child(_parent_pid: i32, _child_pid: i32) {
    // println!("Forked into child process. Parent PID: {}, Child PID {}", parent_pid, child_pid);
    // Child hook must return
    return;
}

#[cfg(target_os = "linux")]
fn after_init(_: Option<&dyn Any>) {
    // println!("Daemon has been initialized");
    return;
}
