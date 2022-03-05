# Plugin: Random

Generate random numbers

## Entity Types

| Name                     | Property | Data Type | Socket Type | Note      |
|--------------------------|----------|-----------|-------------|-----------|
|                          |
| RandomNumber             | trigger  | bool      | input       |           |
|                          | result   | number    | output      |           |
|                          |
| RandomIntegerWithinRange | trigger  | bool      | input       |           |
|                          | low      | number    | output      | Inclusive |
|                          | high     | number    | output      | Exclusive |
|                          | result   | number    | output      |           |

## Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |

## Repository

| Name                     | Repository                                             |
|--------------------------|--------------------------------------------------------|
| inexor-rgf-plugin-random | https://github.com/aschaeffer/inexor-rgf-plugin-random |

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
