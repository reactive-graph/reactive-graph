use crate::Namespace;
use crate::TypeSystem;

pub struct TypeSystemProvider(Namespace, TypeSystem);

impl TypeSystemProvider {
    pub fn new(id: Namespace, type_system: TypeSystem) -> TypeSystemProvider {
        TypeSystemProvider(id, type_system)
    }

    pub fn id(&self) -> Namespace {
        self.0.clone()
    }

    pub fn type_system(&self) -> TypeSystem {
        self.1.clone()
    }

    pub fn unpack(self) -> (Namespace, TypeSystem) {
        (self.0, self.1)
    }
}
