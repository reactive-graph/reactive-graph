use crate::tooling::instances::plugins::args::InstallPluginArgs;
use std::path::PathBuf;
use std::process::exit;

pub fn install_plugin(_instance_dir: &PathBuf, _args: InstallPluginArgs) {
    eprintln!("Not yet implemented");
    exit(1);
}
