use serde_json::Value;

pub trait Disconnectable {
    fn disconnect(&self);
}

pub trait Operation: Disconnectable {
    fn lhs(&self, value: Value);

    fn result(&self) -> Value;
}

pub trait BehaviourType {
    fn type_name(&self) -> String;
}
