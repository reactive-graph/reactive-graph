use crate::PLUGIN_NAME_PREFIX;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct PluginDependency {
    /// The name of the dependency plugin.
    pub name: &'static str,

    /// The version of the dependency plugin.
    pub version: &'static str,
}

impl PluginDependency {
    pub fn new(name: &'static str, version: &'static str) -> Self {
        PluginDependency { name, version }
    }

    pub fn name_canonicalized(&self) -> String {
        self.name.replace(PLUGIN_NAME_PREFIX, "")
    }

    pub fn name_version(&self) -> String {
        format!("{}:{}", self.name_canonicalized(), self.version)
    }
}
