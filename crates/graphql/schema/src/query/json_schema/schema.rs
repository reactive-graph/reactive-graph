use async_graphql::Error;
use async_graphql::Object;
use async_graphql::Result;
use schemars::Schema;
use serde_json::Map;
use serde_json::Value;

pub struct GraphQLJsonSchema {
    json_schema: Schema,
}

impl GraphQLJsonSchema {
    fn as_object(&self) -> Result<&Map<String, Value>> {
        self.json_schema.as_object().ok_or(Error::new("Invalid json schema"))
    }

    fn get_string(&self, key: &str) -> Result<String> {
        self.as_object()?
            .get(key)
            .and_then(|v| v.as_str().map(|v| v.to_owned()))
            .ok_or(Error::new(format!("Missing property {}", key)))
    }

    fn get_string_optional(&self, key: &str) -> Result<Option<String>> {
        match self.as_object()?.get(key) {
            Some(v) => match v.as_str() {
                Some(v) => Ok(Some(v.to_owned())),
                None => Err(Error::new(format!("Property {} is not a string", key))),
            },
            None => Ok(None),
        }
    }
    fn get_array(&self, key: &str) -> Result<Vec<Value>> {
        match self.as_object()?.get(key).ok_or(Error::new(format!("Missing property {}", key)))?.as_array() {
            Some(v) => Ok(v.clone()),
            None => Err(Error::new(format!("Property {} is not an array", key))),
        }
    }
}

#[Object(name = "JsonSchemaTyped")]
impl GraphQLJsonSchema {
    /// Returns the schema identifier of the JSON schema
    #[graphql(name = "_id")]
    async fn _id(&self) -> Result<String> {
        self.get_string("$id")
    }

    #[graphql(name = "_schema")]
    async fn _schema(&self) -> Result<String> {
        self.get_string("$schema")
    }

    #[graphql(name = "type")]
    async fn ty(&self) -> Result<String> {
        self.get_string("type")
    }

    async fn title(&self) -> Result<Option<String>> {
        self.get_string_optional("title")
    }

    async fn description(&self) -> Result<Option<String>> {
        self.get_string_optional("description")
    }

    async fn properties(&self) -> Result<Vec<Value>> {
        self.get_array("properties")
    }

    async fn required(&self) -> Result<Vec<Value>> {
        self.get_array("required")
    }
}

impl From<Schema> for GraphQLJsonSchema {
    fn from(json_schema: Schema) -> Self {
        GraphQLJsonSchema { json_schema }
    }
}
