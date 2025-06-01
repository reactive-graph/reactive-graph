use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::TypeDefinition;

pub fn dynamic_schema_id<T: Into<TypeDefinition>>(ty: T) -> String {
    let ty = ty.into();
    format!(
        "https://schema.reactive-graph.io/schema/json/dynamic_graph/{}/{}/{}.schema.json",
        ty.type_id_type.full_name().to_lowercase(),
        ty.namespace(),
        ty.type_name()
    )
}
