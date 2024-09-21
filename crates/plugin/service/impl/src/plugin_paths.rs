use std::env::consts::DLL_EXTENSION;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

fn get_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

// TODO: replace relative with absolute path replacement
pub fn get_deploy_path(path: &Path) -> Option<PathBuf> {
    file_prefix(path).and_then(|file_prefix| {
        path.parent()
            .and_then(|path| path.parent())
            .map(|path| path.join("deploy").join(file_prefix).with_extension(DLL_EXTENSION))
    })
}

// TODO: replace relative with absolute path replacement
pub fn get_install_path(path: &Path) -> Option<PathBuf> {
    file_prefix(path).and_then(|file_prefix| {
        path.parent().and_then(|path| path.parent()).map(|path| {
            path.join("installed")
                .join(file_prefix)
                .with_extension(format!("{}.{}", get_timestamp(), DLL_EXTENSION))
        })
    })
}

pub fn get_stem(path: &Path) -> Option<String> {
    file_prefix(path).and_then(|stem| Some(stem.to_str()?.to_string()))
}

// Use this workaround until #![feature(path_file_prefix)] is stabilized
pub fn file_prefix(path: &Path) -> Option<&OsStr> {
    path.file_name().map(split_file_at_dot).map(|(before, _after)| before)
}

// Use this workaround until #![feature(path_file_prefix)] is stabilized
fn split_file_at_dot(file: &OsStr) -> (&OsStr, Option<&OsStr>) {
    let slice = os_str_as_u8_slice(file);
    if slice == b".." {
        return (file, None);
    }

    // The unsafety here stems from converting between &OsStr and &[u8]
    // and back. This is safe to do because (1) we only look at ASCII
    // contents of the encoding and (2) new &OsStr values are produced
    // only from ASCII-bounded slices of existing &OsStr values.
    let i = match slice[1..].iter().position(|b| *b == b'.') {
        Some(i) => i + 1,
        None => return (file, None),
    };
    let before = &slice[..i];
    let after = &slice[i + 1..];
    unsafe { (u8_slice_as_os_str(before), Some(u8_slice_as_os_str(after))) }
}

// Use this workaround until #![feature(path_file_prefix)] is stabilized
fn os_str_as_u8_slice(s: &OsStr) -> &[u8] {
    unsafe { &*(s as *const OsStr as *const [u8]) }
}

// Use this workaround until #![feature(path_file_prefix)] is stabilized
unsafe fn u8_slice_as_os_str(s: &[u8]) -> &OsStr {
    // SAFETY: see the comment of `os_str_as_u8_slice`
    unsafe { &*(s as *const [u8] as *const OsStr) }
}
