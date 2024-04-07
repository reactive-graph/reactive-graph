mod mutation;
mod query;

mod util {
    use async_graphql::Variables;
    use serde_json::json;

    use reactive_graph_remotes_model::InstanceAddress;

    pub fn address_to_vars(address: &InstanceAddress) -> Variables {
        Variables::from_json(json!({
            "address": {
                "hostname": address.hostname,
                "port": address.port,
                "secure": address.secure
            }
        }))
    }
}
