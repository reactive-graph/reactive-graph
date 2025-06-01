pub mod args;

use crate::server::daemon::args::DaemonArguments;
use daemonize_me::Daemon;
use daemonize_me::Group;
use daemonize_me::User;
use std::any::Any;
use std::fs::File;
use std::process::exit;

// Initialize daemon
pub fn daemonize(args: &DaemonArguments) {
    let daemon = Daemon::new()
        .work_dir(args.daemon_working_directory.clone().unwrap_or(String::from(".")))
        .setup_post_fork_parent_hook(post_fork_parent)
        .setup_post_fork_child_hook(post_fork_child)
        .setup_post_init_hook(after_init, None);
    let daemon = match args.daemon_name {
        Some(ref daemon_name) => daemon.name(daemon_name.as_ref()),
        None => daemon,
    };
    let daemon = match args.daemon_pid {
        Some(ref daemon_pid) => daemon.pid_file(daemon_pid, Some(false)),
        None => daemon,
    };
    let daemon = match args.daemon_stdout {
        Some(ref daemon_stdout) => match File::create(daemon_stdout) {
            Ok(stdout) => daemon.stdout(stdout),
            Err(_) => daemon,
        },
        None => daemon,
    };
    let daemon = match args.daemon_stderr {
        Some(ref daemon_stderr) => match File::create(daemon_stderr) {
            Ok(stderr) => daemon.stderr(stderr),
            Err(_) => daemon,
        },
        None => daemon,
    };
    let daemon = if let (Some(ref daemon_user), Some(ref daemon_group)) = (args.daemon_user.clone(), args.daemon_group.clone()) {
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
            eprintln!("Failed to run as daemon: {e}");
            exit(-1);
        }
    }
}

pub fn post_fork_parent(_parent_pid: i32, _child_pid: i32) -> ! {
    exit(0);
}

pub fn post_fork_child(_parent_pid: i32, _child_pid: i32) {
    // Child hook must return
}

pub fn after_init(_: Option<&dyn Any>) {}
