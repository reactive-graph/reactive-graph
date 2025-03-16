use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::hash_set::IntoIter;
use std::ops::Deref;
use std::ops::DerefMut;

use serde::Deserialize;
use serde::Serialize;
use serde::Serializer;

use reactive_graph_remotes_model::DEFAULT_ENDPOINT_DYNAMIC_GRAPH;
use reactive_graph_remotes_model::DEFAULT_ENDPOINT_GRAPHQL;
use reactive_graph_remotes_model::DEFAULT_ENDPOINT_PLUGIN;
use reactive_graph_remotes_model::DEFAULT_ENDPOINT_RUNTIME;
use reactive_graph_remotes_model::DEFAULT_USER_AGENT;
use reactive_graph_remotes_model::InstanceAddress;

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
            if address.user_agent == DEFAULT_USER_AGENT && address_2.user_agent != DEFAULT_USER_AGENT {
                address.user_agent = address_2.user_agent.clone();
            }
            if address.endpoint_graphql == DEFAULT_ENDPOINT_GRAPHQL && address_2.endpoint_graphql != DEFAULT_ENDPOINT_GRAPHQL {
                address.endpoint_graphql = address_2.endpoint_graphql.clone();
            }
            if address.endpoint_dynamic_graph == DEFAULT_ENDPOINT_DYNAMIC_GRAPH && address_2.endpoint_dynamic_graph != DEFAULT_ENDPOINT_DYNAMIC_GRAPH {
                address.endpoint_dynamic_graph = address_2.endpoint_dynamic_graph.clone();
            }
            if address.endpoint_runtime == DEFAULT_ENDPOINT_RUNTIME && address_2.endpoint_runtime != DEFAULT_ENDPOINT_RUNTIME {
                address.endpoint_runtime = address_2.endpoint_runtime.clone();
            }
            if address.endpoint_plugin == DEFAULT_ENDPOINT_PLUGIN && address_2.endpoint_plugin != DEFAULT_ENDPOINT_PLUGIN {
                address.endpoint_plugin = address_2.endpoint_plugin.clone();
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
