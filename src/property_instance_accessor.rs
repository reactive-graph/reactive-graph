use serde_json::Value;

pub trait PropertyInstanceGetter {
    /// Returns the json value of the given property by name
    fn get<S: Into<String>>(&self, property_name: S) -> Option<Value>;

    /// Returns the boolean value of the given property by name
    fn as_bool<S: Into<String>>(&self, property_name: S) -> Option<bool>;

    /// Returns the u64 value of the given property by name
    fn as_u64<S: Into<String>>(&self, property_name: S) -> Option<u64>;

    /// Returns the i64 value of the given property by name
    fn as_i64<S: Into<String>>(&self, property_name: S) -> Option<i64>;

    /// Returns the f64 value of the given property by name
    fn as_f64<S: Into<String>>(&self, property_name: S) -> Option<f64>;

    /// Returns the string value of the given property by name
    fn as_string<S: Into<String>>(&self, property_name: S) -> Option<String>;
}

pub trait MutablePropertyInstanceSetter: PropertyInstanceGetter {
    /// Sets the value of the given property by name
    fn set<S: Into<String>>(&mut self, property_name: S, value: Value);
}

pub trait PropertyInstanceSetter: PropertyInstanceGetter {
    /// Sets the value of the given property by name
    fn set<S: Into<String>>(&self, property_name: S, value: Value);

    fn set_no_propagate<S: Into<String>>(&self, property_name: S, value: Value);
}
