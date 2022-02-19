# Entity Behaviour Provider

## Trait `EntityBehaviourProvider`

```rust
const EXAMPLE: &'static str = "example";

#[wrapper]
pub struct ExampleStorage(std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<Example>>>);

#[provides]
fn create_example_storage() -> ExampleStorage {
    ExampleStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait ExampleEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_example(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_example(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

pub struct ExampleEntityBehaviourProviderImpl {
    example_behaviours: ExampleStorage,
}

interfaces!(ExampleEntityBehaviourProviderImpl: dyn EntityBehaviourProvider);

#[component]
impl ExampleEntityBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            example_behaviours: create_example_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl ExampleEntityBehaviourProvider for ExampleEntityBehaviourProviderImpl {
    fn create_example(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let example_behaviour = Example::new(entity_instance);
        if example_behaviour.is_ok() {
            let example_behaviour = Arc::new(example_behaviour.unwrap());
            self.example_behaviours.0.write().unwrap().insert(id, example_behaviour);
            debug!("Added behaviour {} to entity instance {}", EXAMPLE, id);
        }
    }

    fn remove_example(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.example_behaviours.0.write().unwrap().remove(&entity_instance.id);
        debug!("Removed behaviour {} from entity instance {}", EXAMPLE, entity_instance.id);
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.example_behaviours.0.write().unwrap().contains_key(&id) {
            self.example_behaviours.0.write().unwrap().remove(&id);
            debug!("Removed behaviour {} from entity instance {}", EXAMPLE, id);
        }
    }
}

impl EntityBehaviourProvider for ExampleEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            EXAMPLE => self.create_example(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            EXAMPLE => self.remove_example(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
```
