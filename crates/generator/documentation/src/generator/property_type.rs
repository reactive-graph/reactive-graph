use crate::generator::TypedMarkdownDocumentation;
use crate::markdown::table::MarkdownTableExt;
use crate::types::config::EnumGenerationMode;
use crate::types::config::ExtensionsDocumentationConfig;
use crate::types::config::PropertyDocumentationConfig;
use crate::types::config::SubTypesGenerationMode;
use documented::DocumentedVariants;
use reactive_graph_graph::ExtensionContainer;
use reactive_graph_graph::NamespacedTypeContainer;
use reactive_graph_graph::NamespacedTypeIdContainer;
use reactive_graph_graph::PropertyType;
use tabled::Table;

impl TypedMarkdownDocumentation<PropertyType> {
    pub fn property(mut self, config: &PropertyDocumentationConfig) -> Self {
        if config.header {
            let mut document = self.document.write().unwrap();
            document.header3(format!("Property `{}`", &self.ty.name));
        }
        if config.description {
            self.property_description();
        }
        self.property_data_type(&config.data_type);
        self.property_socket_type(&config.socket_type);
        self.property_mutability(&config.mutability);
        self.property_extensions(&config.extensions);
        self
    }

    pub fn property_description(&mut self) {
        if !self.ty.description.is_empty() {
            let mut document = self.document.write().unwrap();
            document.paragraph(self.ty.description.clone());
        }
    }

    pub fn property_data_type(&mut self, mode: &EnumGenerationMode) {
        let mut document = self.document.write().unwrap();
        match mode {
            EnumGenerationMode::None => {}
            EnumGenerationMode::Short => {
                document.paragraph(format!("Data Type: `{}`", self.ty.data_type));
            }
            EnumGenerationMode::Full => {
                document.header4("Data Type");
                document.paragraph(format!(
                    "<details><summary><code>{}</code></summary>{}</details>",
                    self.ty.data_type,
                    self.ty.data_type.get_variant_docs()
                ));
            }
        }
    }

    pub fn property_socket_type(&mut self, mode: &EnumGenerationMode) {
        let mut document = self.document.write().unwrap();
        match mode {
            EnumGenerationMode::None => {}
            EnumGenerationMode::Short => {
                document.paragraph(format!("Socket Type: `{}`", self.ty.socket_type));
            }
            EnumGenerationMode::Full => {
                document.header4("Socket Type");
                document.paragraph(format!(
                    "<details><summary><code>{}</code></summary>{}</details>",
                    self.ty.socket_type,
                    self.ty.socket_type.get_variant_docs()
                ));
            }
        }
    }

    pub fn property_mutability(&mut self, mode: &EnumGenerationMode) {
        let mut document = self.document.write().unwrap();
        match mode {
            EnumGenerationMode::None => {}
            EnumGenerationMode::Short => {
                document.paragraph(format!("Mutability: `{}`", self.ty.mutability));
            }
            EnumGenerationMode::Full => {
                document.header4("Mutability");
                document.paragraph(format!(
                    "<details><summary><code>{}</code></summary>{}</details>",
                    self.ty.mutability,
                    self.ty.mutability.get_variant_docs()
                ));
            }
        };
    }

    pub fn property_extensions(&mut self, config: &ExtensionsDocumentationConfig) {
        let extensions = self.ty.get_own_extensions_cloned();
        if extensions.is_empty() {
            return;
        }
        let mut document = self.document.write().unwrap();
        if config.header {
            document.header4("Property Extensions");
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
}
