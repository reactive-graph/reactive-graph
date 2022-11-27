use crate::model::RelationType;

#[derive(Debug)]
pub enum RelationTypeProviderError {
    InitializationError,
}

pub trait RelationTypeProvider: Send + Sync {
    fn get_relation_types(&self) -> Vec<RelationType>;
}

#[macro_export]
macro_rules! relation_type_provider {
    ($relation_type_provider:expr) => {{
        let relation_type_provider = $relation_type_provider.clone();
        let relation_type_provider: Result<Arc<dyn RelationTypeProvider>, _> = <dyn query_interface::Object>::query_arc(relation_type_provider);
        if relation_type_provider.is_err() {
            return Err(RelationTypeProviderError::InitializationError);
        }
        Ok(relation_type_provider.ok())
    }};
}

#[macro_export]
macro_rules! relation_type_provider_impl {
    ($asset: ident, $path: expr) => {
        paste::paste! {
            #[derive(rust_embed::RustEmbed)]
            #[folder = $path]
            struct [<$asset RelationTypeAsset>];

            pub trait [<$asset RelationTypeProvider>]: $crate::RelationTypeProvider + Send + Sync {}

            #[derive(Clone)]
            pub struct [<$asset RelationTypeProviderImpl>] {}

            interfaces!([<$asset RelationTypeProviderImpl>]: dyn $crate::RelationTypeProvider);

            #[inexor_rgf_core_di::component]
            impl [<$asset RelationTypeProviderImpl>] {
                #[provides]
                fn new() -> Self {
                    Self {}
                }
            }

            #[inexor_rgf_core_di::provides]
            impl [<$asset RelationTypeProvider>] for [<$asset RelationTypeProviderImpl>] {}

            impl $crate::RelationTypeProvider for [<$asset RelationTypeProviderImpl>] {
                fn get_relation_types(&self) -> Vec<inexor_rgf_core_model::RelationType> {
                    $crate::embedded_asset_provider_impl!([<$asset RelationTypeAsset>], RelationType)
                }
            }
        }
    };
}
