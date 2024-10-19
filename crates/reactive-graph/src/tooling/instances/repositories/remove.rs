use crate::tooling::instances::repositories::args::DeleteRepositoryArgs;
use std::path::PathBuf;
use std::process::exit;

pub fn remove_repository(instance_dir: &PathBuf, args: DeleteRepositoryArgs) {
    if args.local_name == "default" && !args.force.unwrap_or(false) {
        eprintln!("The default repository cannot be removed!");
        exit(1);
    }
    let mut repository_dir = instance_dir.clone();
    repository_dir.push(&args.local_name);
    match std::fs::remove_dir_all(&repository_dir) {
        Ok(_) => {
            println!("Removed repository {} from {}", args.local_name, repository_dir.to_string_lossy());
            exit(0);
        }
        Err(e) => {
            eprintln!("Cannot remove repository {} from {}: {}", args.local_name, repository_dir.to_string_lossy(), e);
            exit(1);
        }
    }
}
