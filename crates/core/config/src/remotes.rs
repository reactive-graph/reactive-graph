use std::collections::hash_set::IntoIter;
use std::collections::HashSet;
use std::ops::Deref;
use std::ops::DerefMut;

use serde::Deserialize;
use serde::Serialize;

use crate::InstanceAddress;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemotesConfig {
    remotes: HashSet<InstanceAddress>,
}

impl RemotesConfig {
    pub fn new(remotes: HashSet<InstanceAddress>) -> Self {
        RemotesConfig { remotes }
    }
}

impl IntoIterator for RemotesConfig {
    type Item = InstanceAddress;
    type IntoIter = IntoIter<InstanceAddress>;

    fn into_iter(self) -> Self::IntoIter {
        self.remotes.into_iter()
    }
}

impl Deref for RemotesConfig {
    type Target = HashSet<InstanceAddress>;

    fn deref(&self) -> &Self::Target {
        &self.remotes
    }
}

impl DerefMut for RemotesConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.remotes
    }
}
