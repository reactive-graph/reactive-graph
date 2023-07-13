mod add;
mod fetch_remotes_from_all_remotes;
mod fetch_remotes_from_remote;
mod get_all;
mod remove;
mod remove_all;
mod update;
mod update_all;

mod util {
    use async_graphql::Variables;
    use serde_json::json;

    use crate::model_runtime::InstanceAddress;

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
