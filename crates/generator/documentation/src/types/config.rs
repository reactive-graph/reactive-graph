use typed_builder::TypedBuilder;

#[derive(Clone, Debug)]
pub enum DocumentationConfigPreset {
    None,
    Short,
    Full,
}

#[derive(Clone, Debug)]
pub enum EnumGenerationMode {
    None,
    Short,
    Full,
}

impl Default for EnumGenerationMode {
    fn default() -> Self {
        Self::Short
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SubTypesGenerationMode {
    None,
    Short,
    Table,
}

impl Default for SubTypesGenerationMode {
    fn default() -> Self {
        Self::Short
    }
}

pub trait FromDocumentationConfigPreset {
    fn new_from_preset(preset: DocumentationConfigPreset) -> Self;
}

#[derive(Clone, Debug, TypedBuilder)]
pub struct DocumentationConfig {
    #[builder(default)]
    pub header: bool,
    #[builder(default)]
    pub description: bool,
    #[builder(default)]
    pub mode: SubTypesGenerationMode,
    #[builder(default)]
    pub components: Option<Box<DocumentationConfig>>,
    #[builder(default)]
    pub properties: PropertiesDocumentationConfig,
    #[builder(default)]
    pub extensions: ExtensionsDocumentationConfig,
    #[builder(default)]
    pub json_schema: JsonSchemaDocumentationConfig,
}

impl Default for DocumentationConfig {
    fn default() -> Self {
        DocumentationConfig {
            header: true,
            description: true,
            mode: Default::default(),
            components: Default::default(),
            properties: Default::default(),
            extensions: Default::default(),
            json_schema: Default::default(),
        }
    }
}

impl FromDocumentationConfigPreset for DocumentationConfig {
    fn new_from_preset(preset: DocumentationConfigPreset) -> Self {
        match preset {
            DocumentationConfigPreset::None => DocumentationConfig::builder()
                .header(false)
                .description(false)
                .mode(SubTypesGenerationMode::None)
                .components(None)
                .properties(PropertiesDocumentationConfig::new_from_preset(preset.clone()))
                .extensions(ExtensionsDocumentationConfig::new_from_preset(preset.clone()))
                .json_schema(JsonSchemaDocumentationConfig::new_from_preset(preset))
                .build(),
            DocumentationConfigPreset::Short => DocumentationConfig::builder()
                .header(true)
                .description(true)
                .mode(SubTypesGenerationMode::Short)
                .components(None)
                .properties(PropertiesDocumentationConfig::new_from_preset(preset.clone()))
                .extensions(ExtensionsDocumentationConfig::new_from_preset(preset.clone()))
                .json_schema(JsonSchemaDocumentationConfig::new_from_preset(preset))
                .build(),
            DocumentationConfigPreset::Full => DocumentationConfig::builder()
                .header(true)
                .description(true)
                .mode(SubTypesGenerationMode::Table)
                .components(Some(Box::new(
                    DocumentationConfig::builder()
                        .header(true)
                        .description(true)
                        .properties(PropertiesDocumentationConfig::new_from_preset(DocumentationConfigPreset::Short))
                        .extensions(ExtensionsDocumentationConfig::new_from_preset(DocumentationConfigPreset::Short))
                        .build(),
                )))
                .properties(PropertiesDocumentationConfig::new_from_preset(preset.clone()))
                .extensions(ExtensionsDocumentationConfig::new_from_preset(preset.clone()))
                .json_schema(JsonSchemaDocumentationConfig::new_from_preset(preset))
                .build(),
        }
    }
}

#[derive(Clone, Debug, TypedBuilder)]
pub struct ExtensionsDocumentationConfig {
    #[builder(default)]
    pub header: bool,
    #[builder(default)]
    pub description: bool,
    pub mode: SubTypesGenerationMode,
}

impl Default for ExtensionsDocumentationConfig {
    fn default() -> Self {
        ExtensionsDocumentationConfig {
            header: true,
            description: true,
            mode: Default::default(),
        }
    }
}

impl FromDocumentationConfigPreset for ExtensionsDocumentationConfig {
    fn new_from_preset(preset: DocumentationConfigPreset) -> Self {
        match preset {
            DocumentationConfigPreset::None => ExtensionsDocumentationConfig::builder()
                .header(false)
                .mode(SubTypesGenerationMode::None)
                .build(),
            DocumentationConfigPreset::Short => ExtensionsDocumentationConfig::builder()
                .header(true)
                .mode(SubTypesGenerationMode::Short)
                .build(),
            DocumentationConfigPreset::Full => ExtensionsDocumentationConfig::builder()
                .header(true)
                .mode(SubTypesGenerationMode::Table)
                .build(),
        }
    }
}

#[derive(Clone, Debug, TypedBuilder)]
pub struct JsonSchemaDocumentationConfig {
    #[builder(default)]
    pub header: bool,
    #[builder(default)]
    pub link: bool,
    #[builder(default)]
    pub source_code: bool,
}

impl Default for JsonSchemaDocumentationConfig {
    fn default() -> Self {
        JsonSchemaDocumentationConfig {
            header: true,
            link: true,
            source_code: false,
        }
    }
}

impl FromDocumentationConfigPreset for JsonSchemaDocumentationConfig {
    fn new_from_preset(preset: DocumentationConfigPreset) -> Self {
        match preset {
            DocumentationConfigPreset::None => JsonSchemaDocumentationConfig::builder().header(false).link(false).source_code(false).build(),
            DocumentationConfigPreset::Short => JsonSchemaDocumentationConfig::builder().header(true).link(true).source_code(false).build(),
            DocumentationConfigPreset::Full => JsonSchemaDocumentationConfig::builder().header(true).link(true).source_code(true).build(),
        }
    }
}

#[derive(Clone, Debug, TypedBuilder)]
pub struct PropertiesDocumentationConfig {
    #[builder(default)]
    pub header: bool,
    #[builder(default)]
    pub mode: SubTypesGenerationMode,
    #[builder(default)]
    pub property: PropertyDocumentationConfig,
}

impl Default for PropertiesDocumentationConfig {
    fn default() -> Self {
        PropertiesDocumentationConfig {
            header: true,
            mode: Default::default(),
            property: Default::default(),
        }
    }
}

impl FromDocumentationConfigPreset for PropertiesDocumentationConfig {
    fn new_from_preset(preset: DocumentationConfigPreset) -> Self {
        match preset {
            DocumentationConfigPreset::None => PropertiesDocumentationConfig::builder()
                .header(false)
                .mode(SubTypesGenerationMode::None)
                .property(PropertyDocumentationConfig::new_from_preset(preset.clone()))
                .build(),
            DocumentationConfigPreset::Short => PropertiesDocumentationConfig::builder()
                .header(true)
                .mode(SubTypesGenerationMode::Short)
                .property(PropertyDocumentationConfig::new_from_preset(preset.clone()))
                .build(),
            DocumentationConfigPreset::Full => PropertiesDocumentationConfig::builder()
                .header(true)
                .mode(SubTypesGenerationMode::Table)
                .property(PropertyDocumentationConfig::new_from_preset(preset.clone()))
                .build(),
        }
    }
}

#[derive(Clone, Debug, TypedBuilder)]
pub struct PropertyDocumentationConfig {
    #[builder(default)]
    pub header: bool,
    #[builder(default)]
    pub description: bool,
    #[builder(default)]
    pub data_type: EnumGenerationMode,
    #[builder(default)]
    pub socket_type: EnumGenerationMode,
    #[builder(default)]
    pub mutability: EnumGenerationMode,
    #[builder(default)]
    pub extensions: ExtensionsDocumentationConfig,
}

impl Default for PropertyDocumentationConfig {
    fn default() -> Self {
        PropertyDocumentationConfig {
            header: true,
            description: true,
            data_type: Default::default(),
            socket_type: Default::default(),
            mutability: Default::default(),
            extensions: Default::default(),
        }
    }
}

impl FromDocumentationConfigPreset for PropertyDocumentationConfig {
    fn new_from_preset(preset: DocumentationConfigPreset) -> Self {
        match preset {
            DocumentationConfigPreset::None => PropertyDocumentationConfig::builder()
                .header(false)
                .description(false)
                .data_type(EnumGenerationMode::None)
                .socket_type(EnumGenerationMode::None)
                .mutability(EnumGenerationMode::None)
                .extensions(ExtensionsDocumentationConfig::new_from_preset(preset.clone()))
                .build(),
            DocumentationConfigPreset::Short => PropertyDocumentationConfig::builder()
                .header(true)
                .description(true)
                .data_type(EnumGenerationMode::Short)
                .socket_type(EnumGenerationMode::Short)
                .mutability(EnumGenerationMode::Short)
                .extensions(ExtensionsDocumentationConfig::new_from_preset(preset.clone()))
                .build(),
            DocumentationConfigPreset::Full => PropertyDocumentationConfig::builder()
                .header(true)
                .description(true)
                .data_type(EnumGenerationMode::Full)
                .socket_type(EnumGenerationMode::Full)
                .mutability(EnumGenerationMode::Full)
                .extensions(ExtensionsDocumentationConfig::new_from_preset(preset.clone()))
                .build(),
        }
    }
}
