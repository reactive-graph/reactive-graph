use crate::ReactiveInstance;
use crate::ReactiveInstanceContainer;
use crate::TypedReactivePropertyImpl;

pub trait TypedReactivePropertyContainer<TY, TypeDefinition> {
    fn new_with_ty<IntoTy: Into<TY>>(ty: IntoTy) -> Self;
    fn new_from_type(type_definition: &TypeDefinition) -> Self;
}

impl<IdType, ReactiveInstanceType, Target> ReactiveInstanceContainer<IdType, ReactiveInstanceType>
    for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, Target>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType>,
{
    fn get_reactive_instance(&self) -> &ReactiveInstanceType {
        &self.reactive_instance
    }
}
