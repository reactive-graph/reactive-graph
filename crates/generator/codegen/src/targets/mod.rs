use semver::VersionReq;
use std::fmt::Display;
use std::fmt::Formatter;

pub mod java;
pub mod rust;

#[non_exhaustive]
#[derive(Debug)]
pub enum CodeGenerationTargets {
    Rust,
    RustReactive,
    RustReactiveManager,
    RustClient,
    RustClientManager,
    Proto,
    JsonSchema,
    TypeScript,
    Java,
}

impl Display for CodeGenerationTargets {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait CodeGenerationTarget {
    /// Returns the name of the target (for example: `rust`).
    fn name() -> String;

    /// Returns the version requirements of the language (for example: `>=1.0.0`).
    fn version() -> VersionReq;

    /// The default file extension of the code files (for example: `rs`).
    fn extension() -> String;
}
