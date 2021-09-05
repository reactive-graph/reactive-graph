// TODO: Move to plugin inexor-rgf-plugin-system

use async_trait::async_trait;

use crate::api::Lifecycle;

#[async_trait]
pub trait SystemConstantsInitializer: Send + Sync + Lifecycle {}
