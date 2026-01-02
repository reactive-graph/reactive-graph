use crate::r_lowercase_string;
use std::env::temp_dir;
use std::path::PathBuf;

pub fn r_temp_dir() -> PathBuf {
    temp_dir().join(r_lowercase_string())
}
