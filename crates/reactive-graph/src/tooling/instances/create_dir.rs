use std::fs::create_dir_all;
use std::path::PathBuf;

pub fn create_dir<S: Into<String>>(working_dir: &PathBuf, sub_dir: S) -> PathBuf {
    let mut target_dir = working_dir.clone();
    target_dir.push(sub_dir.into());
    match create_dir_all(&target_dir) {
        Ok(_) => println!("Created {}", target_dir.to_string_lossy()),
        Err(e) => println!("Failed to create {}: {}", target_dir.to_string_lossy(), e),
    }
    target_dir
}
