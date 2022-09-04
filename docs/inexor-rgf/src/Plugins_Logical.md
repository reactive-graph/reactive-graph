# Plugin Logical

This plugin provides logical gates and operations.

## Components

| Name              | Properties | DataType | SocketType | Description                                                |
|-------------------|------------|----------|------------|------------------------------------------------------------|
|                   |
| condition         | condition  | bool     | input      | Accepts a boolean condition and returns an result          |
|                   | result     | any      | output     | The datatype may be overridden by the concrete entity type |
|                   |
| action            | trigger    | bool     | input      | Triggers an action on `true`                               |
|                   | result     | any      | output     | The datatype may be overridden by the concrete entity type |
|                   |
| generator         | trigger    | bool     | input      | Produces triggers (signals boolean `true`)                 |
|                   |
| logical_gate      | lhs        | bool     | input      |                                                            |
|                   | rhs        | bool     | input      |                                                            |
|                   | result     | bool     | output     |                                                            |
|                   |
| logical_operation | lhs        | bool     | input      |                                                            |
|                   | result     | bool     | output     |                                                            |

## Entity Types

| Name         | Components        | Properties   | DataType | SocketType | Description                                                                           |
|--------------|-------------------|--------------|----------|------------|---------------------------------------------------------------------------------------|
| not          | logical_operation | lhs          | bool     | input      | NOT-Operation                                                                         |
|              | logical_operation | result       | bool     | output     |                                                                                       |
||
| and          | logical_gate      | lhs          | bool     | input      | AND-Gate                                                                              |
|              | logical_gate      | rhs          | bool     | input      |                                                                                       |
|              | logical_gate      | result       | bool     | output     |                                                                                       |
||
| nand         | logical_gate      | lhs          | bool     | input      | NAND-Gate                                                                             | 
|              | logical_gate      | rhs          | bool     | input      |                                                                                       |
|              | logical_gate      | result       | bool     | output     |                                                                                       |
||
| nor          | logical_gate      | lhs          | bool     | input      | NOR-Gate                                                                              |
|              | logical_gate      | rhs          | bool     | input      |                                                                                       |
|              | logical_gate      | result       | bool     | output     |                                                                                       |
||
| or           | logical_gate      | lhs          | bool     | input      | OR-Gate                                                                               |
|              | logical_gate      | rhs          | bool     | input      |                                                                                       |
|              | logical_gate      | result       | bool     | output     |                                                                                       |
||
| xor          | logical_gate      | lhs          | bool     | input      | XOR-Gate                                                                              |
|              | logical_gate      | rhs          | bool     | input      |                                                                                       |
|              | logical_gate      | result       | bool     | output     |                                                                                       |
||
| xnor         | logical_gate      | lhs          | bool     | input      | XNOR-Gate                                                                             |
|              | logical_gate      | rhs          | bool     | input      |                                                                                       |
|              | logical_gate      | result       | bool     | output     |                                                                                       |
||
| if_then_else | condition         | condition    | bool     | input      | Each time it's triggered, either the then-payload or the else-payload gets propagated |
|              |                   | then_payload | any      | input      | Will be propagated if the condition is `true`                                         |
|              |                   | else_payload | any      | input      | Will be propagated if the condition is `false`                                        |
|              | condition         | result       | any      | output     |                                                                                       |
||
| toggle       | action            | trigger      | bool     | input      | If triggered the result will toggled                                                  |
|              | action            | result       | bool     | output     |                                                                                       |
||
| trigger      | action            | trigger      | bool     | input      | If triggered the payload will be copied to the result                                 |
|              |                   | payload      | any      | input      |                                                                                       |
|              | action            | result       | any      | output     |                                                                                       |

## Platform Compatibility

| Platform | Compatibility |
|----------|:-------------:|
| Linux    |       ✓       |
| MacOS    |       ✓       |
| Windows  |       ✓       |

## Repository

| Name                      | Repository                                                                                                         |
|---------------------------|--------------------------------------------------------------------------------------------------------------------|
| inexor-rgf-plugin-logical | [https://github.com/inexorgame/inexor-rgf-plugin-logical](https://github.com/inexorgame/inexor-rgf-plugin-logical) |
