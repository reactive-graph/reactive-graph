use crate::tooling::instances::repositories::args::DeleteRepositoryArgs;
use anyhow::anyhow;
use std::path::Path;

pub fn remove_repository(instance_dir: &Path, args: DeleteRepositoryArgs) -> anyhow::Result<()> {
    if args.local_name == "default" && !args.force.unwrap_or(false) {
        return Err(anyhow!("The default repository cannot be removed!"));
    }
    let mut repository_dir = instance_dir.to_owned();
    repository_dir.push(&args.local_name);
    match std::fs::remove_dir_all(&repository_dir) {
        Ok(_) => {
            println!("Removed repository {} from {}", args.local_name, repository_dir.to_string_lossy());
            Ok(())
        }
        Err(e) => {
            eprintln!("Cannot remove repository {} from {}: {}", args.local_name, repository_dir.to_string_lossy(), e);
            Err(e.into())
        }
    }
}
