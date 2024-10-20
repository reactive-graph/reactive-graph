use serde::Serialize;

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
