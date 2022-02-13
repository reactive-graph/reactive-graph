# 5. Plugin System

TODO: Introduction

## Trait `Plugin`

```rust
impl Plugin for ComparisonPluginImpl {

    // ===== Metadata =====

    fn metadata(&self) -> Result<PluginMetadata, PluginError> {}

    // ===== Lifecycle =====

    fn init(&self) -> Result<(), PluginError> {}

    fn post_init(&self) -> Result<(), PluginError> {}

    fn pre_shutdown(&self) -> Result<(), PluginError> {}

    fn shutdown(&self) -> Result<(), PluginError> {}

    // ===== Context =====

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginError> {}

    // ===== Providers =====
    
    fn get_component_provider(&self) -> Result<Arc<dyn ComponentProvider>, PluginError> {}

    fn get_entity_type_provider(&self) -> Result<Arc<dyn EntityTypeProvider>, PluginError> {}

    fn get_relation_type_provider(&self) -> Result<Arc<dyn RelationTypeProvider>, PluginError> {}

    fn get_component_behaviour_provider(&self) -> Result<Arc<dyn ComponentBehaviourProvider>, PluginError> {}

    fn get_entity_behaviour_provider(&self) -> Result<Arc<dyn EntityBehaviourProvider>, PluginError> {}

    fn get_relation_behaviour_provider(&self) -> Result<Arc<dyn RelationBehaviourProvider>, PluginError> {}

    fn get_flow_provider(&self) -> Result<Arc<dyn FlowProvider>, PluginError> {}

    fn get_web_resource_provider(&self) -> Result<Arc<dyn WebResourceProvider>, PluginError> {}
}
```

### Provide plugin metadata

Built-in metadata of the plugin:
* The name of the plugin (use environment variable `CARGO_PKG_NAME`)
* The description of the plugin (use environment variable `CARGO_PKG_DESCRIPTION`)
* The version of the plugin (use environment variable `CARGO_PKG_VERSION`)

```rust
impl Plugin for ComparisonPluginImpl {
    fn metadata(&self) -> Result<PluginMetadata, PluginError> {
        Ok(PluginMetadata {
            name: env!("CARGO_PKG_NAME").into(),
            description: env!("CARGO_PKG_DESCRIPTION").into(),
            version: env!("CARGO_PKG_VERSION").into(),
        })
    }
}
```

### Lifecycle

The plugin trait provides methods to do initialization and shutdown before and after setting up and shutting down
providers.

TODO: Lifecycle Image

```rust
impl Plugin for ComparisonPluginImpl {
    fn init(&self) -> Result<(), PluginError> {
        // Called before initialization of the providers
        Ok(())
    }

    fn post_init(&self) -> Result<(), PluginError> {
        // Called after initialization of the providers
        Ok(())
    }

    fn pre_shutdown(&self) -> Result<(), PluginError> {
        // Called before shutdown of the providers
        Ok(())
    }

    fn shutdown(&self) -> Result<(), PluginError> {
        // Called after shutdown of the providers
        Ok(())
    }
}
```

## Trait `PluginContext`

```rust
pub trait PluginContext: Send + Sync {
    /// Returns the component manager.
    fn get_component_manager(&self) -> Arc<dyn ComponentManager>;

    /// Returns the entity type manager.
    fn get_entity_type_manager(&self) -> Arc<dyn EntityTypeManager>;

    /// Returns the relation type manager.
    fn get_relation_type_manager(&self) -> Arc<dyn RelationTypeManager>;

    /// Returns the entity instance manager.
    fn get_entity_instance_manager(&self) -> Arc<dyn EntityInstanceManager>;

    /// Returns the relation instance manager.
    fn get_relation_instance_manager(&self) -> Arc<dyn RelationInstanceManager>;

    /// Returns the flow manager.
    fn get_flow_manager(&self) -> Arc<dyn FlowManager>;
}
```

### Make use of the plugin context

The plugin have to store a reference to the plugin context. Therefore, you have to implement the method `set_context`
which is called during initialization:

```rust
impl Plugin for ComparisonPluginImpl {
    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginError> {
        self.context.0.write().unwrap().replace(context);
        Ok(())
    }
}
```

If you want to make use of the `EntityInstanceManager` you have to:

1. Get a RwLock on the context
2. Get a reference of the `EntityInstanceManager` using `get_entity_instance_manager`
3. Create the entity instance using a builder (non-reactive)
4. Create/register the entity instance using `create` returns a reactive entity instance

```rust
impl TestManager {
    fn create_entity(&self) {
        let reader = self.context.0.read().unwrap();
        let entity_instance_manager = reader.as_ref().unwrap().get_entity_instance_manager().clone();
        let entity_instance = EntityInstanceBuilder::new("entity_type_name")
            .property("property_name", json!("property_value"))
            .get();
        let reactive_entity_instance = entity_instance_manager.create(entity_instance);
    }
}
```

### Implement or ignore providers

If your plugin provide components you have to return a reference of the `ComponentProvider`:

```rust
impl Plugin for ComparisonPluginImpl {
    fn get_component_provider(&self) -> Result<Arc<dyn ComponentProvider>, PluginError> {
        let component_provider = self.component_provider.clone();
        let component_provider: Result<Arc<dyn ComponentProvider>, _> = <dyn query_interface::Object>::query_arc(component_provider);
        if component_provider.is_err() {
            return Err(PluginError::NoComponentProvider);
        }
        Ok(component_provider.unwrap())
    }
}
```

If your plugin doesn't provide components:

```rust
impl Plugin for ComparisonPluginImpl {
    fn get_component_provider(&self) -> Result<Arc<dyn ComponentProvider>, PluginError> {
        Err(PluginError::NoComponentProvider)
    }
}
```

The same applies to all other providers:
* `ComponentProvider`
* `EntityTypeProvider`
* `RelationTypeProvider`
* `EntityBehaviourProvider`
* `RelationBehaviourProvider`
* `FlowProvider`
* `WebResourceProvider`
