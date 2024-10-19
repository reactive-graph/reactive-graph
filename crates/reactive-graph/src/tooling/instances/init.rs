use crate::tooling::instances::args::InitInstanceArgs;
use crate::tooling::instances::create_dir::create_dir;
use crate::tooling::instances::repositories::args::InitRepositoryArgs;
use crate::tooling::instances::repositories::init::init_repository;
use std::path::PathBuf;
use std::process::exit;

pub fn init_instance(instance_dir: &PathBuf, _args: InitInstanceArgs) {
    create_dir(instance_dir, "config");
    create_dir(instance_dir, "logs");
    create_dir(instance_dir, "plugins/deploy");
    create_dir(instance_dir, "plugins/installed");
    init_repository(instance_dir, InitRepositoryArgs::default());

    eprintln!("Provision config files is not yet implemented");
    exit(0);
}
