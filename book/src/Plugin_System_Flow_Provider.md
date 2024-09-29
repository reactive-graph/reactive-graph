# Flow Provider

Plugins can implement the trait `FlowProvider` in order to create a flow during initialization of the
plugin.

## Trait `FlowProvider`

```rust
impl FlowProvider for ExampleFlowProviderProviderImpl {
    fn get_flows(&self) -> Vec<Flow> {
        // Return a vector of flows
    }
}
```

## Use cases

* Read `Flow`s from external JSON-file from a specific location
* Build JSON-file into plugin binary using RustEmbed
* Programmatically create `Flow`s
