# Plugin Arithmetic

This plugin provides arithmetic gates and operations.

## Components

| Name                 | Properties | DataType | SocketType | Description |
|----------------------|------------|----------|------------|-------------|
|                      |
| arithmetic_operation | lhs        | number   | input      |             |
|                      | result     | number   | output     |             |
|                      |
| arithmetic_gate      | lhs        | number   | input      |             |
|                      | rhs        | number   | input      |             |
|                      | result     | number   | output     |             |

## Entity Types

| Name      | Components           | Properties | DataType | SocketType | Description                                       |
|-----------|----------------------|------------|----------|------------|---------------------------------------------------|
||
| add       | arithmetic_gate      | lhs        | number   | input      | Addition                                          |
|           | arithmetic_gate      | rhs        | number   | input      |                                                   |
|           | arithmetic_gate      | result     | number   | output     |                                                   |
||
| counter   | action               | trigger    | bool     | input      | If triggered, the result will be incremented by 1 |
|           | action               | result     | number   | output     |                                                   |
||
| decrement | arithmetic_operation | lhs        | number   | input      | Decrements the input by 1                         |
|           | arithmetic_operation | result     | number   | output     |                                                   |
||
| div       | arithmetic_gate      | lhs        | number   | input      | Division                                          |
|           | arithmetic_gate      | rhs        | number   | input      |                                                   |
|           | arithmetic_gate      | result     | number   | output     |                                                   |
||
| increment | arithmetic_operation | lhs        | number   | input      | Increments the input by 1                         |
|           | arithmetic_operation | result     | number   | output     |                                                   |
||
| max       | arithmetic_gate      | lhs        | number   | input      | Max value                                         |
|           | arithmetic_gate      | rhs        | number   | input      |                                                   |
|           | arithmetic_gate      | result     | number   | output     |                                                   |
||
| min       | arithmetic_gate      | lhs        | number   | input      | Min value                                         |
|           | arithmetic_gate      | rhs        | number   | input      |                                                   |
|           | arithmetic_gate      | result     | number   | output     |                                                   |
||
| mod       | arithmetic_gate      | lhs        | number   | input      | Modulo                                            |
|           | arithmetic_gate      | rhs        | number   | input      |                                                   |
|           | arithmetic_gate      | result     | number   | output     |                                                   |
||
| mul       | arithmetic_gate      | lhs        | number   | input      | Multiplication                                    |
|           | arithmetic_gate      | rhs        | number   | input      |                                                   |
|           | arithmetic_gate      | result     | number   | output     |                                                   |
||
| sub       | arithmetic_gate      | lhs        | number   | input      | Subtraction                                       |
|           | arithmetic_gate      | rhs        | number   | input      |                                                   |
|           | arithmetic_gate      | result     | number   | output     |                                                   |

## Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |

## Repository

| Name                         | Repository                                                                                                               |
|------------------------------|--------------------------------------------------------------------------------------------------------------------------|
| inexor-rgf-plugin-arithmetic | [https://github.com/inexorgame/inexor-rgf-plugin-arithmetic](https://github.com/inexorgame/inexor-rgf-plugin-arithmetic) |

## Usage

### GraphQL: Create a counter

```graphql
mutation {
  instances {
    entities {
      create(
        type: "counter",
        id: "93419a15-ee61-449e-b942-1d6bc5230218",
        properties: [
          {
            name: "trigger",
            value: false
          },
          {
            name: "result",
            value: 0
          }
        ]
      ) {
        id
        type {
          name
        }
        properties(
          names: [
            "result"
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

### GraphQL: Increase counter

```graphql
mutation {
  instances {
    entities {
      update(
        id: "93419a15-ee61-449e-b942-1d6bc5230218",
        properties: [
          {
            name: "trigger",
            value: true
          }
        ]
      ) {
        id
        type {
          name
        }
        properties(
          names: [
            "result"
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

### GraphQL: Reset counter

```graphql
mutation {
  instances {
    entities {
      update(
        id: "93419a15-ee61-449e-b942-1d6bc5230218",
        properties: [
          {
            name: "trigger",
            value: false
          },
          {
            name: "result",
            value: 0
          }
        ]
      ) {
        id
        type {
          name
        }
        properties(
          names: [
            "result"
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
