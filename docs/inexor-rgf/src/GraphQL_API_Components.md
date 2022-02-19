# Components

## Get Component by Name

```graphql
query {
  types {
    components(name: "component1") {
      name
      description
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

## Find Components

Search by name with wildcards (`?` = single character, `*` = multiple characters)

```graphql
query {
  types {
    components(search: "*player*") {
      name
      description
      properties {
        name
        dataType
        socketType
      }
    }
  }
}
```

## Create Component

```graphql
mutation {
  types {
    components {
      create(
        name: "component1",
        properties: [
          {
            name: "first"
            description: "First Property"
            dataType: ANY
            socketType: NONE
            extensions: [
              {
                name: "sorted"
                extension: {
                  sortProperty: "first"
                }
              }
            ]
          }
        ]
      ) {
        name
        description
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
}
```

## Delete Component

```graphql
mutation {
  types {
    components {
      delete(name: "component1")
    }
  }
}
```

## Navigate from Component to Entity Types

```graphql
query {
  types {
    components(name: "component1") {
      entityTypes {
        name
      }
    }
  }
}
```

## Navigate from Component to Relation Types

```graphql
query {
  types {
    components(name: "component1") {
      relationTypes {
        name
        fullName
      }
    }
  }
}
```
