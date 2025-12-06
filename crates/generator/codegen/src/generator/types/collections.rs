use crate::CodeGenerationConfig;
use crate::CodeGenerationError;
use crate::generator::types::CodeGenerationResult;
use crate::generator::types::TypeGenerator;
use crate::targets::CodeGenerationTarget;
use crate::targets::CodeGenerationTargets;
use crate::targets::java::Java;
use crate::targets::rust::Rust;
use reactive_graph_graph::Component;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::NamespacedTypeContainer;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::RelationType;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_graph::TypeResolver;
use reactive_graph_graph::types::TypeSystem;
use std::hash::Hash;

pub trait GenerateTypesForTarget: GenerateTypes<Java> + GenerateTypes<Rust> {
    fn generate_type_for_target(
        &self,
        target: CodeGenerationTargets,
        config: &CodeGenerationConfig,
        resolver: &TypeResolver,
    ) -> Result<Vec<CodeGenerationResult>, CodeGenerationError> {
        match target {
            CodeGenerationTargets::Java => GenerateTypes::<Java>::generate_types(self, config, resolver),
            CodeGenerationTargets::Rust => GenerateTypes::<Rust>::generate_types(self, config, resolver),
            _ => Err(CodeGenerationError::TargetNotSupported(target)),
        }
    }
}

impl<TYS, TY> GenerateTypesForTarget for TYS
where
    TYS: NamespacedTypeContainer<Type = TY> + GenerateTypes<Java> + GenerateTypes<Rust>,
    TY: TypeGenerator<Java> + TypeGenerator<Rust> + Eq + Hash + TypeDefinitionGetter + NamespacedTypeGetter + Clone,
{
}

pub trait GenerateTypes<Target: CodeGenerationTarget>
where
    Self::Container: NamespacedTypeContainer<Type = Self::Type>,
    Self::Type: TypeGenerator<Target>,
{
    type Container;
    type Type;

    fn generate_types(&self, config: &CodeGenerationConfig, resolver: &TypeResolver) -> Result<Vec<CodeGenerationResult>, CodeGenerationError>;
}

impl<TYS, TY, Target> GenerateTypes<Target> for TYS
where
    TYS: NamespacedTypeContainer<Type = TY>,
    TY: TypeGenerator<Target> + Eq + Hash + TypeDefinitionGetter + NamespacedTypeGetter + Clone,
    Target: CodeGenerationTarget,
{
    type Container = TYS;
    type Type = TY;

    fn generate_types(&self, config: &CodeGenerationConfig, resolver: &TypeResolver) -> Result<Vec<CodeGenerationResult>, CodeGenerationError> {
        let mut v = vec![];
        for entry in self.types().iter() {
            let ty = entry.key();
            // println!("Generating types for {}", ty.namespace().to_string());
            v.push(ty.generate_type(config, resolver)?);
        }
        Ok(v)
    }
}

pub trait GenerateTypeSystemTypesForTarget
where
    Self: GenerateTypeSystemTypes<Java>,
    Self: GenerateTypeSystemTypes<Rust>,
{
    fn generate_types_for_target(
        &self,
        target: CodeGenerationTargets,
        config: &CodeGenerationConfig,
        resolver: &TypeResolver,
    ) -> Result<Vec<CodeGenerationResult>, CodeGenerationError> {
        match target {
            CodeGenerationTargets::Java => GenerateTypeSystemTypes::<Java>::generate_types(self, config, resolver),
            CodeGenerationTargets::Rust => GenerateTypeSystemTypes::<Rust>::generate_types(self, config, resolver),
            _ => Err(CodeGenerationError::TargetNotSupported(target)),
        }
    }
}

impl GenerateTypeSystemTypesForTarget for TypeSystem {}

pub trait GenerateTypeSystemTypes<Target: CodeGenerationTarget> {
    fn generate_types(&self, config: &CodeGenerationConfig, resolver: &TypeResolver) -> Result<Vec<CodeGenerationResult>, CodeGenerationError>;
}

