use crate::tooling::instances::commands::InstancesCommands;
use crate::tooling::instances::provisioning::Chown;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct InstancesArgs {
    /// The working directory of the instance.
    /// Defaults to the current directory.
    pub working_directory: Option<PathBuf>,

    #[command(subcommand)]
    pub commands: InstancesCommands,
}

#[derive(Parser, Debug)]
pub struct ChownArgs {
    /// The numeric user id of the owner user.
    #[cfg(target_os = "linux")]
    #[arg(long)]
    pub uid: Option<u32>,

    /// The numeric group id of the owner group.
    #[cfg(target_os = "linux")]
    #[arg(long)]
    pub gid: Option<u32>,
}

impl ChownArgs {
    pub fn get_chown(&self) -> Option<Chown> {
        #[cfg(target_os = "linux")]
        if let (Some(uid), Some(gid)) = (self.uid, self.gid) {
            return Some(Chown::new(uid, gid));
        };
        None
    }
}
