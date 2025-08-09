use crate::error::DocumentationGenerationError;
use crate::generator::MarkdownDocumentation;
use crate::types::GenerateDocumentation;
use reactive_graph_graph::FlowType;

impl GenerateDocumentation<FlowType> for FlowType {
    fn generate_documentation(&self) -> Result<MarkdownDocumentation<FlowType>, DocumentationGenerationError> {
        Ok(MarkdownDocumentation::new(self.clone()).ty().description().variables().extensions())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::GenerateDocumentations;
    use default_test::DefaultTest;
    use reactive_graph_graph::FlowType;
    use reactive_graph_graph::FlowTypes;
    use std::fs::remove_file;

    #[test]
    pub fn test_generate_flow_type_documentation() {
        let flow_type = FlowType::default_test();
        let md = flow_type.generate_documentation().expect("Failed to generate documentation for flow type");
        println!("{md}");
    }

    #[test]
    pub fn test_generate_flow_types_documentations() {
        FlowTypes::default_test()
            .generate_documentations()
            .expect("Failed to generate documentations for flow types");
    }

    #[test]
    pub fn test_write_flow_type_documentation() {
        let flow_type = FlowType::default_test();
        let path = flow_type
            .generate_documentation()
            .expect("Failed to generate documentation for flow type")
            .write()
            .expect("Failed to write flow type documentation");
        remove_file(path).expect("Failed to delete flow type documentation");
    }

    #[test]
    pub fn test_write_flow_types_documentations() {
        FlowTypes::default_test()
            .write_documentations()
            .expect("Failed to write documentations for flow types");
    }
}
