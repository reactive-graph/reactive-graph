use crate::tooling::instances::plugins::args::UninstallPluginArgs;
use anyhow::anyhow;
use anyhow::Result;
use std::path::PathBuf;

pub fn uninstall_plugin(_instance_dir: &PathBuf, _args: UninstallPluginArgs) -> Result<()> {
    Err(anyhow!("Not yet implemented"))
}
