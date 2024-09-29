# Events: System Events

System events are events that are emitted by the system itself. There is no other way to get this information than
through these events.

## Entity Types

| Entity Type  | Component | Property | Data Type | Socket Type |
|--------------|-----------|----------|-----------|-------------|
| System Event | event     | event    | Any       | Output      |
|              | labeled   | label    | String    | None        |

## Types of Events

| Label                                                 | Description                                       | Payload                                   |
|-------------------------------------------------------|---------------------------------------------------|-------------------------------------------|
| `/io/reactive-graph/events/type/component/created`    | Triggered if a component has been created         | Name of the created component             |
| `/io/reactive-graph/events/type/component/deleted`    | Triggered if a component has been deleted         | Name of the deleted component             |
| `/io/reactive-graph/events/type/entity/created`       | Triggered if an entity type has been created      | Name of the created entity type           |
| `/io/reactive-graph/events/type/entity/deleted`       | Triggered if an entity type has been deleted      | Name of the deleted entity type           |
| `/io/reactive-graph/events/type/relation/created`     | Triggered if a relation type has been created     | Name of the created relation type         |
| `/io/reactive-graph/events/type/relation/deleted`     | Triggered if a relation type has been deleted     | Name of the deleted relation type         |
| `/io/reactive-graph/event/type/changed`               | Triggered if the type system has changed          |                                           |
| `/io/reactive-graph/events/instance/entity/created`   | Triggered if an entity instance has been created  | UUID of the created entity instance       |
| `/io/reactive-graph/events/instance/entity/deleted`   | Triggered if an entity instance has been deleted  | UUID of the deleted entity instance       |
| `/io/reactive-graph/events/instance/relation/created` | Triggered if a relation instance has been created | Edge key of the created relation instance |
| `/io/reactive-graph/events/instance/relation/deleted` | Triggered if a relation instance has been deleted | Edge key of the deleted relation instance |
| `/io/reactive-graph/events/flow/created`              | Triggered if a flow has been created              | UUID of the created flow                  |
| `/io/reactive-graph/events/flow/deleted`              | Triggered if a flow has been deleted              | UUID of the deleted flow                  |

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
  entity(label: "/io/reactive-graph/event/type/component/created", propertyName: "event") {
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
  entity(label: "/io/reactive-graph/event/type/component/deleted", propertyName: "event") {
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
  entity(label: "/io/reactive-graph/event/instance/flow/created", propertyName: "event") {
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
  entity(label: "/io/reactive-graph/event/instance/flow/deleted", propertyName: "event") {
    name
    value
    type {
      dataType
      socketType
    }
  }
}
```
