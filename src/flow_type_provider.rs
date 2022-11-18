use crate::model::FlowType;

#[derive(Debug)]
pub enum FlowTypeProviderError {
    InitializationError,
}

pub trait FlowTypeProvider: Send + Sync {
    fn get_flow_types(&self) -> Vec<FlowType>;
}

#[macro_export]
macro_rules! flow_type_provider {
    ($flow_type_provider:expr) => {{
        let flow_type_provider = $flow_type_provider.clone();
        let flow_type_provider: Result<Arc<dyn FlowTypeProvider>, _> = <dyn query_interface::Object>::query_arc(flow_type_provider);
        if flow_type_provider.is_err() {
            return Err(FlowTypeProviderError::InitializationError);
        }
        Ok(flow_type_provider.ok())
    }};
}

#[macro_export]
macro_rules! flow_type_provider_impl {
    ($asset: ident, $path: expr) => {
        paste! {
            #[derive(RustEmbed)]
            #[folder = $path]
            struct [<$asset FlowTypeAsset>];

            pub trait [<$asset FlowTypeProvider>]: FlowTypeProvider + Send + Sync {}

            #[derive(Clone)]
            pub struct [<$asset FlowTypeProviderImpl>] {}

            interfaces!([<$asset FlowTypeProviderImpl>]: dyn FlowTypeProvider);

            #[component]
            impl [<$asset FlowTypeProviderImpl>] {
                #[provides]
                fn new() -> Self {
                    Self {}
                }
            }

            #[provides]
            impl [<$asset FlowTypeProvider>] for [<$asset FlowTypeProviderImpl>] {}

            impl FlowTypeProvider for [<$asset FlowTypeProviderImpl>] {
                fn get_flow_types(&self) -> Vec<FlowType> {
                    embedded_asset_provider_impl!([<$asset FlowTypeAsset>], FlowType)
                }
            }
        }
    };
}
