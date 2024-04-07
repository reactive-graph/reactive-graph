# Plugin: Random

Generate random numbers, strings, UUIDs or booleans.

## Entity Types

| Name                        | Property | Data Type | Socket Type | Note                                        |
|-----------------------------|----------|-----------|-------------|---------------------------------------------|
|                             |
| pseudo_random_number        | trigger  | bool      | input       |                                             |
|                             | seed     | number    | input       | fixed, u64                                  |
|                             | result   | number    | output      |                                             |
|                             |
| random_bool                 | trigger  | bool      | input       |                                             |
|                             | result   | bool      | output      |                                             |
|                             |
| random_number               | trigger  | bool      | input       |                                             |
|                             | result   | number    | output      |                                             |
|                             |
| random_integer_within_range | trigger  | bool      | input       |                                             |
|                             | low      | number    | input       | Inclusive                                   |
|                             | high     | number    | input       | Exclusive                                   |
|                             | result   | number    | output      |                                             |
|                             |
| random_string               | trigger  | bool      | input       |                                             |
|                             | length   | number    | input       |                                             |
|                             | result   | string    | output      |                                             |
|                             |
| random_uuid                 | trigger  | bool      | input       |                                             |
|                             | result   | string    | output      | The generated UUID as string representation |

## Platform Compatibility

| Platform | Compatibility |
|----------|:-------------:|
| Linux    |       ✓       |
| MacOS    |       ✓       |
| Windows  |       ✓       |

## Repository

| Name                     | Repository                                                                                                                                         |
|--------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|
| inexor-rgf-plugin-random | [https://github.com/reactive-graph/plugins-core/tree/main/plugins/random](https://github.com/reactive-graph/plugins-core/tree/main/plugins/random) |

## Usage

{{ graphql_playground(config="/examples/graphql/plugins/random/number/tabs.json") }}

{{ graphql_playground(config="/examples/graphql/plugins/random/string/tabs.json") }}
