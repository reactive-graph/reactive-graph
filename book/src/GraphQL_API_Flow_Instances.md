# Flow Instance

## Get all flow ids

```graphql
query {
  instances {
    flows {
      id
    }
  }
}
```

## Get all flows of a particular entity type

```graphql
query {
  instances {
    flows(type: "generic_flow") {
      id
    }
  }
}
```

## Get the label of a flows by id

```graphql
query {
  instances {
    flows(id: "uuid") {
      label
    }
  }
}
```

## Get the id of a flows by label

```graphql
query {
  instances {
    flows(label: "/io/reactive-graph/flows/game-servers") {
      id
    }
  }
}
```

## Get the entity instance which are contained by a flow

```graphql
query {
  instances {
    flows(id: "uuid") {
      entities {
        id
      }
    }
  }
}
```

## Get the properties of the wrapper entity instance of a flow

```graphql
query {
  instances {
    flows(id: "uuid") {
      wrapper {
        properties {
          name
          value
          type {
            dataType
            socketType
          }
        }
      }
    }
  }
}
```

## Get the relation instances which are contained by a flow

```graphql
query {
  instances {
    flows(id: "uuid") {
      relations {
        outbound {
          id
        }
        type {
          name
        }
        inbound {
          id
        }
      }
    }
  }
}
```

## Get all entity instances and all relation instances of a flow

```graphql
query {
  instances {
    flows(id: "uuid") {
      type {
        name
      }
      label
      entities {
        id
        label
        type {
          name
        }
        properties {
          name
          value
        }
      }
      relations {
        type {
          name
        }
        outbound {
          id
        }
        inbound {
          id
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
