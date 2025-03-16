use crate::tooling::instances::certificates::KEYS_DIR_NAME;
use crate::tooling::instances::certificates::generate_certificate;
use crate::tooling::instances::certificates::get_keys_dir;
use crate::tooling::instances::config::CONFIG_DIR_NAME;
use crate::tooling::instances::config::get_config_dir;
use crate::tooling::instances::init::args::InitInstanceArgs;
use crate::tooling::instances::logging::LOG_DIR_NAME;
use crate::tooling::instances::logging::create_log_files;
use crate::tooling::instances::provisioning::create_dir;
use crate::tooling::instances::provisioning::write_file;
use crate::tooling::instances::repositories::args::InitRepositoryArgs;
use crate::tooling::instances::repositories::init::init_repository;
use anyhow::Result;
use include_dir::Dir;
use include_dir::include_dir;
use std::path::PathBuf;

pub mod args;

static CONFIG_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../config");

pub fn init_instance(instance_dir: &PathBuf, args: InitInstanceArgs) -> Result<()> {
    let chown = args.chown.get_chown();
    create_dir(instance_dir, CONFIG_DIR_NAME, &chown)?;
    create_dir(instance_dir, LOG_DIR_NAME, &chown)?;
    create_dir(instance_dir, KEYS_DIR_NAME, &chown)?;
    create_dir(instance_dir, "plugins/deploy", &chown)?;
    create_dir(instance_dir, "plugins/installed", &chown)?;
    create_dir(instance_dir, "repositories", &chown)?;

    // Provisioning default repository
    let repository_args = InitRepositoryArgs::default().chown(args.chown);
    init_repository(instance_dir, repository_args)?;

    // Provisioning config files
    let config_dir = get_config_dir(instance_dir);
    for config_file in CONFIG_DIR.files() {
        let path = config_file.path();
        if let (Some(filename), Some(contents)) = (path.file_name(), config_file.contents_utf8()) {
            write_file(&config_dir, filename, contents, &chown)?;
        }
    }

    // Provisioning log files
    create_log_files(instance_dir, &chown)?;

    // Provisioning keys
    let keys_dir = get_keys_dir(instance_dir);
    generate_certificate(&keys_dir, args.certificate)?;

    // TODO: Provisioning standard library plugins

    // exit(0);
    Ok(())
}
