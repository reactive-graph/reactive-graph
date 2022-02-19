# Entity Types

## Get Entity Type by Name

```graphql
query {
  types {
    entities(name: "example") {
      name
      description
      components {
        name
      }
      properties {
        name
        dataType
        socketType
        extensions {
          name
          extension
        }
      }
    }
  }
}
```

## Find Entity Types

```graphql
query {
  types {
    entities(search: "*xamp*") {
      name
      description
      components {
        name
      }
      properties {
        name
        dataType
        socketType
      }
    }
  }
}
```

## Create Entity Type

```graphql
mutation {
  types {
    entities {
      create(
        name: "example",
        components: [
          "example_component"
        ]
        properties: [
          {
            name: "example"
            description: "Example Input Property"
            dataType: STRING
            socketType: INPUT
            extensions: []
          }
        ]
      ) {
        name
        description
        components {
          name
        }
        properties {
          name
          dataType
          socketType
        }
      }
    }
  }
}
```

## Delete Entity Type

```graphql
mutation {
  types {
    entities {
      delete(name: "example")
    }
  }
}
```
