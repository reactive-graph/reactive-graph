use crate::generator::MarkdownDocumentation;
use crate::markdown::table::MarkdownTableExt;
use documented::DocumentedVariants;
use reactive_graph_graph::VariablesContainer;
use tabled::Table;

impl<TY> MarkdownDocumentation<TY>
where
    TY: VariablesContainer,
{
    pub fn variables(mut self) -> Self {
        let variables = self.ty.get_own_variables_cloned();
        if variables.is_empty() {
            return self;
        }
        self.document.header2("Variables");
        let table = Table::new(&mut variables.to_vec().into_iter()).to_owned();
        // println!("{}", table);
        self.document.table(table);

        for variable in variables.iter() {
            self.document.header3(format!("Variable `{}`", variable.name));
            if !variable.description.is_empty() {
                self.document.paragraph(variable.description.clone());
            }
            self.document.header4("Data Type");
            self.document.paragraph(format!(
                "<details><summary><code>{}</code></summary>{}</details>",
                variable.data_type,
                variable.data_type.get_variant_docs()
            ));
            self.document.header4("Socket Type");
            self.document.paragraph(format!(
                "<details><summary><code>{}</code></summary>{}</details>",
                variable.socket_type,
                variable.socket_type.get_variant_docs()
            ));
            self.document.header4("Mutability");
            self.document.paragraph(format!(
                "<details><summary><code>{}</code></summary>{}</details>",
                variable.mutability,
                variable.mutability.get_variant_docs()
            ));
            if !variable.extensions.is_empty() {
                self.document.header4("Extensions (TODO)");
            }
        }
        self
    }
}
