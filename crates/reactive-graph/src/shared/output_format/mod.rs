use clap::Parser;
pub use render_table::*;
use serde::Serialize;

pub mod render_table;

#[derive(clap::ValueEnum, Default, Debug, Clone, Serialize)]
pub enum OutputFormatArgs {
    // The output is formatted as a table.
    #[default]
    Table,
    // The output is formatted as a HTML table.
    HtmlTable,
    // The output is formatted as a Markdown table.
    MarkdownTable,
    // Shows the count.
    Count,
    // The output is returned as JSON.
    Json,
    // The output is returned as JSON5.
    Json5,
    // The output is returned as TOML.
    Toml,
}

#[derive(Parser, Debug)]
pub struct OutputFormatArgsOptional {
    /// The output format.
    #[arg(long)]
    pub output_format: Option<OutputFormatArgs>,
}
