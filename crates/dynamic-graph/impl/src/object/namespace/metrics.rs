use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::ResolverContext;
use async_graphql::dynamic::TypeRef;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::FlowTypeManager;
use reactive_graph_type_system_api::RelationTypeManager;
use std::sync::Arc;

const TYPE_METRICS: &str = "NamespaceMetrics";
const ROOT_NAMESPACE: &str = "root";

#[derive(Debug)]
enum ResolvedNamespace {
    Root,
    Namespace(String),
}

impl From<Option<String>> for ResolvedNamespace {
    fn from(value: Option<String>) -> Self {
        match value {
            None => ResolvedNamespace::Root,
            Some(namespace) => {
                if namespace.as_str() == ROOT_NAMESPACE {
                    ResolvedNamespace::Root
                } else {
                    ResolvedNamespace::Namespace(namespace)
                }
            }
        }
    }
}

pub fn metrics_type_name() -> TypeRef {
    TypeRef::named_nn(TYPE_METRICS)
}

pub fn metrics_object() -> Object {
    Object::new(TYPE_METRICS)
        .field(Field::new("components", TypeRef::named_nn(TypeRef::INT), move |ctx| {
            let namespace = extract_namespace_from_parent_value(&ctx);
            FieldFuture::new(async move {
                let component_manager = ctx.data::<Arc<dyn ComponentManager + Send + Sync>>()?;
                let count = match namespace {
                    ResolvedNamespace::Root => component_manager.count(),
                    ResolvedNamespace::Namespace(namespace) => component_manager.count_by_namespace(&namespace),
                };
                Ok(Some(FieldValue::value(count)))
            })
        }))
        .field(Field::new("entityTypes", TypeRef::named_nn(TypeRef::INT), move |ctx| {
            let namespace = extract_namespace_from_parent_value(&ctx);
            FieldFuture::new(async move {
                let entity_type_manager = ctx.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
                let count = match namespace {
                    ResolvedNamespace::Root => entity_type_manager.count(),
                    ResolvedNamespace::Namespace(namespace) => entity_type_manager.count_by_namespace(&namespace),
                };
                Ok(Some(FieldValue::value(count)))
            })
        }))
        .field(Field::new("relationTypes", TypeRef::named_nn(TypeRef::INT), move |ctx| {
            let namespace = extract_namespace_from_parent_value(&ctx);
            FieldFuture::new(async move {
                let relation_type_manager = ctx.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
                let count = match namespace {
                    ResolvedNamespace::Root => relation_type_manager.count(),
                    ResolvedNamespace::Namespace(namespace) => relation_type_manager.count_by_namespace(&namespace),
                };
                Ok(Some(FieldValue::value(count)))
            })
        }))
        .field(Field::new("flowTypes", TypeRef::named_nn(TypeRef::INT), move |ctx| {
            let namespace = extract_namespace_from_parent_value(&ctx);
            FieldFuture::new(async move {
                let flow_type_manager = ctx.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
                let count = match namespace {
                    ResolvedNamespace::Root => flow_type_manager.count(),
                    ResolvedNamespace::Namespace(namespace) => flow_type_manager.count_by_namespace(&namespace),
                };
                Ok(Some(FieldValue::value(count)))
            })
        }))
}

pub fn metrics_field(namespace: Option<String>) -> Field {
    let namespace = namespace.clone().unwrap_or("root".to_string());
    let description = format!("Metrics for {namespace}");
    Field::new("_metrics", metrics_type_name(), move |_ctx| {
        let namespace = namespace.clone();
        FieldFuture::new(async move { Ok(Some(FieldValue::value(namespace))) })
    })
    .description(description)
}

fn extract_namespace_from_parent_value(ctx: &ResolverContext) -> ResolvedNamespace {
    ctx.parent_value
        .as_value()
        .and_then(|v| v.clone().into_json().ok().and_then(|value| value.as_str().map(|s| s.to_owned())))
        .into()
}
