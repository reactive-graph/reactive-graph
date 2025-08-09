use crate::generator::MarkdownDocumentation;
use crate::markdown::table::MarkdownTableExt;
use reactive_graph_graph::ComponentTypeIdContainer;
use reactive_graph_graph::NamespacedTypeIdContainer;
use tabled::Table;

impl<TY> MarkdownDocumentation<TY>
where
    TY: ComponentTypeIdContainer,
{
    pub fn components(mut self) -> Self {
        let tys = self.ty.get_components_cloned();
        if tys.is_empty() {
            return self;
        }
        self.document.header2("Components");
        let table = Table::new(&mut tys.to_vec().into_iter()).to_owned();
        self.document.table(table);
        self
    }
}
