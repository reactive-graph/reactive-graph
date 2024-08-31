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
|
| add       | arithmetic_gate      | lhs        | number   | input      | Addition                                          |
|           | arithmetic_gate      | rhs        | number   | input      |                                                   |
|           | arithmetic_gate      | result     | number   | output     |                                                   |
|
| counter   | action               | trigger    | bool     | input      | If triggered, the result will be incremented by 1 |
|           | action               | result     | number   | output     |                                                   |
|
| decrement | arithmetic_operation | lhs        | number   | input      | Decrements the input by 1                         |
|           | arithmetic_operation | result     | number   | output     |                                                   |
|
| div       | arithmetic_gate      | lhs        | number   | input      | Division                                          |
|           | arithmetic_gate      | rhs        | number   | input      |                                                   |
|           | arithmetic_gate      | result     | number   | output     |                                                   |
|
| increment | arithmetic_operation | lhs        | number   | input      | Increments the input by 1                         |
|           | arithmetic_operation | result     | number   | output     |                                                   |
|
| max       | arithmetic_gate      | lhs        | number   | input      | Max value                                         |
|           | arithmetic_gate      | rhs        | number   | input      |                                                   |
|           | arithmetic_gate      | result     | number   | output     |                                                   |
|
| min       | arithmetic_gate      | lhs        | number   | input      | Min value                                         |
|           | arithmetic_gate      | rhs        | number   | input      |                                                   |
|           | arithmetic_gate      | result     | number   | output     |                                                   |
|
| mod       | arithmetic_gate      | lhs        | number   | input      | Modulo                                            |
|           | arithmetic_gate      | rhs        | number   | input      |                                                   |
|           | arithmetic_gate      | result     | number   | output     |                                                   |
|
| mul       | arithmetic_gate      | lhs        | number   | input      | Multiplication                                    |
|           | arithmetic_gate      | rhs        | number   | input      |                                                   |
|           | arithmetic_gate      | result     | number   | output     |                                                   |
|
| sub       | arithmetic_gate      | lhs        | number   | input      | Subtraction                                       |
|           | arithmetic_gate      | rhs        | number   | input      |                                                   |
|           | arithmetic_gate      | result     | number   | output     |                                                   |

## Platform Compatibility

| Platform | Compatibility |
|----------|:-------------:|
| Linux    |       ✓       |
| MacOS    |       ✓       |
| Windows  |       ✓       |

## Repository

| Name                             | Repository                                                                                                                                                 |
|----------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------|
| reactive-graph-plugin-arithmetic | [https://github.com/reactive-graph/plugins-core/tree/main/plugins/arithmetic](https://github.com/reactive-graph/plugins-core/tree/main/plugins/arithmetic) |

## Usage

{{ graphql_playground(config="/examples/graphql/plugins/arithmetic/add/tabs.json") }}

---

{{ graphql_playground(config="/examples/graphql/plugins/arithmetic/counter/tabs.json") }}
