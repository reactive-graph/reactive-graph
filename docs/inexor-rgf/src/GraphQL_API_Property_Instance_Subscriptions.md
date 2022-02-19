# Property Instance Subscriptions

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
