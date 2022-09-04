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

### GraphQL

### Create random number generator

```graphql
mutation {
  instances {
    entities {
      create(
        type: "random_number",
        id: "24f1e42f-1072-4c39-a239-774af89286c6",
        properties: [
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

### Generate random number

```graphql
mutation {
  instances {
    entities {
      update(
        id: "24f1e42f-1072-4c39-a239-774af89286c6",
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
