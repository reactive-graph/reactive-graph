use async_trait::async_trait;

use crate::api::ComponentManager;
use crate::api::EntityTypeManager;
use crate::api::FlowTypeManager;
use crate::api::RelationTypeManager;
use crate::api::TypeCategoryManager;
use crate::di::component;
use crate::di::provides;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::Extension;
use crate::model::ExtensionContainer;
use crate::model::TypeCategory;
use crate::model::TypeDefinition;
use crate::model::TypeDefinitionGetter;
use crate::model::TypeIdType;
use crate::model_runtime::EXTENSION_TYPE_CATEGORY;

pub struct ExtensionContainers<T: ExtensionContainer> {
    pub containers: Vec<T>,
}

impl<T> ExtensionContainers<T>
where
    T: ExtensionContainer + TypeDefinitionGetter,
{
    pub fn new(containers: Vec<T>) -> Self {
        ExtensionContainers { containers }
    }

    pub fn get_categories(&self) -> Vec<TypeCategory> {
        self.containers
            .iter()
            .filter_map(get_type_category_extension)
            .filter_map(get_type_category)
            .collect()
    }

    pub fn get_types_by_type_category(&self, category: &TypeCategory) -> Vec<TypeDefinition> {
        self.containers.iter().filter_map(|e| get_type_definition(e, category)).collect()
    }
}

pub fn get_type_category_extension<T: ExtensionContainer>(container: &T) -> Option<Extension> {
    container.get_own_extension(&EXTENSION_TYPE_CATEGORY)
}

pub fn get_type_category(extension: Extension) -> Option<TypeCategory> {
    extension.extension.as_str().map(TypeCategory::new)
}

fn has_type_category(extension: Extension, category: &TypeCategory) -> bool {
    get_type_category(extension).map(|category2| &category2 == category).unwrap_or(false)
}

fn get_type_definition<T: ExtensionContainer + TypeDefinitionGetter>(container: &T, category: &TypeCategory) -> Option<TypeDefinition> {
    container.get_own_extension(&EXTENSION_TYPE_CATEGORY).and_then(|extension| {
        if has_type_category(extension, category) {
            Some(container.type_definition())
        } else {
            None
        }
    })
}

#[component]
pub struct TypeCategoryManagerImpl {
    component_manager: Wrc<dyn ComponentManager>,
    entity_type_manager: Wrc<dyn EntityTypeManager>,
    relation_type_manager: Wrc<dyn RelationTypeManager>,
    flow_type_manager: Wrc<dyn FlowTypeManager>,
}

#[async_trait]
#[provides]
impl TypeCategoryManager for TypeCategoryManagerImpl {
    fn get_all_type_categories(&self) -> Vec<TypeCategory> {
        let mut categories: Vec<TypeCategory> = vec![
            ExtensionContainers::new(self.component_manager.get_all()).get_categories(),
            ExtensionContainers::new(self.entity_type_manager.get_all()).get_categories(),
            ExtensionContainers::new(self.relation_type_manager.get_all()).get_categories(),
            ExtensionContainers::new(self.flow_type_manager.get_all()).get_categories(),
        ]
        .into_iter()
        .flatten()
        .collect();
        categories.sort();
        categories.dedup();
        categories
    }

    fn get_types_by_type_category(&self, category: &TypeCategory) -> Vec<TypeDefinition> {
        vec![
            ExtensionContainers::new(self.component_manager.get_all()).get_types_by_type_category(category),
            ExtensionContainers::new(self.entity_type_manager.get_all()).get_types_by_type_category(category),
            ExtensionContainers::new(self.relation_type_manager.get_all()).get_types_by_type_category(category),
            ExtensionContainers::new(self.flow_type_manager.get_all()).get_types_by_type_category(category),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn get_types_by_type_category_and_type_id_type(&self, category: &TypeCategory, type_id_type: &TypeIdType) -> Vec<TypeDefinition> {
        self.get_types_by_type_category(category)
            .iter()
            .filter(|t| &t.type_id_type == type_id_type)
            .cloned()
            .collect()
    }
}
