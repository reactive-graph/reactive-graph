# Events

Events are (of course!) represented as reactive entity instances that have a component `event`. Since Inexor is already
a reactive system it wouldn't make sense to implement another event system. So, basically the events are conventions
rather than implementation.

## Component

Entity types or relation types can be composed with the component `event` in order to have an property `event`. 

| Component | Property | Data Type | Socket Type |
|-----------|----------|-----------|-------------|
| Event     | event    | Any       | Output      |
