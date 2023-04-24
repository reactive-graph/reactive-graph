use crate::model::EntityType;

#[derive(Debug)]
pub enum EntityTypeProviderError {
    InitializationError,
}

pub trait EntityTypeProvider: Send + Sync {
    fn get_entity_types(&self) -> Vec<EntityType>;
}

#[macro_export]
macro_rules! entity_type_provider {
    ($entity_type_provider:expr) => {{
        let entity_type_provider = $entity_type_provider.clone();
        let entity_type_provider: Result<Arc<dyn EntityTypeProvider>, _> = <dyn query_interface::Object>::query_arc(entity_type_provider);
        if entity_type_provider.is_err() {
            return Err(EntityTypeProviderError::InitializationError);
        }
        Ok(entity_type_provider.ok())
    }};
}

#[macro_export]
macro_rules! entity_type_provider_impl {
    ($asset: ident, $path: expr $(, $format: expr)*) => {
        paste::paste! {
            use inexor_rgf_core_model::EntityType as ModelEntityType;

            #[derive(rust_embed::RustEmbed)]
            #[folder = $path]
            struct [<$asset EntityTypeAsset>];

            pub trait [<$asset EntityTypeProvider>]: $crate::EntityTypeProvider + Send + Sync {}

            #[derive(Clone)]
            pub struct [<$asset EntityTypeProviderImpl>] {}

            interfaces!([<$asset EntityTypeProviderImpl>]: dyn $crate::EntityTypeProvider);

            #[inexor_rgf_core_di::component]
            impl [<$asset EntityTypeProviderImpl>] {
                #[provides]
                fn new() -> Self {
                    Self {}
                }
            }

            #[inexor_rgf_core_di::provides]
            impl [<$asset EntityTypeProvider>] for [<$asset EntityTypeProviderImpl>] {}

            impl $crate::EntityTypeProvider for [<$asset EntityTypeProviderImpl>] {
                fn get_entity_types(&self) -> Vec<ModelEntityType> {
                    $crate::embedded_asset_provider_impl!([<$asset EntityTypeAsset>], ModelEntityType $(, $format)*)
                }
            }
        }
    };
}
