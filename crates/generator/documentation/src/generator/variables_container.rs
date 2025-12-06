use crate::generator::TypedMarkdownDocumentation;
use crate::markdown::table::MarkdownTableExt;
use crate::types::config::PropertiesDocumentationConfig;
use crate::types::config::SubTypesGenerationMode;
use documented::DocumentedVariants;
use markdown_builder::List;
use reactive_graph_graph::VariablesContainer;
use tabled::Table;

impl<TY> TypedMarkdownDocumentation<TY>
where
    TY: VariablesContainer,
{
    pub fn variables(self, config: &PropertiesDocumentationConfig) -> Self {
        let variables = self.ty.get_own_variables_cloned();
        if variables.is_empty() {
            return self;
        }
        {
            let mut document = self.document.write().unwrap();
            if config.header {
                document.header2("Variables");
            }
            match config.mode {
                SubTypesGenerationMode::None => {}
                SubTypesGenerationMode::Short => {
                    let mut list = List::unordered();
                    for property_type in variables.iter() {
                        list.items.push(property_type.name.clone().into());
                    }
                    document.list(list);
                }
                SubTypesGenerationMode::Table => {
                    let table = Table::new(&mut variables.to_vec().into_iter()).to_owned();
                    document.table(table);
                }
            }

            for variable in variables.iter() {
                document.header3(format!("Variable `{}`", variable.name));
                if !variable.description.is_empty() {
                    document.paragraph(variable.description.clone());
                }
                document.header4("Data Type");
                document.paragraph(format!(
                    "<details><summary><code>{}</code></summary>{}</details>",
                    variable.data_type,
                    variable.data_type.get_variant_docs()
                ));
                document.header4("Socket Type");
                document.paragraph(format!(
                    "<details><summary><code>{}</code></summary>{}</details>",
                    variable.socket_type,
                    variable.socket_type.get_variant_docs()
                ));
                document.header4("Mutability");
                document.paragraph(format!(
                    "<details><summary><code>{}</code></summary>{}</details>",
                    variable.mutability,
                    variable.mutability.get_variant_docs()
                ));
                if !variable.extensions.is_empty() {
                    document.header4("Extensions (TODO)");
                }
            }
        }

        self
    }
}
