# Property Instance Subscriptions

```admonish tip "GraphQL Schema Visualization"
Explore the subscriptions using the [Plugin GraphQL Schema Visualization](./Plugins_GraphQL_Schema_Visualization.md)
[https://hostname:31415/graphql-schema-visualization/subscription](https://hostname:31415/graphql-schema-visualization/subscription)
```

## Subscribe changes of the key `right-ctrl`

```graphql
subscription keyDownRightCtrl {
  entity(
    label: "/org/inexor/input/any_device/key/key_rightctrl",
    propertyName: "key_down"
  ) {
    name
    value
    type {
      name
      dataType
      socketType
      extensions {
        name
        extension
      }
    }
  }
}
```
