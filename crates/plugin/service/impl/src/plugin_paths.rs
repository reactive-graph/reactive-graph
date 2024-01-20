use std::env::consts::DLL_EXTENSION;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

fn get_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

// TODO: replace relative with absolute path replacement
pub fn get_deploy_path(path: &Path) -> Option<PathBuf> {
    path.file_prefix().and_then(|file_prefix| {
        path.parent()
            .and_then(|path| path.parent())
            .map(|path| path.join("deploy").join(file_prefix).with_extension(DLL_EXTENSION))
    })
}

// TODO: replace relative with absolute path replacement
pub fn get_install_path(path: &Path) -> Option<PathBuf> {
    path.file_prefix().and_then(|file_prefix| {
        path.parent().and_then(|path| path.parent()).map(|path| {
            path.join("installed")
                .join(file_prefix)
                .with_extension(format!("{}.{}", get_timestamp(), DLL_EXTENSION))
        })
    })
}

pub fn get_stem(path: &Path) -> Option<String> {
    path.file_prefix().and_then(|stem| Some(stem.to_str()?.to_string()))
}
