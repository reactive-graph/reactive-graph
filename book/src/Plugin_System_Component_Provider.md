# Component Provider

Plugins can implement the trait `ComponentProvider` in order to register components during initialization of the
plugin.

## Trait `ComponentProvider`

```rust
impl ComponentProvider for ExampleComponentProviderImpl {
    fn get_components(&self) -> Vec<Component> {
        // Return a vector of components
    }
}
```

## Use cases

* Read `Component`s from external JSON-file from a specific location
* Build JSON-file into plugin binary using RustEmbed
* Programmatically create `Component`s
