use crate::InstanceAddress;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemotesConfig {
    remotes: Vec<InstanceAddress>,
}

impl RemotesConfig {
    pub fn new(remotes: Vec<InstanceAddress>) -> Self {
        RemotesConfig { remotes }
    }
}

impl IntoIterator for RemotesConfig {
    type Item = InstanceAddress;
    type IntoIter = std::vec::IntoIter<InstanceAddress>;

    fn into_iter(self) -> Self::IntoIter {
        self.remotes.into_iter()
    }
}
