use markdown_builder::Markdown;
use markdown_builder::MarkdownElement;
use tabled::Table;
use tabled::settings::Style;

#[derive(Clone, Debug)]
pub struct MarkdownTable(pub Table);

impl MarkdownElement for MarkdownTable {
    fn render(&self) -> String {
        format!("{}\n", self.0.to_string())
    }
}

pub trait MarkdownTableExt {
    fn table(&mut self, table: Table) -> &mut Self;
}

impl MarkdownTableExt for Markdown {
    fn table(&mut self, mut table: Table) -> &mut Self {
        let table = table.with(Style::markdown());
        self.elements.push(Box::new(MarkdownTable(table.to_owned())));
        self
    }
}
