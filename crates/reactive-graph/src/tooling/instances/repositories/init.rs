use crate::tooling::instances::provisioning::create_dir;
use crate::tooling::instances::repositories::args::InitRepositoryArgs;
use anyhow::anyhow;
use anyhow::Result;
use std::path::Path;

pub fn init_repository(instance_dir: &Path, args: InitRepositoryArgs) -> Result<()> {
    let chown = args.chown.get_chown();
    let mut repository_dir = instance_dir.to_owned();
    repository_dir.push(format!("repositories/{}", args.local_name));
    match args.url {
        None => {
            create_dir(&repository_dir, "types/components", &chown)?;
            create_dir(&repository_dir, "types/entities", &chown)?;
            create_dir(&repository_dir, "types/relations", &chown)?;
            create_dir(&repository_dir, "types/flows", &chown)?;
            create_dir(&repository_dir, "instances/entities", &chown)?;
            create_dir(&repository_dir, "instances/relations", &chown)?;
            create_dir(&repository_dir, "instances/flows", &chown)?;
            Ok(())
        }
        Some(_url) => {
            // TODO: git clone
            // TODO: verify
            Err(anyhow!("Not yet implemented"))
        }
    }
}
