use serde_json::Value;

pub trait Operation {
    fn lhs(&self, value: Value);

    fn result(&self) -> Value;
}
