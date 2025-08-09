use crate::generator::MarkdownDocumentation;
use crate::markdown::table::MarkdownTableExt;
use documented::DocumentedVariants;
use reactive_graph_graph::ExtensionContainer;
use reactive_graph_graph::NamespacedTypeContainer;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypeContainer;
use tabled::Table;

impl<TY> MarkdownDocumentation<TY>
where
    TY: PropertyTypeContainer,
{
    pub fn properties(mut self) -> Self {
        let property_types = self.ty.get_own_properties_cloned();
        if property_types.is_empty() {
            return self;
        }
        self.document.header2("Properties");
        let table = Table::new(&mut property_types.to_vec().into_iter()).to_owned();
        self.document.table(table);

        for property_type in property_types.iter() {
            self.property(property_type.value())
        }
        self
    }

    pub fn property(&mut self, property_type: &PropertyType) {
        self.document.header3(format!("Property `{}`", property_type.name));
        self.property_description(property_type);
        self.property_data_type(property_type);
        self.property_socket_type(property_type);
        self.property_mutability(property_type);
        self.property_extensions(property_type);
    }

    pub fn property_description(&mut self, property_type: &PropertyType) {
        if !property_type.description.is_empty() {
            self.document.paragraph(property_type.description.clone());
        }
    }

    pub fn property_data_type(&mut self, property_type: &PropertyType) {
        self.document.header4("Data Type");
        self.document.paragraph(format!(
            "<details><summary><code>{}</code></summary>{}</details>",
            property_type.data_type,
            property_type.data_type.get_variant_docs()
        ));
    }

    pub fn property_socket_type(&mut self, property_type: &PropertyType) {
        self.document.header4("Socket Type");
        self.document.paragraph(format!(
            "<details><summary><code>{}</code></summary>{}</details>",
            property_type.socket_type,
            property_type.socket_type.get_variant_docs()
        ));
    }

    pub fn property_mutability(&mut self, property_type: &PropertyType) {
        self.document.header4("Mutability");
        self.document.paragraph(format!(
            "<details><summary><code>{}</code></summary>{}</details>",
            property_type.mutability,
            property_type.mutability.get_variant_docs()
        ));
    }

    pub fn property_extensions(&mut self, property_type: &PropertyType) {
        let extensions = property_type.get_own_extensions_cloned();
        if extensions.is_empty() {
            return;
        }
        self.document.header4("Property Extensions");
        let table = Table::new(&mut extensions.to_vec().into_iter()).to_owned();
        self.document.table(table);
    }
}
