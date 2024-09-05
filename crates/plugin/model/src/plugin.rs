#[derive(Clone, Debug, Ord, Eq, PartialOrd, PartialEq)]
pub struct Plugin {
    pub name: String,
    pub short_name: String,
    pub state: String,
    pub version: String,
    pub plugin_api_version: String,
    pub rustc_version: String,
}
