use crate::generator::TypedMarkdownDocumentation;
use crate::markdown::table::MarkdownTableExt;
use crate::types::config::ExtensionsDocumentationConfig;
use crate::types::config::SubTypesGenerationMode;
use reactive_graph_graph::ExtensionContainer;
use reactive_graph_graph::NamespacedTypeContainer;
use reactive_graph_graph::NamespacedTypeIdContainer;
use tabled::Table;

impl<TY> TypedMarkdownDocumentation<TY>
where
    TY: ExtensionContainer,
{
    pub fn extensions(self, config: &ExtensionsDocumentationConfig) -> Self {
        let extensions = self.ty.get_own_extensions_cloned();
        if extensions.is_empty() {
            return self;
        }
        {
            let mut document = self.document.write().unwrap();
            if config.header {
                document.header2("Extensions");
            }
            match config.mode {
                SubTypesGenerationMode::None => {}
                SubTypesGenerationMode::Short => {
                    let table = Table::new(&mut extensions.type_ids().to_vec().into_iter()).to_owned();
                    document.table(table);
                }
                SubTypesGenerationMode::Table => {
                    let table = Table::new(&mut extensions.to_vec().into_iter()).to_owned();
                    document.table(table);
                }
            }
        }
        self
    }
}
