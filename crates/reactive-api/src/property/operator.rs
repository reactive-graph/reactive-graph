use crate::ReactiveInstance;
use crate::TypedReactivePropertyImpl;
use std::ops::BitOrAssign;
use std::ops::ShlAssign;
use std::ops::ShrAssign;

impl<IdTypeSelf, ReactiveInstanceTypeSelf, TargetSelf, IdTypeOther, ReactiveInstanceTypeOther, TargetOther>
    ShlAssign<&TypedReactivePropertyImpl<IdTypeOther, ReactiveInstanceTypeOther, TargetOther>>
    for TypedReactivePropertyImpl<IdTypeSelf, ReactiveInstanceTypeSelf, TargetSelf>
where
    IdTypeSelf: Clone,
    ReactiveInstanceTypeSelf: ReactiveInstance<IdTypeSelf> + 'static,
    IdTypeOther: Clone,
    ReactiveInstanceTypeOther: ReactiveInstance<IdTypeOther>,
{
    fn shl_assign(&mut self, rhs: &TypedReactivePropertyImpl<IdTypeOther, ReactiveInstanceTypeOther, TargetOther>) {
        let self_reactive_instance = self.reactive_instance.clone();
        let self_property_name = self.property_name.clone();
        // Be sure that the reverse stream is removed
        self.reactive_instance.remove_observer(&self.property_name, rhs.handle_id);
        rhs.reactive_instance.observe_with_handle(
            &rhs.property_name,
            move |v| {
                self_reactive_instance.set(&self_property_name, v.clone());
            },
            self.handle_id,
        );
        if let Some(v) = rhs.reactive_instance.get(&rhs.property_name) {
            self.reactive_instance.set(&self.property_name, v);
        }
    }
}

impl<IdTypeSelf, ReactiveInstanceTypeSelf, TargetSelf, IdTypeOther, ReactiveInstanceTypeOther, TargetOther>
    ShrAssign<&TypedReactivePropertyImpl<IdTypeOther, ReactiveInstanceTypeOther, TargetOther>>
    for TypedReactivePropertyImpl<IdTypeSelf, ReactiveInstanceTypeSelf, TargetSelf>
where
    IdTypeSelf: Clone,
    ReactiveInstanceTypeSelf: ReactiveInstance<IdTypeSelf>,
    IdTypeOther: Clone,
    ReactiveInstanceTypeOther: ReactiveInstance<IdTypeOther> + 'static,
{
    fn shr_assign(&mut self, rhs: &TypedReactivePropertyImpl<IdTypeOther, ReactiveInstanceTypeOther, TargetOther>) {
        let rhs_reactive_instance = rhs.reactive_instance.clone();
        let rhs_property_name = rhs.property_name.clone();
        // Be sure that the reverse stream is removed
        rhs.reactive_instance.remove_observer(&rhs.property_name, self.handle_id);
        self.reactive_instance.observe_with_handle(
            &self.property_name,
            move |v| {
                rhs_reactive_instance.set(&rhs_property_name, v.clone());
            },
            rhs.handle_id,
        );
        if let Some(v) = self.reactive_instance.get(&self.property_name) {
            rhs.reactive_instance.set(&rhs.property_name, v);
        }
    }
}

impl<IdTypeSelf, ReactiveInstanceTypeSelf, TargetSelf, IdTypeOther, ReactiveInstanceTypeOther, TargetOther>
    BitOrAssign<&TypedReactivePropertyImpl<IdTypeOther, ReactiveInstanceTypeOther, TargetOther>>
    for TypedReactivePropertyImpl<IdTypeSelf, ReactiveInstanceTypeSelf, TargetSelf>
where
    IdTypeSelf: Clone,
    ReactiveInstanceTypeSelf: ReactiveInstance<IdTypeSelf>,
    IdTypeOther: Clone,
    ReactiveInstanceTypeOther: ReactiveInstance<IdTypeOther>,
{
    fn bitor_assign(&mut self, rhs: &TypedReactivePropertyImpl<IdTypeOther, ReactiveInstanceTypeOther, TargetOther>) {
        rhs.reactive_instance.remove_observer(&rhs.property_name, self.handle_id);
        self.reactive_instance.remove_observer(&rhs.property_name, rhs.handle_id);
        println!("remove streams in both directions {} <--> {}", self.handle_id, rhs.handle_id);
    }
}
