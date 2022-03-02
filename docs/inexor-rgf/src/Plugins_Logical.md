# Plugin Logical

This plugin provides logical gates and operations.

## Components

| Name              | Properties | DataType | SocketType | Description |
|-------------------|------------|----------|------------|-------------|
| condition         | condition  | bool     | input      |             |
|                   | result     | any      | output     |             |
| logical_gate      | lhs        | bool     | input      |             |
|                   | rhs        | bool     | input      |             |
|                   | result     | bool     | output     |             |
| logical_operation | lhs        | bool     | input      |             |
|                   | result     | bool     | output     |             |

## Entity Types

| Name         | Components        | Properties   | DataType | SocketType | Description   |
|--------------|-------------------|--------------|----------|------------|---------------|
| not          | logical_operation | lhs          | bool     | input      | NOT-Operation |
|              | logical_operation | result       | bool     | output     |               |
||
| and          | logical_gate      | lhs          | bool     | input      | AND-Gate      |
|              | logical_gate      | rhs          | bool     | input      |               |
|              | logical_gate      | result       | bool     | output     |               |
||
| nand         | logical_gate      | lhs          | bool     | input      | NAND-Gate     | 
|              | logical_gate      | rhs          | bool     | input      |               |
|              | logical_gate      | result       | bool     | output     |               |
||
| nor          | logical_gate      | lhs          | bool     | input      | NOR-Gate      |
|              | logical_gate      | rhs          | bool     | input      |               |
|              | logical_gate      | result       | bool     | output     |               |
||
| or           | logical_gate      | lhs          | bool     | input      | OR-Gate       |
|              | logical_gate      | rhs          | bool     | input      |               |
|              | logical_gate      | result       | bool     | output     |               |
||
| xor          | logical_gate      | lhs          | bool     | input      | XOR-Gate      |
|              | logical_gate      | rhs          | bool     | input      |               |
|              | logical_gate      | result       | bool     | output     |               |
||
| xnor         | logical_gate      | lhs          | bool     | input      | XNOR-Gate     |
|              | logical_gate      | rhs          | bool     | input      |               |
|              | logical_gate      | result       | bool     | output     |               |
||
| if_then_else | condition         | condition    | any      | input      |               |
|              |                   | then_payload | any      | input      |               |
|              |                   | else_payload | any      | input      |               |
|              | condition         | result       | any      | output     |               |
||
| trigger      | condition         | condition    | any      | input      |               |
|              |                   | payload      | any      | input      |               |
|              | condition         | result       | any      | output     |               |

## Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |

## Repository

| Project             | Module | Sub-Module | Functionality                                                        | Tests                                                                                                                                                      |
|---------------------|--------|------------|----------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reactive Graph Flow | Plugin | Logical    | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/aschaeffer/inexor-rgf-plugin-logical">](https://app.codecov.io/gh/aschaeffer/inexor-rgf-plugin-logical) |
