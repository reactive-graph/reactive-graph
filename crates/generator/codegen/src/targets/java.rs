use crate::targets::CodeGenerationTarget;
use semver::VersionReq;

pub struct Java {}

impl CodeGenerationTarget for Java {
    fn name() -> String {
        "java".to_string()
    }

    fn version() -> VersionReq {
        VersionReq::parse(">=21").unwrap()
    }

    fn extension() -> String {
        "java".to_string()
    }
}
