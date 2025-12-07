use crate::ComponentTypeId;
use crate::ComponentTypeIds;
use crate::PropertyType;
use std::collections::HashMap;
use std::ops::Deref;

pub struct DivergentPropertyType {
    existing: PropertyType,
    divergent: PropertyType,
}

impl DivergentPropertyType {
    pub fn new(existing: &PropertyType, divergent: &PropertyType) -> Self {
        Self {
            existing: existing.clone(),
            divergent: divergent.clone(),
        }
    }

    pub fn existing(&self) -> &PropertyType {
        &self.existing
    }

    pub fn divergent(&self) -> &PropertyType {
        &self.divergent
    }
}

pub struct DivergentPropertyTypes(Vec<DivergentPropertyType>);

impl DivergentPropertyTypes {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, existing: &PropertyType, divergent: &PropertyType) {
        self.0.push(DivergentPropertyType::new(existing, divergent));
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Deref for DivergentPropertyTypes {
    type Target = Vec<DivergentPropertyType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Divergent {
    divergent_components: HashMap<ComponentTypeId, DivergentPropertyTypes>,
    unfulfilled_components: ComponentTypeIds,
}

impl Divergent {
    pub fn new() -> Self {
        Divergent {
            divergent_components: HashMap::new(),
            unfulfilled_components: ComponentTypeIds::new(),
        }
    }

    pub fn is_divergent(&self) -> bool {
        !self.divergent_components.is_empty() || !self.unfulfilled_components.is_empty()
    }

    pub fn divergent_component<TY: Into<ComponentTypeId>>(&mut self, ty: TY, divergent_property_types: DivergentPropertyTypes) {
        self.divergent_components.insert(ty.into(), divergent_property_types);
    }
    pub fn unfulfilled_component<TY: Into<ComponentTypeId>>(&mut self, ty: TY) {
        self.unfulfilled_components.insert(ty.into());
    }

    pub fn divergent_components(&self) -> ComponentTypeIds {
        let divergent_components = self.unfulfilled_components.clone();
        for ty in self.divergent_components.keys().cloned() {
            divergent_components.insert(ty);
        }
        divergent_components
    }
}
