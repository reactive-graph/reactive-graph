# Plugin: Random

Generate random numbers

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

| Name                     | Repository                                                                                                       |
|--------------------------|------------------------------------------------------------------------------------------------------------------|
| inexor-rgf-plugin-random | [https://github.com/inexorgame/inexor-rgf-plugin-random](https://github.com/inexorgame/inexor-rgf-plugin-random) |

## Usage

<graphql-playground
  id="plugin-random-example-random-number-generator"
  title="Create random number generator"
  href="/examples/plugin-random-create-random-number-generator.graphql">
This example creates an entity which is a random number generator.
</graphql-playground>

<graphql-playground
  id="plugin-random-example-generate-random-number"
  title="Generate random number"
  href="/examples/plugin-random-generate-random-number.graphql">
This example uses the `random_number` generator to generate random numbers.<br>
Please run the example above first.<br>
Note that the property `trigger` is activated.
</graphql-playground>
