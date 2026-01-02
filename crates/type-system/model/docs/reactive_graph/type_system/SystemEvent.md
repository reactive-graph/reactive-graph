# EntityType `SystemEvent`

## Fully Qualified Namespace

`reactive_graph::type_system::SystemEvent`

## Description

Events of the type system

## Components

| Component                       | Description                                                                                       | Properties                        |
|---------------------------------|---------------------------------------------------------------------------------------------------|-----------------------------------|
| `reactive_graph::core::Event`   | This component spawns events.                                                                     | <ul compact><li>`event`</li></ul> |
| `reactive_graph::core::Labeled` | The label is a hierarchical path with static segments, named parameters and catch-all parameters. | <ul compact><li>`label`</li></ul> |

## Properties from components

| name  | description                                                                       | data_type | socket_type | mutability |
|-------|-----------------------------------------------------------------------------------|-----------|-------------|------------|
| event | On receiving a boolean trigger the action will be executed                        | Any       | Output      | Mutable    |
| label | Hierarchical path with static segments, named parameters and catch-all parameters | String    | None        | Mutable    |

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

### Property `label`

Hierarchical path with static segments, named parameters and catch-all parameters

#### Data Type

<details><summary><code>String</code></summary>Represents a JSON string.</details>

#### Socket Type

<details><summary><code>None</code></summary>The property doesn't act as input or
output socket.</details>

#### Mutability

<details><summary><code>Mutable</code></summary>The property is mutable.</details>

## JSON Schema

[https://schema.reactive-graph.io/schema/json/dynamic_graph/types/entity/reactive_graph/type_system/SystemEvent.schema.json]()

```json
{
  "$id": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/entity/reactive_graph/type_system/SystemEvent.schema.json",
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "description": "Events of the type system",
  "properties": {
    "$id": {
      "default": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/entity/reactive_graph/type_system/SystemEvent.schema.json",
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
    },
    "id": {
      "format": "uuid",
      "type": "string"
    },
    "label": {
      "description": "Hierarchical path with static segments, named parameters and catch-all parameters",
      "type": "string"
    }
  },
  "required": [
    "label",
    "event",
    "id"
  ],
  "title": "reactive_graph::type_system::SystemEvent",
  "type": "object"
}
```
