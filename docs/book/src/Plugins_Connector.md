# Plugin Connector

## What is a `Connector`?

A connector connects a property of the `outbound entity instance` with a property of the `inbound entity
instance` and **propagates** the changes of the value.

During propagation a propagation function is called. The propagation function has one single input (the
incoming value). Connectors of different types has different propagation functions.

The propagation function can only do simple things (like casting or logging) but in fact even these
simple operations makes the control flow much simpler and much more readable.

#### How does a connector work?

The connector is a relation instance which connects two entity instances. The relation itself stores
the names of the output property and the input property.

In theory, it's also possible to connect two properties of the same entity instance.

On construction the streams are connected.

On destruction of the connector, the stream will be removed.

---
**Warning**

1. Connecting properties of the same entity instance is discouraged to prevent feedback loops
2. No type checks are performed on construction (yet; you are responsible)
3. There is no check about feedback loops (yet; you are responsible)
4. Renaming the relation properties (outbound_property_name, inbound_property_name) doesn't have any
   effect (yet). You have to remove the old connector and recreate a new connector.

---

## Components

| Name                | Description                                                                             | Properties             | Data Type | Socket Type |
|---------------------|-----------------------------------------------------------------------------------------|------------------------|-----------|-------------|
| connector           | Connects two properties                                                                 | outbound_property_name | string    | none        |
|                     |                                                                                         | inbound_property_name  | string    | none        |
|                     |
| buffer              | A buffer for FIFOs and interpolation                                                    | buffer_size            | number    | none        |
|                     |                                                                                         | buffer                 | array     | none        |
|                     |
| propagation_counter | Counts connector propagations. This component can be applied on all types of connectors | propagation_count      | number    | none        |

## Relation Types

| Name                            | Components | Description                                                                                        |
|---------------------------------|------------|----------------------------------------------------------------------------------------------------|
| buffered_fifo_connector         | connector  | This connector propagates the first inserted value of the FIFO buffer with the given size          |
|                                 | buffer     |
| debounce_connector              | connector  | This connector propagates the value if and only if the value is different                          |
| debug_connector                 | connector  | This connector logs the value before propagation (log level debug)                                 |
| default_connector               | connector  | This is the default connector type, which simply does nothing than propagate the value             |
| delay_connector                 | connector  | This connector propagates the value after a given duration. This operation is blocking             |
| numeric_interpolation_connector | connector  | This connector propagates the average of the numeric elements in the buffer                        |
|                                 | buffer     |
| parse_float_connector           | connector  | This connector parses a string value and propagates a float value                                  |
| parse_int_connector             | connector  | This connector parses a string value and propagates a int value                                    |
| to_string_connector             | connector  | This connector converts the value of any type to string before propagation                         |
| trace_connector                 | connector  | This connector logs the value before propagation (log level trace)                                 |
| increment_by_connector          | connector  | This connector adds the value of the outbound property to the value of the inbound property        |
| decrement_by_connector          | connector  | This connector subtracts the value of the outbound property from the value of the inbound property |

### Future: More (useful) connectors

| Name                | Components | Description                                                              |
|---------------------|------------|--------------------------------------------------------------------------|
| str_split_connector | connector  | A string is split into tokens. Propagates an JSON array of string tokens |
| str_join_connector  | connector  | Joins an array of strings and propagates the resulting string            |

## Platform Compatibility

| Platform | Compatibility |
|----------|:-------------:|
| Linux    |       ✓       |
| MacOS    |       ✓       |
| Windows  |       ✓       |

## Repository

| Name                        | Repository                                                                                                                                               |
|-----------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------|
| inexor-rgf-plugin-connector | [https://github.com/reactive-graph/plugins-core/tree/main/plugins/connector](https://github.com/reactive-graph/plugins-core/tree/main/plugins/connector) |

## Usage

{{ graphql_playground(config="/examples/graphql/plugins/connector/tabs.json") }}

## Usage

### GraphQL: Create a new connector

```graphql
mutation {
  instances {
    relations {
      createConnector: create(
        edgeKey: {
          outboundId: "dc82735e-28ec-4c24-aedb-d968b73b288f",
          typeName: "default_connector--value--value"
          inboundId: "4cf8e6d8-f92e-4ffa-9610-ec0635f55e08",
        },
        properties: [
          {
            name: "outbound_property_name",
            value: "value"
          },
          {
            name: "inbound_property_name",
            value: "value"
          }
        ]
      ) {
        type {
          name
          fullName
        },
        properties(
          names: [
            "outbound_property_name",
            "inbound_property_name"
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
