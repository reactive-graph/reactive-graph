use crate::tooling::instances::plugins::args::UninstallPluginArgs;
use std::path::PathBuf;
use std::process::exit;

pub fn uninstall_plugin(_instance_dir: &PathBuf, _args: UninstallPluginArgs) {
    eprintln!("Not yet implemented");
    exit(1);
}
