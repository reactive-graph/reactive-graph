# Plugin Config

Read configuration files (TOML) into an entity instance.

## Entity Types

| Name        | Properties    | Data Type | Socket Type |
|-------------|---------------|-----------|-------------|
| config_file | filename      | string    | none        |
|             | configuration | object    | output      |

## Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |

## Usage (GraphQL)

### Read in a configuration

```graphql
mutation {
  instances {
    entities {
      create(
        type: "config_file"
        id: "aed6c9b0-e495-4423-baeb-5597b66416f4"
        properties: [
          {
            name: "filename"
            value: "config/plugins.toml"
          }
        ]
      ) {
        id
        type {
          name
        }
        properties(
          names: [
            "filename"
            "configuration"
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

### Get all configurations
```graphql
query getAllConfigs {
  instances {
    entities(type: "config_file") {
      id
      label
      description
      properties {
        name
        value
      }
    }
  }
}
```

### "Reload" a configuration

By triggering the property `filename' the configuration file will be read again:

```graphql
mutation reloadPluginsConfig {
  instances {
    entities {
      update(
        id: "aed6c9b0-e495-4423-baeb-5597b66416f4",
        properties: [
          {
            name: "filename",
            value: "config/plugins.toml"
          }
        ]
      ) {
        id
        type {
          name
        }
        properties {
          name
          value
        }
      }
    }
  }
}
```
