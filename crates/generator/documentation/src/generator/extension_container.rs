use crate::generator::MarkdownDocumentation;
use crate::markdown::table::MarkdownTableExt;
use reactive_graph_graph::ExtensionContainer;
use reactive_graph_graph::NamespacedTypeContainer;
use tabled::Table;

impl<TY> MarkdownDocumentation<TY>
where
    TY: ExtensionContainer,
{
    pub fn extensions(mut self) -> Self {
        let extensions = self.ty.get_own_extensions_cloned();
        if extensions.is_empty() {
            return self;
        }
        self.document.header2("Extensions");
        let table = Table::new(&mut extensions.to_vec().into_iter()).to_owned();
        self.document.table(table);
        self
    }
}
