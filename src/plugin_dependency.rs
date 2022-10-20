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
}
