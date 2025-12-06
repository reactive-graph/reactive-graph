use crate::error::DocumentationGenerationError;
use crate::generator::TypedMarkdownDocumentation;
use crate::markdown::code_block::MarkdownCodeBlockExt;
use crate::types::config::JsonSchemaDocumentationConfig;
use markdown_builder::LinkBuilder;
use reactive_graph_graph::ComponentTypeIdContainer;
use reactive_graph_graph::JsonSchemaIdGetter;
use reactive_graph_graph::PropertyTypeContainer;
use reactive_graph_graph::TypeDefinitionJsonSchemaGetter;
use reactive_graph_graph::TypeResolveError::ComponentResolveError;
use reactive_graph_graph::TypeResolver;

impl<TY> TypedMarkdownDocumentation<TY>
where
    TY: TypeDefinitionJsonSchemaGetter,
{
    pub fn json_schema(self, config: &JsonSchemaDocumentationConfig) -> Self {
        let json_schema = self.ty.json_schema().to_value();
        let Ok(json_schema) = serde_json::to_string_pretty(&json_schema) else {
            return self;
        };
        {
            let mut document = self.document.write().unwrap();
            if config.header {
                document.header2("JSON Schema");
            }
            if config.link {
                document.link(LinkBuilder::new().url(self.ty.json_schema_id().to_string()).build());
            }
            if config.source_code {
                document.code_block("json".to_string(), json_schema);
            }
        }
        self
    }
}

impl<TY> TypedMarkdownDocumentation<TY>
where
    TY: TypeDefinitionJsonSchemaGetter + ComponentTypeIdContainer + PropertyTypeContainer + Clone,
{
    pub fn json_schema_with_components(self, config: &JsonSchemaDocumentationConfig, resolver: &TypeResolver) -> Result<Self, DocumentationGenerationError> {
        let ty = self.ty.clone();
        for component_ty in ty.get_components_cloned() {
            let component = resolver.component(&component_ty).ok_or(ComponentResolveError(component_ty))?;
            ty.merge_non_existent_properties(component.properties);
        }
        let json_schema = ty.json_schema().to_value();
        let Ok(json_schema) = serde_json::to_string_pretty(&json_schema) else {
            return Ok(self);
        };
        {
            let mut document = self.document.write().unwrap();
            if config.header {
                document.header2("JSON Schema");
            }
            if config.link {
                document.link(LinkBuilder::new().url(ty.json_schema_id().to_string()).build());
            }
            if config.source_code {
                document.code_block("json".to_string(), json_schema);
            }
        }
        Ok(self)
    }
}
