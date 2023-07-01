#[cynic::schema_for_derives(file = r#"schema.graphql"#, module = "crate::schema::schema")]
pub mod mapping {
    use crate::schema::system::instance::InstanceInfo;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct GetInstanceInfo {
        pub system: GetInstanceInfoSystem,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "System")]
    pub struct GetInstanceInfoSystem {
        pub instance_info: InstanceInfo,
    }
}

pub mod queries {
    use crate::client::system::instance::mapping::GetInstanceInfo;

    pub fn get_instance_info() -> cynic::Operation<GetInstanceInfo, ()> {
        use cynic::QueryBuilder;
        GetInstanceInfo::build(())
    }
}

pub mod api {
    use std::sync::Arc;

    use crate::client::system::instance::queries::get_instance_info;
    use crate::model_runtime::InstanceInfo;
    use crate::InexorRgfClient;
    use crate::InexorRgfClientExecutionError;

    pub struct Instance {
        client: Arc<InexorRgfClient>,
    }

    impl Instance {
        pub fn new(client: Arc<InexorRgfClient>) -> Self {
            Self { client }
        }

        pub async fn get_instance_info(&self) -> Result<InstanceInfo, InexorRgfClientExecutionError> {
            self.client.run_graphql(get_instance_info(), |data| data.system.instance_info.into()).await
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::InexorRgfClient;
    use inexor_rgf_rt::runtime::Runtime;
    use inexor_rgf_rt::runtime::RuntimeBuilder;
    use std::sync::Arc;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_instance_info() {
        RuntimeBuilder::new()
            .ignore_config_files()
            .disable_all_plugins(true)
            .instance_name("iojasdf")
            .instance_description("oijasdfnmei")
            .pick_free_port()
            .init()
            .await
            .post_init()
            .await
            .spawn()
            .await
            .with_runtime(|runtime: Arc<dyn Runtime>| async move {
                // Get instance info from the runtime
                let rt_instance_info = runtime.get_instance_service().get_instance_info();
                let rt_address = rt_instance_info.address();

                // Create a client
                let client = InexorRgfClient::new(rt_address.clone()).expect("Cannot create client");

                // Fetch instance info via client & graphql
                let instance_info = client.system().instance().get_instance_info().await.expect("Cannot fetch instance info");

                assert_eq!(instance_info.hostname, rt_address.hostname);
                assert_eq!(u16::try_from(instance_info.port).expect("invalid port number"), rt_address.port);
                assert_eq!(instance_info.secure, rt_address.secure);
                assert_eq!(instance_info.name, rt_instance_info.name);
                assert_eq!(instance_info, rt_instance_info);
            })
            .await
            .pre_shutdown()
            .await
            .shutdown()
            .await;
    }
}
