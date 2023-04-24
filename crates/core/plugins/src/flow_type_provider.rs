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
    ($asset: ident, $path: expr $(, $format: expr)*) => {
        paste::paste! {
            use inexor_rgf_core_model::FlowType as ModelFlowType;

            #[derive(rust_embed::RustEmbed)]
            #[folder = $path]
            struct [<$asset FlowTypeAsset>];

            pub trait [<$asset FlowTypeProvider>]: $crate::FlowTypeProvider + Send + Sync {}

            #[derive(Clone)]
            pub struct [<$asset FlowTypeProviderImpl>] {}

            interfaces!([<$asset FlowTypeProviderImpl>]: dyn $crate::FlowTypeProvider);

            #[inexor_rgf_core_di::component]
            impl [<$asset FlowTypeProviderImpl>] {
                #[provides]
                fn new() -> Self {
                    Self {}
                }
            }

            #[inexor_rgf_core_di::provides]
            impl [<$asset FlowTypeProvider>] for [<$asset FlowTypeProviderImpl>] {}

            impl $crate::FlowTypeProvider for [<$asset FlowTypeProviderImpl>] {
                fn get_flow_types(&self) -> Vec<ModelFlowType> {
                    $crate::embedded_asset_provider_impl!([<$asset FlowTypeAsset>], ModelFlowType $(, $format)*)
                }
            }
        }
    };
}
