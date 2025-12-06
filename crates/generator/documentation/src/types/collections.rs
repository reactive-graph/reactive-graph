use crate::error::DocumentationGenerationError;
use crate::generator::MarkdownDocumentation;
use crate::generator::TypedMarkdownDocumentation;
use crate::generator::type_definition::MarkdownDocumentationWriter;
use crate::types::GenerateDocumentation;
use crate::types::config::DocumentationConfig;
use reactive_graph_graph::NamespacedTypeContainer;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_graph::TypeResolver;
use reactive_graph_graph::TypeSystem;
use std::hash::Hash;
use std::path::PathBuf;

pub trait GenerateDocumentations
where
    Self::Container: NamespacedTypeContainer<Type = Self::Type>,
    Self::Type: GenerateDocumentation<Self::Type>,
{
    type Container;
    type Type;

    fn generate_typed_documentations(
        &self,
        config: &DocumentationConfig,
        resolver: &TypeResolver,
    ) -> Result<Vec<TypedMarkdownDocumentation<Self::Type>>, DocumentationGenerationError>;

    fn generate_documentations(
        &self,
        config: &DocumentationConfig,
        resolver: &TypeResolver,
    ) -> Result<Vec<MarkdownDocumentation>, DocumentationGenerationError> {
        Ok(GenerateDocumentations::generate_typed_documentations(self, config, resolver)?
            .iter()
            .map(MarkdownDocumentation::from)
            .collect())
    }

    fn write_documentations(
        &self,
        config: &DocumentationConfig,
        resolver: &TypeResolver,
        base_path: &PathBuf,
    ) -> Result<Vec<PathBuf>, DocumentationGenerationError>;
}

impl<TYS, TY> GenerateDocumentations for TYS
where
    TYS: NamespacedTypeContainer<Type = TY>,
    TY: GenerateDocumentation<TY> + Eq + Hash + TypeDefinitionGetter + NamespacedTypeGetter + Clone,
{
    type Container = TYS;
    type Type = TY;

    fn generate_typed_documentations(
        &self,
        config: &DocumentationConfig,
        resolver: &TypeResolver,
    ) -> Result<Vec<TypedMarkdownDocumentation<Self::Type>>, DocumentationGenerationError> {
        let mut v = vec![];
        for entry in self.types().iter() {
            let ty = entry.key();
            v.push(ty.generate_documentation(config, resolver)?);
        }
        Ok(v)
    }

    fn write_documentations(
        &self,
        config: &DocumentationConfig,
        resolver: &TypeResolver,
        base_path: &PathBuf,
    ) -> Result<Vec<PathBuf>, DocumentationGenerationError> {
        let mut v = vec![];
        for documentation in self.generate_typed_documentations(config, resolver)? {
            v.push(documentation.write(base_path)?);
        }
        Ok(v)
    }
}

pub trait RenderDocumentations {
    fn render(&self, config: &DocumentationConfig, resolver: &TypeResolver) -> Result<Vec<String>, DocumentationGenerationError>;
    fn render_all(&self, config: &DocumentationConfig, resolver: &TypeResolver) -> Result<String, DocumentationGenerationError> {
        Ok(self.render(config, resolver)?.join("\n"))
    }
}

impl<TYS, TY> RenderDocumentations for TYS
where
    TYS: NamespacedTypeContainer<Type = TY> + GenerateDocumentations<Container = TYS>,
    TY: GenerateDocumentation<TY> + Eq + Hash + TypeDefinitionGetter + NamespacedTypeGetter + Clone,
{
    fn render(&self, config: &DocumentationConfig, resolver: &TypeResolver) -> Result<Vec<String>, DocumentationGenerationError> {
        Ok(self
            .generate_documentations(config, resolver)?
            .iter()
            .map(MarkdownDocumentation::to_string)
            .collect())
    }
}

pub trait GenerateTypeSystemDocumentations {
    fn generate_documentations(
        &self,
        config: &DocumentationConfig,
        resolver: &TypeResolver,
    ) -> Result<Vec<MarkdownDocumentation>, DocumentationGenerationError>;
    fn write_documentations(
        &self,
        base_path: &PathBuf,
        config: &DocumentationConfig,
        resolver: &TypeResolver,
    ) -> Result<Vec<PathBuf>, DocumentationGenerationError>;
}

impl GenerateTypeSystemDocumentations for TypeSystem {
    fn generate_documentations(
        &self,
        config: &DocumentationConfig,
        resolver: &TypeResolver,
    ) -> Result<Vec<MarkdownDocumentation>, DocumentationGenerationError> {
        let mut results = vec![];
        results.append(&mut GenerateDocumentations::generate_documentations(self.components(), config, resolver)?);
        results.append(&mut GenerateDocumentations::generate_documentations(self.entity_types(), config, resolver)?);
        results.append(&mut GenerateDocumentations::generate_documentations(self.relation_types(), config, resolver)?);
        results.append(&mut GenerateDocumentations::generate_documentations(self.flow_types(), config, resolver)?);
        Ok(results)
    }

    fn write_documentations(
        &self,
        base_path: &PathBuf,
        config: &DocumentationConfig,
        resolver: &TypeResolver,
    ) -> Result<Vec<PathBuf>, DocumentationGenerationError> {
        let mut results = vec![];
        results.append(&mut GenerateDocumentations::write_documentations(self.components(), config, resolver, base_path)?);
        results.append(&mut GenerateDocumentations::write_documentations(self.entity_types(), config, resolver, base_path)?);
        results.append(&mut GenerateDocumentations::write_documentations(self.relation_types(), config, resolver, base_path)?);
        results.append(&mut GenerateDocumentations::write_documentations(self.flow_types(), config, resolver, base_path)?);
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use crate::DocumentationConfigPreset;
    use crate::FromDocumentationConfigPreset;
    use crate::GenerateDocumentations;
    use crate::types::config::DocumentationConfig;
    use reactive_graph_graph::Components;
    use reactive_graph_graph::EntityTypes;
    use reactive_graph_graph::NamespacedTypeComponentTypeIdContainer;
    use reactive_graph_graph::RandomNamespacedTypes;
    use reactive_graph_graph::RandomNamespacedTypesWithId;
    use reactive_graph_graph::TypeSystem;
    use reactive_graph_utils_test::r_temp_dir;

    #[test]
    pub fn test_generate_documentations() {
        let config = DocumentationConfig::new_from_preset(DocumentationConfigPreset::Full);
        let entity_types = EntityTypes::random_types(0..10).unwrap();
        let resolver = TypeSystem::builder()
            .components(Components::random_types_with_ids(&entity_types.get_component_type_ids()).unwrap())
            .entity_types(entity_types.clone())
            .build()
            .into();
        let results = GenerateDocumentations::generate_documentations(&entity_types, &config, &resolver).unwrap();
        assert_eq!(entity_types.len(), results.len());
    }

    #[test]
    pub fn test_write_documentations() {
        let config = DocumentationConfig::new_from_preset(DocumentationConfigPreset::Full);
        let entity_types = EntityTypes::random_types(0..10).unwrap();
        let resolver = TypeSystem::builder()
            .components(Components::random_types_with_ids(&entity_types.get_component_type_ids()).unwrap())
            .entity_types(entity_types.clone())
            .build()
            .into();
        let temp_dir = r_temp_dir();
        println!("temp dir path: {temp_dir:?}");

        let results = GenerateDocumentations::write_documentations(&entity_types, &config, &resolver, &temp_dir)
            .expect(&format!("Failed to write documentations to {temp_dir:?}"));
        assert_eq!(entity_types.len(), results.len());
    }
}
