# Entity Type Provider

Plugins can implement the trait `EntityTypeProvider` in order to register entity types during initialization of the
plugin.

## Trait `EntityTypeProvider`

```rust
impl EntityTypeProvider for ExampleEntityTypeProviderProviderImpl {
    fn get_entity_types(&self) -> Vec<EntityType> {
        // Return a vector of entity types
    }
}
```

## Use cases

* Read `EntityType`s from external JSON-file from a specific location
* Build JSON-file into plugin binary using RustEmbed
* Programmatically create `EntityType`s
