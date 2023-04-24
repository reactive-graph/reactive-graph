use async_trait::async_trait;

use crate::model::TypeCategory;
use crate::model::TypeDefinition;
use crate::model::TypeIdType;

#[async_trait]
pub trait TypeCategoryManager: Send + Sync {
    /// Returns the type categories.
    fn get_all_type_categories(&self) -> Vec<TypeCategory>;

    /// Returns all types for the given category.
    fn get_types_by_type_category(&self, category: &TypeCategory) -> Vec<TypeDefinition>;

    /// Returns all types for the given category.
    fn get_types_by_type_category_and_type_id_type(&self, category: &TypeCategory, type_id_type: &TypeIdType) -> Vec<TypeDefinition>;
}
