use itertools::Itertools;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::NamespacedTypeIdContainer;

pub fn field_name_collision<TY, TY2, TYS>(ty: &TY, tys: TYS) -> bool
where
    TY: NamespacedTypeGetter,
    TY2: NamespacedTypeGetter,
    TYS: NamespacedTypeIdContainer<TypeId = TY2, TypeIds = TYS>,
{
    tys.to_vec()
        .iter()
        .find_or_first(|entry| ty.namespace() == entry.namespace() && ty.type_name() == entry.type_name())
        .is_some()
}
