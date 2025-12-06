use crate::generator::TypedMarkdownDocumentation;
use crate::markdown::table::MarkdownTableExt;
use crate::types::config::PropertiesDocumentationConfig;
use crate::types::config::SubTypesGenerationMode;
use markdown_builder::List;
use reactive_graph_graph::PropertyTypeContainer;
use reactive_graph_graph::PropertyTypes;
use tabled::Table;

impl<TY> TypedMarkdownDocumentation<TY>
where
    TY: PropertyTypeContainer,
{
    pub fn own_properties(self, config: &PropertiesDocumentationConfig) -> Self {
        let property_types = self.ty.get_own_properties_cloned();
        if !property_types.is_empty() {
            let mut document = self.document.write().unwrap();
            if config.header {
                match config.mode {
                    SubTypesGenerationMode::None => {}
                    SubTypesGenerationMode::Short => {
                        document.header3("Properties");
                    }
                    SubTypesGenerationMode::Table => {
                        document.header2("Properties");
                    }
                }
            }
        }
        self.properties(&property_types, config)
    }

    pub fn properties(self, property_types: &PropertyTypes, config: &PropertiesDocumentationConfig) -> Self {
        if property_types.is_empty() {
            return self;
        }
        let mut property_types = property_types.to_vec();
        property_types.sort();
        {
            let mut document = self.document.write().unwrap();
            match config.mode {
                SubTypesGenerationMode::None => {}
                SubTypesGenerationMode::Short => {
                    let mut list = List::unordered();
                    for property_type in property_types.iter() {
                        list.items.push(property_type.name.clone().into());
                    }
                    document.list(list);
                }
                SubTypesGenerationMode::Table => {
                    let table = Table::new(&mut property_types.clone().into_iter()).to_owned();
                    document.table(table);
                }
            }
        }
        match config.mode {
            SubTypesGenerationMode::None => {}
            SubTypesGenerationMode::Short => {}
            SubTypesGenerationMode::Table => {
                for property_type in property_types.iter() {
                    TypedMarkdownDocumentation::new_with_document(property_type.clone(), self.document.clone()).property(&config.property);
                }
            }
        }
        self
    }
}
