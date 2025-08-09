use crate::generator::MarkdownDocumentation;
use crate::markdown::code_block::MarkdownCodeBlockExt;
use markdown_builder::LinkBuilder;
use reactive_graph_graph::JsonSchemaIdGetter;
use reactive_graph_graph::TypeDefinitionJsonSchemaGetter;

impl<TY> MarkdownDocumentation<TY>
where
    TY: TypeDefinitionJsonSchemaGetter,
{
    pub fn json_schema(mut self) -> Self {
        let json_schema = self.ty.json_schema().to_value();
        let Ok(json_schema) = serde_json::to_string_pretty(&json_schema) else {
            return self;
        };
        self.document.header2("JSON Schema");
        self.document.link(LinkBuilder::new().url(self.ty.json_schema_id().to_string()).build());
        self.document.code_block("json".to_string(), json_schema);
        self
    }
}
