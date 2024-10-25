use crate::tooling::instances::provisioning::create_empty_file;
use crate::tooling::instances::provisioning::Chown;
use std::path::PathBuf;

pub const LOG_DIR_NAME: &str = "log";
pub const LOG_FILE_NAME: &str = "reactive-graph.log";
pub const ERROR_LOG_FILE_NAME: &str = "reactive-graph.error.log";

pub fn create_log_files(instance_dir: &PathBuf, chown: &Option<Chown>) -> anyhow::Result<()> {
    let log_dir = get_log_dir(instance_dir);
    let mut log_file = log_dir.to_owned();
    log_file.push(LOG_FILE_NAME);
    create_empty_file(&log_file, &chown)?;
    let mut error_log_file = log_dir.to_owned();
    error_log_file.push(ERROR_LOG_FILE_NAME);
    create_empty_file(&error_log_file, &chown)?;
    Ok(())
}

pub fn get_log_dir(instance_dir: &PathBuf) -> PathBuf {
    let mut log_dir = instance_dir.to_owned();
    log_dir.push(LOG_DIR_NAME);
    log_dir
}
