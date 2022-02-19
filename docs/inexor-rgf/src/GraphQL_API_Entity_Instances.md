# Entity Instances

## Query an entity instance by id

```graphql
query {
  instances {
    entities(id: "0-0-0-0") {
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
```

## Query a property instance by name of an entity instance by label

```graphql
query {
  instances {
    entities(label: "/org/inexor/input/any_device/key_f10") {
      properties(name: "key_down") {
        value
      }
    }
  }
}
```

## Query the positions of all cameras that are looking at a player

```graphql
query {
  instances {
    entities(type: "player", label: "/org/inexor/game/players/Hanack") {
      inbound(type: "look_at") {
        outbound {
          properties(name: "position") {
            value
          }
        }
      }
    }
  }
}
```
