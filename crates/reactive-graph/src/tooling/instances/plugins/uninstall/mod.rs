use crate::tooling::instances::plugins::uninstall::args::UninstallPluginArgs;
use anyhow::Result;
use anyhow::anyhow;
use std::path::Path;

pub mod args;

pub fn uninstall_plugin(_instance_dir: &Path, _args: UninstallPluginArgs) -> Result<()> {
    Err(anyhow!("Not yet implemented"))
}
