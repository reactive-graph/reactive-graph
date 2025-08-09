use crate::error::CodeGenerationError;
use crate::generator::GenerateCode;
use crate::target::rust::Rust;
use reactive_graph_graph::Component;

impl GenerateCode<Component, Rust> for Component {
    fn generate_code(&self) -> Result<(), CodeGenerationError> {
        todo!()
    }
}

#[cfg(test)]
pub mod tests {
    use default_test::DefaultTest;
    use reactive_graph_graph::Component;

    #[test]
    pub fn test_generate_component_documentation() {
        let component = Component::default_test();
        component.generate()
    }
}