impl<Target: CodeGenerationTarget> GenerateTypeSystemTypes<Target> for TypeSystem
where
    Component: TypeGenerator<Target>,
    EntityType: TypeGenerator<Target>,
    RelationType: TypeGenerator<Target>,
    FlowType: TypeGenerator<Target>,
{
    fn generate_types(&self, config: &CodeGenerationConfig, resolver: &TypeResolver) -> Result<Vec<CodeGenerationResult>, CodeGenerationError> {
        let mut results = Vec::new();
        results.append(&mut GenerateTypes::<Target>::generate_types(self.components(), config, resolver)?);
        results.append(&mut GenerateTypes::<Target>::generate_types(self.entity_types(), config, resolver)?);
        results.append(&mut GenerateTypes::<Target>::generate_types(self.relation_types(), config, resolver)?);
        results.append(&mut GenerateTypes::<Target>::generate_types(self.flow_types(), config, resolver)?);
        results.push(self.generate_type(config, resolver)?);
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use crate::CodeGenerationConfig;
    use crate::GenerateTypeSystemTypesForTarget;
    use crate::generator::types::collections::GenerateTypeSystemTypes;
    use crate::generator::types::collections::GenerateTypes;
    use crate::generator::types::collections::GenerateTypesForTarget;
    use crate::targets::CodeGenerationTargets;
    use crate::targets::rust::Rust;
    use reactive_graph_graph::EntityTypes;
    use reactive_graph_graph::RandomNamespacedTypes;
    use reactive_graph_graph::TypeResolver;
    use reactive_graph_graph::prelude::TypeSystem;

    const CARGO_CRATE_NAME: &str = env!("CARGO_CRATE_NAME");

    #[test]
    pub fn test_generate_types() {
        let config = CodeGenerationConfig::with_temp_dir();
        let entity_types = EntityTypes::random_types(0..10).unwrap();
        let resolver = TypeResolver::new();
        let code_gen_results = GenerateTypes::<Rust>::generate_types(&entity_types, &config, &resolver).expect("Failed to generate types");
        assert_eq!(code_gen_results.len(), entity_types.len());
    }

    #[test]
    pub fn test_generate_types_for_target() {
        let config = CodeGenerationConfig::with_temp_dir();
        let entity_types = EntityTypes::random_types(0..10).unwrap();
        let resolver = TypeResolver::new();
        let code_gen_results = entity_types
            .generate_type_for_target(CodeGenerationTargets::Rust, &config, &resolver)
            .expect("Failed to generate types");
        println!("{} {}", code_gen_results.len(), entity_types.len());
        assert_eq!(code_gen_results.len(), entity_types.len());
    }

    #[test]
    pub fn test_generate_type_system_types() {
        let config = CodeGenerationConfig::with_temp_dir();
        let type_system = TypeSystem::random_tree(2..3, 1..5, 1..3, 1..2).expect("Failed to create a random type system tree");
        let resolver = TypeResolver::new();
        resolver.insert(CARGO_CRATE_NAME.to_string(), type_system.clone());
        let code_gen_results = GenerateTypeSystemTypes::<Rust>::generate_types(&type_system, &config, &resolver).expect("Failed to generate types");
        assert_eq!(code_gen_results.len(), type_system.len());
    }

    #[test]
    pub fn test_generate_type_system_types_for_target() {
        let config = CodeGenerationConfig::with_temp_dir();
        let type_system = TypeSystem::random_tree(2..3, 1..5, 1..3, 1..2).expect("Failed to create a random type system tree");
        // type_system.merge_own_component_properties().expect("Failed to merge component properties");
        let resolver = TypeResolver::new();
        resolver.insert(CARGO_CRATE_NAME.to_string(), type_system.clone());
        let code_gen_results = type_system
            .generate_types_for_target(CodeGenerationTargets::Rust, &config, &resolver)
            .expect("Failed to generate types");
        assert_eq!(code_gen_results.len(), type_system.len());
    }
}
