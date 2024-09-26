# Events: Shutdown

## Shutting down

| Property | Value  | Description                      |
|----------|--------|----------------------------------|
| shutdown | `true` | Boolean: Shutdown immediately    |
| shutdown | `5`    | Numeric: Shutdown in `5` seconds |

## GraphQL

```graphql
mutation {
  instances {
    entities {
      update(
        label: "/io/reactive-graph/commands/core/shutdown"
        properties: [
          {
            name: "shutdown"
            value: 5
          }
        ]
      ) {
        id
        type {
          name
        }
        properties(
          names: [
            "shutdown"
          ]
        ) {
          name
          value
        }
      }
    }
  }
}

```
