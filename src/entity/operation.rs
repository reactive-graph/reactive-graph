use serde_json::Value;

use crate::model::ReactiveEntityInstance;
use crate::Behaviour;

pub trait Operation: Behaviour<ReactiveEntityInstance> {
    fn lhs(&self, value: Value);

    fn result(&self) -> Value;
}
