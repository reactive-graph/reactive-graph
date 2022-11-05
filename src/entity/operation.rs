use serde_json::Value;

use crate::Behaviour;

pub trait Operation: Behaviour {
    fn lhs(&self, value: Value);

    fn result(&self) -> Value;
}
