use anyhow::Result;
use std::fs::create_dir_all;
use std::fs::write;
use std::fs::OpenOptions;
use std::path::Path;
use std::path::PathBuf;

pub struct Chown {
    pub uid: u32,
    pub gid: u32,
}

impl Chown {
    pub fn new(uid: u32, gid: u32) -> Self {
        Self { uid, gid }
    }
}

pub fn create_dir<S: Into<String>>(working_dir: &Path, sub_dir: S, chown: &Option<Chown>) -> Result<PathBuf> {
    let mut target_dir = working_dir.to_owned();
    target_dir.push(sub_dir.into());
    match create_dir_all(&target_dir) {
        Ok(_) => {
            println!("Created {}", target_dir.to_string_lossy());
            #[cfg(target_os = "linux")]
            if let Some(chown) = chown {
                use std::os::unix::fs;
                fs::chown(&target_dir, Some(chown.uid), Some(chown.gid))?;
            }
            Ok(target_dir)
        }
        Err(e) => {
            eprintln!("Failed to create {}: {}", target_dir.to_string_lossy(), e);
            Err(e.into())
        }
    }
}

pub fn write_file<P: Into<PathBuf>>(target_dir: &Path, file: P, content: &str, chown: &Option<Chown>) -> Result<PathBuf> {
    let mut target_file = target_dir.to_owned();
    target_file.push(file.into());
    match write(&target_file, content) {
        Ok(_) => {
            println!("Wrote file {}", target_file.to_string_lossy());
            #[cfg(target_os = "linux")]
            if let Some(chown) = chown {
                use std::os::unix::fs;
                fs::chown(&target_file, Some(chown.uid), Some(chown.gid))?;
            }
            Ok(target_file)
        }
        Err(e) => {
            eprintln!("Failed to write file {}: {}", target_file.to_string_lossy(), e);
            Err(e.into())
        }
    }
}

pub fn create_empty_file(target_file: &Path, chown: &Option<Chown>) -> Result<()> {
    match OpenOptions::new().create(true).truncate(false).write(true).open(target_file) {
        Ok(_file) => {
            println!("Created file {}", target_file.to_string_lossy());
            #[cfg(target_os = "linux")]
            if let Some(chown) = chown {
                use std::os::unix::fs;
                fs::chown(target_file, Some(chown.uid), Some(chown.gid))?;
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to create file {}: {}", target_file.to_string_lossy(), e);
            Err(e.into())
        }
    }
}
