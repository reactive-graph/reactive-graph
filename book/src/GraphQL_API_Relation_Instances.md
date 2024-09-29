# Relation Instances

## Get pairs of outbound + inbound entity instances of all relations of a specific type

```graphql
query {
  instances {
    relations(type: "looks_at") {
      outbound {
        id
      }
      inbound {
        id
      }
    }
  }
}
```


## Get all inbound relation instances of an entity instance

```graphql
query {
  instances {
    entities(id: "uuid") {
      inbound {
        type {
          name
        }
        properties {
          name
          value
        }
      }
    }
  }
}
```

## Get the entity instances which are inbound relations to an entity instance

```graphql
query {
  instances {
    entities(type: "player") {
      inbound {
        outbound {
          id
          type {
            name
          }
          properties {
            name
            value
          }
        }
      }
    }
  }
}
```

## Get the entity instances which are outbound relations to an entity instance

```graphql
query {
  instances {
    entities(type: "player") {
      outbound {
        inbound {
          id
          type {
            name
          }
          properties {
            name
            value
          }
        }
      }
    }
  }
}
```
