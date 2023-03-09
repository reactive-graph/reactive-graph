use serde_json::Value;

use crate::entity::Operation;

pub trait Gate: Operation {
    fn rhs(&self, value: Value);
}
