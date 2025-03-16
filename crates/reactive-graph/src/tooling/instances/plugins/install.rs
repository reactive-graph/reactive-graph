use crate::tooling::instances::plugins::args::InstallPluginArgs;
use anyhow::Result;
use anyhow::anyhow;
use std::path::Path;

pub fn install_plugin(_instance_dir: &Path, _args: InstallPluginArgs) -> Result<()> {
    Err(anyhow!("Not yet implemented"))
}
