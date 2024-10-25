use crate::tooling::instances::plugins::args::UninstallPluginArgs;
use anyhow::anyhow;
use anyhow::Result;
use std::path::Path;

pub fn uninstall_plugin(_instance_dir: &Path, _args: UninstallPluginArgs) -> Result<()> {
    Err(anyhow!("Not yet implemented"))
}
