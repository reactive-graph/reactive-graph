pub use collections::GenerateTypeSystemTypes;
pub use collections::GenerateTypeSystemTypesForTarget;
pub use collections::GenerateTypes;
pub use collections::GenerateTypesForTarget;
pub use result::CodeGenerationResult;
pub use type_generator::TypeGenerator;
pub use type_generator::TypeGeneratorForTarget;

pub mod collections;
pub mod component;
pub mod entity;
pub mod extension;
pub mod flow;
pub mod header;
pub mod namespace;
pub mod property;
pub mod relation;
pub mod result;
pub mod system;
pub mod type_definition;
pub mod type_generator;

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
