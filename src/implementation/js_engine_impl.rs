use async_trait::async_trait;
use waiter_di::*;

use crate::api::{JsEngine, ReactiveEntityInstanceManager};

#[component]
pub struct JsEngineImpl {
    reactive_entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,
}

#[async_trait]
#[provides]
impl JsEngine for JsEngineImpl {
    fn init(&self) {}

    fn run(&self) {}
}
