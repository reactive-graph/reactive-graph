# Plugin Comparison

## Components

| Name            | Property | Data Type | Socket Type |
|-----------------|----------|-----------|-------------|
| comparison_gate | lhs      | any       | input       |
|                 | rhs      | any       | input       |
|                 | result   | bool      | output      |

## Entity Types

| Name                   | Components      | Description                                          |
|------------------------|-----------------|------------------------------------------------------|
| equals                 | comparison_gate | Returns true, if lhs and rhs are equal               |
| greater_than           | comparison_gate | Returns true, if lhs is greater than rhs             |
| greater_than_or_equals | comparison_gate | Returns true, if lhs is greater than or equal to rhs |
| lower_than             | comparison_gate | Returns true, if lhs is lower than rhs               |
| lower_than_or_equals   | comparison_gate | Returns true, if lhs is lower than or equal to rhs   |
| not_equals             | comparison_gate | Returns true, if lhs and rhs are not equal           |

## Platform Compatibility

| Platform | Compatibility |
|----------|:-------------:|
| Linux    |       ✓       |
| MacOS    |       ✓       |
| Windows  |       ✓       |

## Repository

| Name                         | Repository                                                                                                                                                     |
|------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------|
| inexor-rgf-plugin-comparison | [https://github.com/inexorgame/inexor-rgf-plugins/tree/main/plugins/comparison](https://github.com/inexorgame/inexor-rgf-plugins/tree/main/plugins/comparison) |

## Usage

{{ graphql_playground(config="/examples/graphql/plugins/comparison/tabs.json") }}
