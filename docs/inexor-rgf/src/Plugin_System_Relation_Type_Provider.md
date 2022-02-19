# Relation Type Provider

Plugins can implement the trait `RelationTypeProvider` in order to register relation types during initialization of the
plugin.

## Trait `RelationTypeProvider`

```rust
impl RelationTypeProvider for ExampleRelationTypeProviderProviderImpl {
    fn get_relation_types(&self) -> Vec<RelationType> {
        // Return a vector of relation types
    }
}
```

## Use cases

* Read `RelationType`s from external JSON-file from a specific location
* Build JSON-file into plugin binary using RustEmbed
* Programmatically create `RelationType`s
