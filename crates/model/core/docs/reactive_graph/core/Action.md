# Component `Action`

## Fully Qualified Namespace

`reactive_graph::core::Action`

## Description

An action can be triggered

## Properties

| name    | description                                                | data_type | socket_type | mutability |
|---------|------------------------------------------------------------|-----------|-------------|------------|
| trigger | On receiving a boolean trigger the action will be executed | Bool      | Input       | Mutable    |

### Property `trigger`

On receiving a boolean trigger the action will be executed

#### Data Type

<details><summary><code>Bool</code></summary>Represents a JSON boolean.</details>

#### Socket Type

<details><summary><code>Input</code></summary>The property acts as input socket and
accepts incoming connections.</details>

#### Mutability

<details><summary><code>Mutable</code></summary>The property is mutable.</details>

## JSON Schema

[https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/core/Action.schema.json]()

```json
{
  "$id": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/core/Action.schema.json",
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "description": "An action can be triggered",
  "properties": {
    "$id": {
      "default": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/core/Action.schema.json",
      "description": "The schema identifier",
      "type": "string"
    },
    "trigger": {
      "description": "On receiving a boolean trigger the action will be executed",
      "type": "boolean"
    }
  },
  "required": [
    "trigger"
  ],
  "title": "reactive_graph::core::Action",
  "type": "object"
}
```
