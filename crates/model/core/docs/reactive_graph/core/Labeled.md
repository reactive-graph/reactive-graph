# Component `Labeled`

## Fully Qualified Namespace

`reactive_graph::core::Labeled`

## Description

The label is a hierarchical path with static segments, named parameters and catch-all
parameters.

## Properties

| name  | description                                                                       | data_type | socket_type | mutability |
|-------|-----------------------------------------------------------------------------------|-----------|-------------|------------|
| label | Hierarchical path with static segments, named parameters and catch-all parameters | String    | None        | Mutable    |

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

[https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/core/Labeled.schema.json]()

```json
{
  "$id": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/core/Labeled.schema.json",
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "description": "The label is a hierarchical path with static segments, named parameters and catch-all parameters.",
  "properties": {
    "$id": {
      "default": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/core/Labeled.schema.json",
      "description": "The schema identifier",
      "type": "string"
    },
    "label": {
      "description": "Hierarchical path with static segments, named parameters and catch-all parameters",
      "type": "string"
    }
  },
  "required": [
    "label"
  ],
  "title": "reactive_graph::core::Labeled",
  "type": "object"
}
```
