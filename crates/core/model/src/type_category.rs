use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeCategory(String);

impl TypeCategory {
    pub fn new(category: &str) -> TypeCategory {
        TypeCategory(String::from(category))
    }
}

impl ToString for TypeCategory {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
