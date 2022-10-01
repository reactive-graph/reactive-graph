# Relation Types

## Get Relation Type by Name

```graphql
query {
  types {
    relations(name: "example_relation") {
      name
      instanceTypeName
      description
      outboundTypes {
        name
      }
      inboundTypes {
        name
      }
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

## Find Relation Type

```graphql
query {
  types {
    relations(search: "example_relation?") {
      name
      instanceTypeName
      description
      outboundTypes {
        name
      }
      inboundTypes {
        name
      }
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

## Create Relation Type

```graphql
mutation {
  types {
    relations {
      create(
        name: "example_relation"
        outboundType: "entitytype1"
        inboundType: "entitytype2"
        components: [
          "component1"
        ]
        properties: [
          {
            name: "example_property"
            description: "Example Property"
            dataType: NUMBER
            socketType: INPUT
            extensions: []
          }
        ]
      ) {
        name
        description
        outboundTypes {
          name
        }
        inboundTypes {
          name
        }
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

## Delete Relation Type

```graphql
mutation {
  types {
    relations {
      delete(name: "example_relation")
    }
  }
}
```
