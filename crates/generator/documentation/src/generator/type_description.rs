use crate::generator::MarkdownDocumentation;
use reactive_graph_graph::TypeDescriptionGetter;

impl<TY> MarkdownDocumentation<TY>
where
    TY: TypeDescriptionGetter,
{
    pub fn description(mut self) -> Self {
        let description = self.ty.description();
        if !description.is_empty() {
            self.document.paragraph(description);
        }
        self
    }
}
