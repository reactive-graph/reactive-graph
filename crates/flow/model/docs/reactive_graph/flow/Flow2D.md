# Component `Flow2D`

## Fully Qualified Namespace

`reactive_graph::flow::Flow2D`

## Description

The position (x,y) of the entity or relation on a two dimensional flow.

## Properties

| name | description                      | data_type | socket_type | mutability |
|------|----------------------------------|-----------|-------------|------------|
| f2dh | The height (y-axis) in a 2D flow | Number    | None        | Mutable    |
| f2dw | The width (x-axis) in a 2D flow  | Number    | None        | Mutable    |
| f2dx | The X position in a 2D flow      | Number    | None        | Mutable    |
| f2dy | The Y position in a 2D flow      | Number    | None        | Mutable    |

### Property `f2dh`

The height (y-axis) in a 2D flow

#### Data Type

<details><summary><code>Number</code></summary>Represents a JSON number, whether
integer or floating point.</details>

#### Socket Type

<details><summary><code>None</code></summary>The property doesn't act as input or
output socket.</details>

#### Mutability

<details><summary><code>Mutable</code></summary>The property is mutable.</details>

### Property `f2dw`

The width (x-axis) in a 2D flow

#### Data Type

<details><summary><code>Number</code></summary>Represents a JSON number, whether
integer or floating point.</details>

#### Socket Type

<details><summary><code>None</code></summary>The property doesn't act as input or
output socket.</details>

#### Mutability

<details><summary><code>Mutable</code></summary>The property is mutable.</details>

### Property `f2dx`

The X position in a 2D flow

#### Data Type

<details><summary><code>Number</code></summary>Represents a JSON number, whether
integer or floating point.</details>

#### Socket Type

<details><summary><code>None</code></summary>The property doesn't act as input or
output socket.</details>

#### Mutability

<details><summary><code>Mutable</code></summary>The property is mutable.</details>

### Property `f2dy`

The Y position in a 2D flow

#### Data Type

<details><summary><code>Number</code></summary>Represents a JSON number, whether
integer or floating point.</details>

#### Socket Type

<details><summary><code>None</code></summary>The property doesn't act as input or
output socket.</details>

#### Mutability

<details><summary><code>Mutable</code></summary>The property is mutable.</details>

## JSON Schema

[https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/flow/Flow2D.schema.json]()

```json
{
  "$id": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/flow/Flow2D.schema.json",
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "description": "The position (x,y) of the entity or relation on a two dimensional flow.",
  "properties": {
    "$id": {
      "default": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/flow/Flow2D.schema.json",
      "description": "The schema identifier",
      "type": "string"
    },
    "f2dh": {
      "description": "The height (y-axis) in a 2D flow",
      "type": "number"
    },
    "f2dw": {
      "description": "The width (x-axis) in a 2D flow",
      "type": "number"
    },
    "f2dx": {
      "description": "The X position in a 2D flow",
      "type": "number"
    },
    "f2dy": {
      "description": "The Y position in a 2D flow",
      "type": "number"
    }
  },
  "required": [
    "f2dh",
    "f2dx",
    "f2dw",
    "f2dy"
  ],
  "title": "reactive_graph::flow::Flow2D",
  "type": "object"
}
```
