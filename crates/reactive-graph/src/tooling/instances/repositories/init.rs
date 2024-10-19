use crate::tooling::instances::create_dir::create_dir;
use crate::tooling::instances::repositories::args::InitRepositoryArgs;
use std::path::PathBuf;
use std::process::exit;

pub fn init_repository(instance_dir: &PathBuf, args: InitRepositoryArgs) {
    let mut repository_dir = instance_dir.clone();
    repository_dir.push(args.local_name);
    create_dir(&repository_dir, "types/components");
    create_dir(&repository_dir, "types/entities");
    create_dir(&repository_dir, "types/relations");
    create_dir(&repository_dir, "types/flows");
    create_dir(&repository_dir, "instances/entities");
    create_dir(&repository_dir, "instances/relations");
    create_dir(&repository_dir, "instances/flows");
    exit(0);
}
