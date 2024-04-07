# Plugin: HTTP

This plugin provides a reactive http client integration. Using entities of type `http` requests via HTTP can be made.

With this plugin it's possible to integrate external services into your home automation.

## Entity Types

| Name    | Property         | Data Type | Socket Type |
|---------|------------------|-----------|-------------|
| http    | url              | string    | input       |
|         | method           | string    | input       |
|         | request_headers  | object    | input       |
|         | payload          | object    | input       |
|         | response_headers | object    | output      |
|         | result           | object    | output      |
|         | status           | number    | output      |
|
| jsonrpc | url              | string    | input       |
|         | jsonrpc_version  | string    | input       |
|         | method           | string    | none        |
|         | params           | object    | input       |
|         | result           | object    | output      |
|         | error            | object    | output      |

## Platform Compatibility

| Platform | Compatibility |
|----------|:-------------:|
| Linux    |       ✓       |
| MacOS    |       ✓       |
| Windows  |       ✓       |

## Repository

| Name                   | Repository                                                                                                                                     |
|------------------------|------------------------------------------------------------------------------------------------------------------------------------------------|
| inexor-rgf-plugin-http | [https://github.com/reactive-graph/plugins-core/tree/main/plugins/http](https://github.com/reactive-graph/plugins-core/tree/main/plugins/http) |

## Usage

{{ graphql_playground(config="/examples/graphql/plugins/http/tabs.json") }}

## Usage

### GraphQL

#### Create HTTP Request

First create an entity instance of type `http` and specify the URL and the method.

```admonish tip "How to execute the HTTP Request"
By creating the entity instance no request will be made. The next example explains how to execute the request.
```

```graphql
mutation {
  instances {
    entities {
      create(
        type: "http",
        id: "a3370278-b05c-4d1a-ad57-cabd575c37e4",
        properties: [
          {
            name: "url",
            value: "https://api.sunrise-sunset.org/json?lat=47.557400&lng=9.707209&formatted=0"
          },
          {
            name: "method",
            value: "GET"
          },
          {
            name: "request_headers",
            value: {}
          },
          {
            name: "payload",
            value: {}
          },
          {
            name: "response_headers",
            value: {}
          },
          {
            name: "result",
            value: {}
          },
          {
            name: "status",
            value: 500
          }
        ]
      ) {
        id
        type {
          name
        }
        properties(
          names: [
            "url",
            "method",
            "request_headers",
            "payload",
            "response_headers",
            "result",
            "status"
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

#### Execute HTTP Request

By triggering the property `payload` the HTTP request will be executed and the results will be written into the
property `result`.

```graphql
mutation {
  instances {
    entities {
      update(
        id: "a3370278-b05c-4d1a-ad57-cabd575c37e4",
        properties: [
          {
            name: "payload",
            value: {}
          }
        ]
      ) {
        id
        type {
          name
        }
        properties(
          names: [
            "url",
            "method",
            "request_headers",
            "payload",
            "response_headers",
            "result",
            "status"
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
