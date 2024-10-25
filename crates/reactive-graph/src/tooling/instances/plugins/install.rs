use crate::tooling::instances::plugins::args::InstallPluginArgs;
use anyhow::anyhow;
use anyhow::Result;
use std::path::PathBuf;

pub fn install_plugin(_instance_dir: &PathBuf, _args: InstallPluginArgs) -> Result<()> {
    Err(anyhow!("Not yet implemented"))
}
