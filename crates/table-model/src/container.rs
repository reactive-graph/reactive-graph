use std::fmt::Display;
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::vec::IntoIter;
use table_to_html::HtmlTable;
use tabled::settings::Style;
use tabled::Table;
use tabled::Tabled;

pub trait TableContainer: Display {
    fn table(&self) -> Table;

    fn markdown_table(&self) -> Table;

    fn html_table(&self) -> HtmlTable;
}

pub trait TableOptions: Sized {
    fn options(table: &mut Table) -> &mut Table;
}

#[derive(Debug)]
pub enum TableOutputFormat {
    Table,
    MarkdownTable,
    HtmlTable,
}

#[derive(Clone, Debug)]
pub enum TableInlineFormat {
    Table,
    Html,
}

pub trait TableInlineFormatSetter {
    fn set_table_inline_format(&mut self, table_inline_format: TableInlineFormat);
}

impl Default for TableInlineFormat {
    fn default() -> Self {
        Self::Table
    }
}

pub struct DefaultTableOptions {}

impl TableOptions for DefaultTableOptions {
    fn options(table: &mut Table) -> &mut Table {
        table.with(Style::extended())
    }
}

pub struct DefaultTableContainer<S, T: Clone + Tabled + From<S> + TableInlineFormatSetter, O: TableOptions>(
    pub(crate) Vec<T>,
    TableOutputFormat,
    PhantomData<S>,
    PhantomData<O>,
);

impl<S: 'static, T: Clone + Tabled + From<S> + TableInlineFormatSetter + 'static, O: TableOptions + 'static> DefaultTableContainer<S, T, O> {
    pub fn into_boxed(self) -> Box<DefaultTableContainer<S, T, O>> {
        Box::new(self)
    }

    pub fn into_html_table(mut self) -> Self {
        self.0.iter_mut().for_each(|item| item.set_table_inline_format(TableInlineFormat::Html));
        self.1 = TableOutputFormat::HtmlTable;
        self
    }

    pub fn into_markdown_table(mut self) -> Self {
        self.0.iter_mut().for_each(|item| item.set_table_inline_format(TableInlineFormat::Html));
        self.1 = TableOutputFormat::MarkdownTable;
        self
    }
}

impl<S, T: Clone + Tabled + From<S> + TableInlineFormatSetter, O: TableOptions> TableContainer for DefaultTableContainer<S, T, O> {
    fn table(&self) -> Table {
        let mut table = Table::new(self.0.to_vec().iter());
        O::options(&mut table).to_owned()
    }

    fn markdown_table(&self) -> Table {
        Table::new(self.0.to_vec().iter()).with(Style::markdown()).to_owned()
    }

    fn html_table(&self) -> HtmlTable {
        let items = self.0.to_vec();
        HtmlTable::with_header(Vec::<Vec<String>>::from(Table::builder(&items)))
    }
}

impl<S, T: Clone + Tabled + From<S> + TableInlineFormatSetter, O: TableOptions> From<Vec<S>> for DefaultTableContainer<S, T, O> {
    fn from(entries: Vec<S>) -> Self {
        DefaultTableContainer::<S, T, O>(entries.into_iter().map(From::from).collect(), TableOutputFormat::Table, PhantomData, PhantomData)
    }
}

impl<S, T: Clone + Tabled + From<S> + TableInlineFormatSetter, O: TableOptions> From<S> for DefaultTableContainer<S, T, O> {
    fn from(s: S) -> Self {
        DefaultTableContainer::<S, T, O>(vec![s.into()], TableOutputFormat::Table, PhantomData, PhantomData)
    }
}

impl<S, T: Clone + Tabled + From<S> + TableInlineFormatSetter, O: TableOptions> From<IntoIter<S>> for DefaultTableContainer<S, T, O> {
    fn from(iter: IntoIter<S>) -> Self {
        DefaultTableContainer::<S, T, O>(iter.map(From::from).collect(), TableOutputFormat::Table, PhantomData, PhantomData)
    }
}

impl<S, T: Clone + Tabled + From<S> + TableInlineFormatSetter, O: TableOptions> FromIterator<S> for DefaultTableContainer<S, T, O> {
    fn from_iter<TT: IntoIterator<Item = S>>(iter: TT) -> Self {
        let entries = iter.into_iter().map(From::from).collect();
        DefaultTableContainer::<S, T, O>(entries, TableOutputFormat::Table, PhantomData, PhantomData)
    }
}

impl<S, T: Clone + Tabled + From<S> + TableInlineFormatSetter, O: TableOptions> Display for DefaultTableContainer<S, T, O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.1 {
            TableOutputFormat::Table => {
                write!(f, "{}", self.table())
            }
            TableOutputFormat::MarkdownTable => {
                write!(f, "{}", self.markdown_table())
            }
            TableOutputFormat::HtmlTable => {
                write!(f, "{}", self.html_table().to_string().trim())
            }
        }
    }
}
