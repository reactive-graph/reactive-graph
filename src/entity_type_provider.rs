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
    ($asset: ident, $path: expr) => {
        paste! {
            #[derive(RustEmbed)]
            #[folder = $path]
            struct [<$asset EntityTypeAsset>];

            pub trait [<$asset EntityTypeProvider>]: EntityTypeProvider + Send + Sync {}

            #[derive(Clone)]
            pub struct [<$asset EntityTypeProviderImpl>] {}

            interfaces!([<$asset EntityTypeProviderImpl>]: dyn EntityTypeProvider);

            #[component]
            impl [<$asset EntityTypeProviderImpl>] {
                #[provides]
                fn new() -> Self {
                    Self {}
                }
            }

            #[provides]
            impl [<$asset EntityTypeProvider>] for [<$asset EntityTypeProviderImpl>] {}

            impl EntityTypeProvider for [<$asset EntityTypeProviderImpl>] {
                fn get_entity_types(&self) -> Vec<EntityType> {
                    embedded_asset_provider_impl!([<$asset EntityTypeAsset>], EntityType)
                }
            }
        }
    };
}
