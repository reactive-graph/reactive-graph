# Component `Event`

## Fully Qualified Namespace

`reactive_graph::core::Event`

## Description

This component spawns events.

## Properties

| name  | description                                                | data_type | socket_type | mutability |
|-------|------------------------------------------------------------|-----------|-------------|------------|
| event | On receiving a boolean trigger the action will be executed | Any       | Output      | Mutable    |

### Property `event`

On receiving a boolean trigger the action will be executed

#### Data Type

<details><summary><code>Any</code></summary>Represents any type
(relations).</details>

#### Socket Type

<details><summary><code>Output</code></summary>The property acts as output socket and
accepts outgoing connections.</details>

#### Mutability

<details><summary><code>Mutable</code></summary>The property is mutable.</details>

## JSON Schema

[https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/core/Event.schema.json]()

```json
{
  "$id": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/core/Event.schema.json",
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "description": "This component spawns events.",
  "properties": {
    "$id": {
      "default": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/core/Event.schema.json",
      "description": "The schema identifier",
      "type": "string"
    },
    "event": {
      "description": "On receiving a boolean trigger the action will be executed",
      "type": [
        "null",
        "boolean",
        "number",
        "string",
        "array",
        "object"
      ]
    }
  },
  "required": [
    "event"
  ],
  "title": "reactive_graph::core::Event",
  "type": "object"
}
```
