use std::cmp::Ordering;
use std::collections::hash_set::IntoIter;
use std::collections::HashSet;
use std::ops::Deref;
use std::ops::DerefMut;

use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;

use crate::InstanceAddress;
use crate::DEFAULT_ENDPOINT;
use crate::DEFAULT_USER_AGENT;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemotesConfig {
    #[serde(serialize_with = "sorted_address_set")]
    remotes: HashSet<InstanceAddress>,
}

impl RemotesConfig {
    pub fn new(remotes: HashSet<InstanceAddress>) -> Self {
        RemotesConfig { remotes }
    }

    pub fn merge(&mut self, mut address: InstanceAddress) {
        if let Some(address_2) = self.remotes.iter().find(|a| a == &&address) {
            if address.endpoint == DEFAULT_ENDPOINT && address_2.endpoint != DEFAULT_ENDPOINT {
                address.endpoint = address_2.endpoint.clone();
            }
            if address.user_agent == DEFAULT_USER_AGENT && address_2.user_agent != DEFAULT_USER_AGENT {
                address.user_agent = address_2.user_agent.clone();
            }
            if address.bearer.is_none() && address_2.bearer.is_some() {
                address.bearer = address_2.bearer.clone();
            }
            let _ = self.remotes.replace(address);
        }
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

fn sorted_address_set<S>(value: &HashSet<InstanceAddress>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut ordered: Vec<_> = value.iter().collect();
    ordered.sort_by(|a1, a2| match a1.hostname.cmp(&a2.hostname) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => a1.port.cmp(&a2.port),
    });
    ordered.serialize(serializer)
}
