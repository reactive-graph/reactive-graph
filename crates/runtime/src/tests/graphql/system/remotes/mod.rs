mod add;
mod get_all;
mod remove;
mod remove_all;
mod update;

mod util {
    use async_graphql::Variables;
    use serde_json::json;

    use crate::model_runtime::InstanceAddress;

    pub fn address_to_vars(address: &InstanceAddress) -> Variables {
        Variables::from_json(json!({
            "hostname": address.hostname,
            "port": address.port,
            "secure": address.secure
        }))
    }
}
