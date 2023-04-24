use std::sync::Arc;

use async_graphql::*;

use crate::api::ComponentManager;
use crate::api::EntityTypeManager;
use crate::api::FlowTypeManager;
use crate::api::RelationTypeManager;
use crate::api::TypeCategoryManager;
use crate::graphql::query::GraphQLComponent;
use crate::graphql::query::GraphQLEntityType;
use crate::graphql::query::GraphQLFlowType;
use crate::graphql::query::GraphQLRelationType;
use crate::model::NamespacedType;
use crate::model::TypeCategory;
use crate::model::TypeIdType;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct GraphQLTypeCategory {
    type_category: TypeCategory,
}

#[Object(name = "TypeCategory")]
impl GraphQLTypeCategory {
    async fn name(&self) -> String {
        self.type_category.to_string()
    }

    async fn components(&self, context: &Context<'_>) -> Result<Vec<GraphQLComponent>> {
        let type_category_manager = context.data::<Arc<dyn TypeCategoryManager>>()?;
        let component_manager = context.data::<Arc<dyn ComponentManager>>()?;
        let components = type_category_manager
            .get_types_by_type_category_and_type_id_type(&self.type_category, &TypeIdType::Component)
            .iter()
            .filter_map(|ty| {
                let ty: NamespacedType = ty.into();
                component_manager.get(&ty.into()).map(|component| component.into())
            })
            .collect();
        Ok(components)
    }

    async fn entity_types(&self, context: &Context<'_>) -> Result<Vec<GraphQLEntityType>> {
        let type_category_manager = context.data::<Arc<dyn TypeCategoryManager>>()?;
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        let entity_types = type_category_manager
            .get_types_by_type_category_and_type_id_type(&self.type_category, &TypeIdType::EntityType)
            .iter()
            .filter_map(|ty| {
                let ty: NamespacedType = ty.into();
                entity_type_manager.get(&ty.into()).map(|entity_type| entity_type.into())
            })
            .collect();
        Ok(entity_types)
    }

    async fn relation_types(&self, context: &Context<'_>) -> Result<Vec<GraphQLRelationType>> {
        let type_category_manager = context.data::<Arc<dyn TypeCategoryManager>>()?;
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let relation_types = type_category_manager
            .get_types_by_type_category_and_type_id_type(&self.type_category, &TypeIdType::RelationType)
            .iter()
            .filter_map(|ty| {
                let ty: NamespacedType = ty.into();
                relation_type_manager.get(&ty.into()).map(|relation_type| relation_type.into())
            })
            .collect();
        Ok(relation_types)
    }

    async fn flow_types(&self, context: &Context<'_>) -> Result<Vec<GraphQLFlowType>> {
        let type_category_manager = context.data::<Arc<dyn TypeCategoryManager>>()?;
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager>>()?;
        let flow_types = type_category_manager
            .get_types_by_type_category_and_type_id_type(&self.type_category, &TypeIdType::FlowType)
            .iter()
            .filter_map(|ty| {
                let ty: NamespacedType = ty.into();
                flow_type_manager.get(&ty.into()).map(|flow_type| flow_type.into())
            })
            .collect();
        Ok(flow_types)
    }
}

impl From<TypeCategory> for GraphQLTypeCategory {
    fn from(type_category: TypeCategory) -> Self {
        GraphQLTypeCategory { type_category }
    }
}
