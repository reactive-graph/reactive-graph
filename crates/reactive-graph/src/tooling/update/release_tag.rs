use crate::tooling::update::args::UpdateArgs;
use self_update::cargo_crate_version;
use std::fmt::Display;
use std::fmt::Formatter;

pub const RELEASE_TAG_NIGHTLY: &str = "nightly";
pub const RELEASE_TAG_LATEST: &str = "latest";

#[derive(Debug)]
pub enum ReleaseTag {
    Nightly,
    Latest,
    Current,
    Version(String),
}

impl ReleaseTag {
    pub fn bin_path_in_archive(&self, current_bin_name: &str) -> String {
        match self {
            ReleaseTag::Nightly | ReleaseTag::Latest => format!("reactive-graph-{{{{ version }}}}-{{{{ target }}}}/{current_bin_name}"),
            ReleaseTag::Current | ReleaseTag::Version(_) => format!("reactive-graph-v{{{{ version }}}}-{{{{ target }}}}/{current_bin_name}"),
        }
    }

    pub fn target_version_tag(&self) -> String {
        prefix_version(match self {
            ReleaseTag::Nightly | ReleaseTag::Latest => RELEASE_TAG_NIGHTLY,
            ReleaseTag::Current => cargo_crate_version!(),
            ReleaseTag::Version(version) => version,
        })
    }
}

impl From<&UpdateArgs> for ReleaseTag {
    fn from(args: &UpdateArgs) -> Self {
        if args.nightly.unwrap_or_default() {
            return ReleaseTag::Nightly;
        }
        if args.latest.unwrap_or_default() {
            return ReleaseTag::Latest;
        }
        if args.current.unwrap_or_default() {
            return ReleaseTag::Current;
        }
        if let Some(version) = &args.version {
            return ReleaseTag::Version(prefix_version(version));
        }
        ReleaseTag::Latest
    }
}

impl Display for ReleaseTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ReleaseTag::Nightly => RELEASE_TAG_NIGHTLY,
                ReleaseTag::Latest => RELEASE_TAG_LATEST,
                ReleaseTag::Current => cargo_crate_version!(),
                ReleaseTag::Version(version) => &version,
            }
        )
    }
}

fn prefix_version(version: &str) -> String {
    if version.starts_with("v") || version == RELEASE_TAG_NIGHTLY {
        version.to_string()
    } else {
        format!("v{version}")
    }
}
