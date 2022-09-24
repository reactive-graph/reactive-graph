use serde_json::Value;

pub trait ReactivePropertyContainer {
    fn tick(&self);

    fn add_property<S: Into<String>>(&self, name: S, value: Value);
}
