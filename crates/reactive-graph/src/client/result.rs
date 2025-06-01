use crate::client::error::CommandError;
use crate::client::error::CommandError::NotImplemented;
use crate::shared::output_format::OutputFormatArgs;
use reactive_graph_serde::error::SerializationError;
use reactive_graph_table_model::container::DefaultTableContainer;
use reactive_graph_table_model::container::TableContainer;
use reactive_graph_table_model::container::TableInlineFormatSetter;
use reactive_graph_table_model::container::TableOptions;
use serde::Serialize;
use serde_json::Value;
use std::fmt::Display;
use std::fmt::Formatter;
use std::marker::PhantomData;
use tabled::Tabled;
use toml::map::Map;

pub enum CommandResponse {
    Message(String),
    Value(Value),
    #[cfg(feature = "toml")]
    TomlValue(toml::Value),
    Table(Box<dyn TableContainer>),
}

pub type CommandResult = Result<CommandResponse, CommandError>;

impl From<String> for CommandResponse {
    fn from(message: String) -> Self {
        CommandResponse::Message(message)
    }
}

impl From<&str> for CommandResponse {
    fn from(message: &str) -> Self {
        CommandResponse::Message(message.to_string())
    }
}

impl From<Value> for CommandResponse {
    fn from(value: Value) -> Self {
        CommandResponse::Value(value)
    }
}

#[cfg(feature = "toml")]
impl From<toml::Value> for CommandResponse {
    fn from(value: toml::Value) -> Self {
        CommandResponse::TomlValue(value)
    }
}

impl<S: 'static, T: Clone + Tabled + From<S> + TableInlineFormatSetter + 'static, O: TableOptions + 'static> From<DefaultTableContainer<S, T, O>>
    for CommandResponse
{
    fn from(t: DefaultTableContainer<S, T, O>) -> Self {
        CommandResponse::Table(t.into_boxed())
    }
}

impl Display for CommandResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandResponse::Message(message) => write!(f, "{message}"),
            CommandResponse::Value(value) => write!(f, "{}", serde_json::to_string_pretty(&value).unwrap_or_default()),
            #[cfg(feature = "toml")]
            CommandResponse::TomlValue(value) => write!(f, "{}", toml::to_string_pretty(&value).unwrap_or_default()),
            CommandResponse::Table(table) => write!(f, "{table}"),
        }
    }
}

pub(crate) enum CommandResultBuilderContent<S: Serialize> {
    Single(S),
    Collection(Vec<S>),
}

pub(crate) struct CommandResultBuilder<S: Serialize, T: Clone + Tabled + From<S>, O: TableOptions> {
    object_or_collection: CommandResultBuilderContent<S>,
    output_format: Option<OutputFormatArgs>,
    table_type: PhantomData<T>,
    table_options: PhantomData<O>,
}

impl<S: Serialize + 'static, T: Clone + Tabled + From<S> + TableInlineFormatSetter + 'static, O: TableOptions + 'static> CommandResultBuilder<S, T, O> {
    pub(crate) fn single(single_object: S, output_format: Option<OutputFormatArgs>) -> Self {
        Self {
            object_or_collection: CommandResultBuilderContent::Single(single_object),
            output_format,
            table_type: Default::default(),
            table_options: Default::default(),
        }
    }

    pub(crate) fn collection(collection: Vec<S>, output_format: Option<OutputFormatArgs>) -> Self {
        CommandResultBuilder {
            object_or_collection: CommandResultBuilderContent::Collection(collection),
            output_format,
            table_type: Default::default(),
            table_options: Default::default(),
        }
    }

    pub(crate) fn into_command_result(self) -> CommandResult {
        match self.output_format.clone().unwrap_or_default() {
            OutputFormatArgs::Table => match self.object_or_collection {
                CommandResultBuilderContent::Single(single_object) => Ok(DefaultTableContainer::<S, T, O>::from(single_object).into()),
                CommandResultBuilderContent::Collection(collection) => Ok(DefaultTableContainer::<S, T, O>::from(collection).into()),
            },
            OutputFormatArgs::HtmlTable => match self.object_or_collection {
                CommandResultBuilderContent::Single(single_object) => Ok(DefaultTableContainer::<S, T, O>::from(single_object).into_html_table().into()),
                CommandResultBuilderContent::Collection(collection) => Ok(DefaultTableContainer::<S, T, O>::from(collection).into_html_table().into()),
            },
            OutputFormatArgs::MarkdownTable => match self.object_or_collection {
                CommandResultBuilderContent::Single(single_object) => Ok(DefaultTableContainer::<S, T, O>::from(single_object).into_markdown_table().into()),
                CommandResultBuilderContent::Collection(collection) => Ok(DefaultTableContainer::<S, T, O>::from(collection).into_markdown_table().into()),
            },
            OutputFormatArgs::Count => match self.object_or_collection {
                CommandResultBuilderContent::Single(_) => Ok("1 result".into()),
                CommandResultBuilderContent::Collection(collection) => Ok(format!("{} result(s)", collection.len()).into()),
            },
            OutputFormatArgs::Json => match self.object_or_collection {
                CommandResultBuilderContent::Single(single_object) => Ok(serde_json::to_value(single_object)
                    .map_err(|e| CommandError::SerializationError(SerializationError::Json(e)))?
                    .into()),
                CommandResultBuilderContent::Collection(collection) => Ok(serde_json::to_value(collection)
                    .map_err(|e| CommandError::SerializationError(SerializationError::Json(e)))?
                    .into()),
            },
            #[cfg(feature = "json5")]
            OutputFormatArgs::Json5 => Err(NotImplemented),
            #[cfg(feature = "toml")]
            OutputFormatArgs::Toml => match self.object_or_collection {
                CommandResultBuilderContent::Single(single_object) => Ok(toml::Value::try_from(single_object)
                    .map_err(|e| CommandError::SerializationError(SerializationError::Toml(e)))?
                    .into()),
                CommandResultBuilderContent::Collection(collection) => {
                    let inner = toml::Value::try_from(collection).map_err(|e| CommandError::SerializationError(SerializationError::Toml(e)))?;
                    let mut map: Map<String, toml::Value> = Map::new();
                    let type_name = std::any::type_name::<S>().rsplit_once("::").unwrap_or_default().1.to_string();
                    map.insert(type_name, inner);
                    let table = toml::Value::Table(map);
                    Ok(table.into())
                }
            },
        }
    }
}
