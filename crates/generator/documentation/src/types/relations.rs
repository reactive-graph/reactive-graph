use crate::error::DocumentationGenerationError;
use crate::generator::MarkdownDocumentation;
use crate::markdown::table::MarkdownTableExt;
use crate::types::GenerateDocumentation;
use reactive_graph_graph::InboundOutboundType;
use reactive_graph_graph::RelationType;
use tabled::Table;

impl MarkdownDocumentation<RelationType> {
    pub fn outbound_type(mut self) -> Self {
        match &self.ty.outbound_type {
            InboundOutboundType::Component(ty) => {
                self.document.header2("Outbound Component");
                self.document.table(Table::new(vec![ty.to_string()]));
            }
            InboundOutboundType::EntityType(ty) => {
                self.document.header2("Outbound Entity");
                self.document.table(Table::new(vec![ty.to_string()]));
            }
        };
        self
    }

    pub fn inbound_type(mut self) -> Self {
        match &self.ty.inbound_type {
            InboundOutboundType::Component(ty) => {
                self.document.header2("Inbound Component");
                self.document.table(Table::new(vec![ty.to_string()]));
            }
            InboundOutboundType::EntityType(ty) => {
                self.document.header2("Inbound Entity");
                self.document.table(Table::new(vec![ty.to_string()]));
            }
        };
        self
    }
}

impl GenerateDocumentation<RelationType> for RelationType {
    fn generate_documentation(&self) -> Result<MarkdownDocumentation<RelationType>, DocumentationGenerationError> {
        Ok(MarkdownDocumentation::new(self.clone())
            .ty()
            .description()
            .outbound_type()
            .inbound_type()
            .components()
            .properties()
            .extensions()
            .json_schema())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::GenerateDocumentations;
    use default_test::DefaultTest;
    use reactive_graph_graph::RelationType;
    use reactive_graph_graph::RelationTypes;
    use std::fs::remove_file;

    #[test]
    pub fn test_generate_relation_type_documentation() {
        let relation_type = RelationType::default_test();
        let md = relation_type
            .generate_documentation()
            .expect("Failed to generate documentation for relation type");
        println!("{md}");
    }

    #[test]
    pub fn test_generate_relation_types_documentations() {
        RelationTypes::default_test()
            .generate_documentations()
            .expect("Failed to generate documentations for relation types");
    }

    #[test]
    pub fn test_write_relation_type_documentation() {
        let relation_type = RelationType::default_test();
        let path = relation_type
            .generate_documentation()
            .expect("Failed to generate documentation for relation type")
            .write()
            .expect("Failed to write relation type documentation");
        remove_file(path).expect("Failed to delete relation type documentation");
    }

    #[test]
    pub fn test_write_relation_types_documentations() {
        RelationTypes::default_test()
            .write_documentations()
            .expect("Failed to write documentations for relation types");
    }
}
