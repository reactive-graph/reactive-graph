# Extension

## Query extensions of an entity type

```graphql
query {
  types {
    entities(name: "example_entity") {
      name
      properties {
        extensions {
          name
          extension
        }
      }
    }
  }
}
```

## Query extension by name

```graphql
query {
  types {
    entities(name: "example_entity") {
      name
      properties {
        extensions(name: "flow_editor_palette") {
          extension
        }
      }
    }
  }
}
```
