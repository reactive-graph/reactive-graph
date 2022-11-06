# Plugin: System Environment

The plugin creates entity instances for each environment variable. As environment variables doesn't change at runtime
this happens only at initialization.

## UUIDs

```admonish info
The UUIDs of the entity instances are stable.
```

## Labels

Each entity instance which represents a system environment variable has a label.

| System Env  | Label                         |
|-------------|-------------------------------|
| `$HOME`     | `/org/inexor/system/env/home` |
| `$PATH`     | `/org/inexor/system/env/path` |

## Entity Types

| Name   | Properties | Data Type | Socket Type |
|--------|------------|-----------|-------------|
| EnvVar | name       | string    | none        |
|        | label      | string    | none        |
|        | value      | string    | output      |

## Platform Compatibility

| Platform | Compatibility |
|----------|:-------------:|
| Linux    |       ✓       |
| MacOS    |       ✓       |
| Windows  |       ✓       |

## Repository

| Name                                 | Repository                                                                                                                               |
|--------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------|
| inexor-rgf-plugin-system-environment | [https://github.com/inexorgame/inexor-rgf-plugin-system-environment](https://github.com/inexorgame/inexor-rgf-plugin-system-environment) |

## Usage

{{ graphql_playground(config="/examples/graphql/plugin-system-environment/plugin-system-environment.json") }}