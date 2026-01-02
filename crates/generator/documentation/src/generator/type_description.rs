use crate::generator::TypedMarkdownDocumentation;
use crate::types::config::DocumentationConfig;
use reactive_graph_graph::TypeDescriptionGetter;

impl<TY> TypedMarkdownDocumentation<TY>
where
    TY: TypeDescriptionGetter,
{
    pub fn description(self, config: &DocumentationConfig) -> Self {
        if config.description {
            let description = self.ty.description();
            if !description.is_empty() {
                let mut document = self.document.write().unwrap();
                if config.header {
                    document.header2("Description");
                }
                document.paragraph(description);
            }
        }
        self
    }
}
