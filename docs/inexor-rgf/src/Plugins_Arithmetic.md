# Plugin Arithmetic

This plugin provides arithmetic gates and operations.

## Components

| Name                 | Properties | DataType | SocketType | Description |
|----------------------|------------|----------|------------|-------------|
| arithmetic_gate      | lhs        | number   | input      |             |
|                      | rhs        | number   | input      |             |
|                      | result     | number   | output     |             |
| arithmetic_operation | lhs        | number   | input      |             |
|                      | result     | number   | output     |             |

## Entity Types

| Name       | Components           | Description               |
|------------|----------------------|---------------------------|
| add        | arithmetic_gate      | Addition                  |
| decrement  | arithmetic_operation | Decrements the input by 1 |
| div        | arithmetic_gate      | Division                  |
| increment  | arithmetic_operation | Increments the input by 1 |
| max        | arithmetic_gate      | Max value                 |
| min        | arithmetic_gate      | Min value                 |
| mod        | arithmetic_gate      | Modulo                    |
| mul        | arithmetic_gate      | Multiplication            |
| sub        | arithmetic_gate      | Subtraction               |

## Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |

## Repository

| Name                         | Repository                                                 |
|------------------------------|------------------------------------------------------------|
| inexor-rgf-plugin-arithmetic | https://github.com/aschaeffer/inexor-rgf-plugin-arithmetic |
