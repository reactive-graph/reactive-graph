pub mod completions;

#[cfg(target_os = "linux")]
pub mod manpages;

pub mod args;
pub mod config;
pub mod info;
pub mod markdown_help;

pub mod output_format;
