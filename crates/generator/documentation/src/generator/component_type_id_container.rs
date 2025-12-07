use crate::error::DocumentationGenerationError;
use crate::generator::TypedMarkdownDocumentation;
use crate::markdown::table::MarkdownTableExt;
use crate::types::config::DocumentationConfig;
use crate::types::config::PropertiesDocumentationConfig;
use crate::types::config::SubTypesGenerationMode;
use reactive_graph_graph::Component;
use reactive_graph_graph::ComponentTypeIdContainer;
use reactive_graph_graph::NamespacedTypeContainer;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::PropertyTypeContainer;
use reactive_graph_graph::PropertyTypes;
use reactive_graph_graph::TypeResolveError::ComponentResolveError;
use reactive_graph_graph::TypeResolver;
use tabled::Table;
use tabled::Tabled;

impl<TY> TypedMarkdownDocumentation<TY>
where
    TY: ComponentTypeIdContainer,
{
    pub fn components(self, config: &DocumentationConfig, resolver: &TypeResolver) -> Result<Self, DocumentationGenerationError> {
        let tys = self.ty.get_components_cloned();
        if tys.is_empty() {
            return Ok(self);
        }
        let components = resolver.components(&tys)?;
        {
            let mut document = self.document.write().unwrap();
            if config.header {
                document.header2("Components");
            }
            match config.mode {
                SubTypesGenerationMode::None => {}
                SubTypesGenerationMode::Short | SubTypesGenerationMode::Table => {
                    let mut components = components.to_vec();
                    components.sort();
                    let view: Vec<ComponentView> = components.into_iter().map(|component| ComponentView::new(component)).collect();
                    let table = Table::new(&mut view.into_iter()).to_owned();
                    document.table(table);
                }
            }
        }
        Ok(self)
    }
}

impl<TY> TypedMarkdownDocumentation<TY>
where
    TY: ComponentTypeIdContainer + PropertyTypeContainer,
{
    pub fn component_properties(
        self,
        config: &PropertiesDocumentationConfig,
        resolver: &TypeResolver,
        merge: bool,
    ) -> Result<Self, DocumentationGenerationError> {
        let property_types = if merge { self.ty.get_own_properties_cloned() } else { PropertyTypes::new() };
        for component_ty in self.ty.get_components_cloned() {
            let Some(component) = resolver.component(&component_ty) else {
                return Err(ComponentResolveError(component_ty).into());
            };
            property_types.merge_non_existent_properties(component.properties);
        }
        if !property_types.is_empty() {
            let mut document = self.document.write().unwrap();
            if config.header {
                match config.mode {
                    SubTypesGenerationMode::None => {}
                    SubTypesGenerationMode::Short => {
                        document.header3("Properties from components");
                    }
                    SubTypesGenerationMode::Table => {
                        document.header2("Properties from components");
                    }
                }
            }
        }
        Ok(self.properties(&property_types, config))
    }
}

#[derive(Tabled)]
struct ComponentView {
    #[tabled(rename = "Component")]
    namespace: String,
    #[tabled(rename = "Description")]
    description: String,
    #[tabled(rename = "Properties")]
    properties: String,
}

impl ComponentView {
    pub fn new(component: Component) -> Self {
        let mut nproperty_names = component.properties.names().to_vec();
        nproperty_names.sort();
        ComponentView {
            namespace: format!("`{}`", component.ty.namespace().to_string()),
            description: component.description.clone(),
            properties: format!("<ul compact><li>`{}`</li></ul>", nproperty_names.join("`</li><li>`")),
        }
    }
}
