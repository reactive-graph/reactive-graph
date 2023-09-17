use crate::ReactiveInstance;
use crate::ReactiveInstanceContainer;
use crate::TypedReactivePropertyImpl;
use crate::TypedReactivePropertyName;

impl<IdTypeSelf, ReactiveInstanceTypeSelf, TargetSelf, IdTypeOther, ReactiveInstanceTypeOther, TargetOther>
    PartialEq<TypedReactivePropertyImpl<IdTypeOther, ReactiveInstanceTypeOther, TargetOther>>
    for TypedReactivePropertyImpl<IdTypeSelf, ReactiveInstanceTypeSelf, TargetSelf>
where
    IdTypeSelf: Clone,
    ReactiveInstanceTypeSelf: ReactiveInstance<IdTypeSelf>,
    IdTypeOther: Clone,
    ReactiveInstanceTypeOther: ReactiveInstance<IdTypeOther> + ReactiveInstanceContainer<IdTypeOther, ReactiveInstanceTypeOther>,
{
    fn eq(&self, other: &TypedReactivePropertyImpl<IdTypeOther, ReactiveInstanceTypeOther, TargetOther>) -> bool {
        let Some(other) = ReactiveInstanceContainer::get(other.get_reactive_instance(), &other.property_name()) else {
            return false;
        };
        let Some(this) = self.reactive_instance.get(&self.property_name) else {
            return false;
        };
        this.eq(&other)
    }
}

impl<IdType, ReactiveInstanceType, Target> Eq for TypedReactivePropertyImpl<IdType, ReactiveInstanceType, Target>
where
    IdType: Clone,
    ReactiveInstanceType: ReactiveInstance<IdType> + ReactiveInstanceContainer<IdType, ReactiveInstanceType>,
{
}
