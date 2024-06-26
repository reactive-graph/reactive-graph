use reactive_graph_graph::FlowInstance;

#[derive(Debug)]
pub enum FlowInstanceProviderError {
    InitializationError,
}

pub trait FlowInstanceProvider: Send + Sync {
    fn get_flow_instances(&self) -> Vec<FlowInstance>;
}

#[macro_export]
macro_rules! flow_instance_provider {
    ($flow_instance_provider:expr) => {{
        let flow_instance_provider = $flow_instance_provider.clone();
        let flow_instance_provider: Result<Arc<dyn FlowInstanceProvider>, _> = <dyn query_interface::Object>::query_arc(flow_instance_provider);
        if flow_instance_provider.is_err() {
            return Err(FlowInstanceProviderError::InitializationError);
        }
        Ok(flow_instance_provider.ok())
    }};
}

#[macro_export]
macro_rules! flow_instance_provider_impl {
    ($asset: ident, $path: expr) => {
        paste::paste! {
            use reactive_graph_graph::FlowInstance as ModelFlowInstance;

            #[derive(reactive_graph_plugin_api::RustEmbed)]
            #[folder = $path]
            struct [<$asset FlowInstanceAsset>];

            pub trait [<$asset FlowInstanceProvider>]: $crate::FlowInstanceProvider + Send + Sync {}

            #[derive(Clone)]
            pub struct [<$asset FlowInstanceProviderImpl>] {}

            interfaces!([<$asset FlowInstanceProviderImpl>]: dyn $crate::FlowInstanceProvider);

            #[reactive_graph_core_di::component]
            impl [<$asset FlowInstanceProviderImpl>] {
                #[provides]
                fn new() -> Self {
                    Self {}
                }
            }

            #[reactive_graph_core_di::provides]
            impl [<$asset FlowInstanceProvider>] for [<$asset FlowInstanceProviderImpl>] {}

            impl $crate::FlowInstanceProvider for [<$asset FlowInstanceProviderImpl>] {
                fn get_flow_instances(&self) -> Vec<ModelFlowInstance> {
                    $crate::embedded_asset_provider_impl!([<$asset FlowInstanceAsset>], ModelFlowInstance)
                }
            }
        }
    };
}
