// RAW and Dirty

// TODO: Move JsEngine to a plugin
// EntityTypes: JsScript
// EntityBehaviour: JsScript

use async_trait::async_trait;

#[async_trait]
pub trait JsEngine: Send + Sync {
    fn init(&self);

    fn run(&self);
}
