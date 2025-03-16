use crate::shared::output_format::OutputFormatArgs;
use crate::shared::output_format::OutputFormatArgsOptional;
use reactive_graph_serde::error::SerializationError;
use serde::Serialize;
use std::process::exit;
use table_to_html::HtmlTable;
use tabled::Table;
use tabled::Tabled;
use tabled::settings::Style;
use tabled::settings::Width;
use tabled::settings::object::Rows;
use thiserror::Error;
use toml::map::Map;

#[derive(Debug, Error)]
pub enum RenderTableError {
    #[error("Failed to serialize: {0}")]
    SerializationError(#[from] SerializationError),
    #[error("Not yet implemented")]
    NotImplemented,
}

pub trait RenderTable {
    fn render(&self, output_format: &OutputFormatArgs) -> Result<String, RenderTableError>;

    fn do_print_table_and_exit(&self, output_format: &OutputFormatArgs) -> !;
    fn print_table_and_exit(&self, output_format: &OutputFormatArgsOptional) -> !;
}

impl<T: Tabled + Serialize> RenderTable for Vec<T> {
    fn render(&self, output_format: &OutputFormatArgs) -> Result<String, RenderTableError> {
        match output_format {
            OutputFormatArgs::Table => {
                let table = Table::new(self).modify(Rows::new(1..), Width::wrap(40)).to_owned();
                Ok(format!("{}", table))
            }
            OutputFormatArgs::HtmlTable => Ok(HtmlTable::with_header(Vec::<Vec<String>>::from(Table::builder(self)))
                .to_string()
                .trim()
                .to_string()),
            OutputFormatArgs::MarkdownTable => {
                let table = Table::new(self).with(Style::markdown()).to_owned();
                Ok(format!("{}", table))
            }
            OutputFormatArgs::Count => Ok(format!("{}", self.len())),
            OutputFormatArgs::Json | OutputFormatArgs::Json5 => Ok(serde_json::to_string_pretty(self).map_err(SerializationError::Json)?),
            OutputFormatArgs::Toml => {
                let inner = toml::Value::try_from(self).map_err(SerializationError::Toml)?;
                let mut map: Map<String, toml::Value> = Map::new();
                let type_name = std::any::type_name::<T>().rsplit_once("::").unwrap_or_default().1.to_string();
                map.insert(type_name, inner);
                let table = toml::Value::Table(map);
                Ok(toml::to_string_pretty(&table).map_err(SerializationError::Toml)?)
            }
        }
    }

    fn do_print_table_and_exit(&self, output_format: &OutputFormatArgs) -> ! {
        match self.render(output_format) {
            Ok(rendered_table) => {
                println!("{rendered_table}");
                exit(0);
            }
            Err(e) => {
                eprintln!("{e}");
                exit(1);
            }
        }
    }
    fn print_table_and_exit(&self, output_format: &OutputFormatArgsOptional) -> ! {
        self.do_print_table_and_exit(&output_format.output_format.clone().unwrap_or_default())
    }
}
