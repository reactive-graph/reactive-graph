# Events: System Events

System events are events that are emitted by the system itself.  There is no other way to get this information than
through these events.

## Entity Types

| Entity Type  | Component | Property | Data Type | Socket Type |
|--------------|-----------|----------|-----------|-------------|
| System Event | event     | event    | Any       | Output      |
|              | labeled   | label    | String    | None        |

## Types of Events

| Label                                          | Description                                       | Payload                                   |
|------------------------------------------------|---------------------------------------------------|-------------------------------------------|
| `/org/inexor/events/type/component/created`    | Triggered if a component has been created         | Name of the created component             |
| `/org/inexor/events/type/component/deleted`    | Triggered if a component has been deleted         | Name of the deleted component             |
| `/org/inexor/events/type/entity/created`       | Triggered if an entity type has been created      | Name of the created entity type           |
| `/org/inexor/events/type/entity/deleted`       | Triggered if an entity type has been deleted      | Name of the deleted entity type           |
| `/org/inexor/events/type/relation/created`     | Triggered if a relation type has been created     | Name of the created relation type         |
| `/org/inexor/events/type/relation/deleted`     | Triggered if a relation type has been deleted     | Name of the deleted relation type         |
| `/org/inexor/event/type/changed`               | Triggered if the type system has changed          |                                           |
| `/org/inexor/events/instance/entity/created`   | Triggered if an entity instance has been created  | UUID of the created entity instance       |
| `/org/inexor/events/instance/entity/deleted`   | Triggered if an entity instance has been deleted  | UUID of the deleted entity instance       |
| `/org/inexor/events/instance/relation/created` | Triggered if a relation instance has been created | Edge key of the created relation instance |
| `/org/inexor/events/instance/relation/deleted` | Triggered if a relation instance has been deleted | Edge key of the deleted relation instance |
| `/org/inexor/events/flow/created`              | Triggered if a flow has been created              | UUID of the created flow                  |
| `/org/inexor/events/flow/deleted`              | Triggered if a flow has been deleted              | UUID of the deleted flow                  |

```admonish tip "Label"
Subscribing to these events is easily possible with a label.
```

## GraphQL Subscription

It is possible to subscribe to these events via GraphQL subscription.

```admonish tip "Transform and Zip Multiple Events"
Instead of subscribing multiple events directly you can zip multiple events and subscribe to the result.
```

### Get new components

```graphql
subscription getSystemEventComponentCreated {
  entity(label: "/org/inexor/event/type/component/created", propertyName: "event") {
    name
    value
    type {
      dataType
      socketType
    }
  }
}
```

### Get which components have been deleted

```graphql
subscription getSystemEventComponentDeleted {
  entity(label: "/org/inexor/event/type/component/deleted", propertyName: "event") {
    name
    value
    type {
      dataType
      socketType
    }
  }
}
```

### Get new flows

```graphql
subscription getSystemEventFlowCreated {
  entity(label: "/org/inexor/event/instance/flow/created", propertyName: "event") {
    name
    value
    type {
      dataType
      socketType
    }
  }
}
```

### Get which flows have been deleted

```graphql
subscription getSystemEventCFlowDeleted {
  entity(label: "/org/inexor/event/instance/flow/deleted", propertyName: "event") {
    name
    value
    type {
      dataType
      socketType
    }
  }
}
```
