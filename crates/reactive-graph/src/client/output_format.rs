use crate::client::result::CommandResult;
use crate::client::result::CommandResultBuilder;
use reactive_graph_table_model::container::TableInlineFormatSetter;
use reactive_graph_table_model::container::TableOptions;

use serde::Serialize;
use std::marker::PhantomData;
use tabled::Tabled;

#[derive(clap::ValueEnum, Default, Debug, Clone, Serialize)]
pub(crate) enum OutputFormatArgs {
    // The output is formatted as a table.
    #[default]
    Table,
    // The output is formatted as a HTML table.
    HtmlTable,
    // The output is formatted as a Markdown table.
    MarkdownTable,
    Count,
    // The output is returned as JSON.
    Json,
    // The output is returned as JSON5.
    Json5,
    // The output is returned as TOML.
    Toml,
}

pub(crate) struct OutputFormatWrapper<S: Serialize, T: Clone + Tabled + From<S> + TableInlineFormatSetter, O: TableOptions>(
    pub Option<OutputFormatArgs>,
    PhantomData<S>,
    PhantomData<T>,
    PhantomData<O>,
);

impl<S: Serialize, T: Clone + Tabled + From<S> + TableInlineFormatSetter, O: TableOptions> From<Option<OutputFormatArgs>> for OutputFormatWrapper<S, T, O> {
    fn from(value: Option<OutputFormatArgs>) -> Self {
        Self(value, PhantomData, PhantomData, PhantomData)
    }
}

impl<S: Serialize + 'static, T: Clone + Tabled + From<S> + TableInlineFormatSetter + 'static, O: TableOptions + 'static> OutputFormatWrapper<S, T, O> {
    pub(crate) fn single(self, single_object: S) -> CommandResult {
        CommandResultBuilder::<S, T, O>::single(single_object, self.0).into_command_result()
    }
    pub(crate) fn collection(self, collection: Vec<S>) -> CommandResult {
        CommandResultBuilder::<S, T, O>::collection(collection, self.0).into_command_result()
    }
}
