# Component `Flow3D`

## Fully Qualified Namespace

`reactive_graph::flow::Flow3D`

## Description

The position (x,y,z) of the entity or relation on a three dimensional flow (in-game visual
scripting).

## Properties

| name | description                      | data_type | socket_type | mutability |
|------|----------------------------------|-----------|-------------|------------|
| f3dd | The depth (z-axis) in a 3D flow  | Number    | None        | Mutable    |
| f3dh | The height (y-axis) in a 3D flow | Number    | None        | Mutable    |
| f3dw | The width (x-axis) in a 3D flow  | Number    | None        | Mutable    |
| f3dx | The X position in a 3D flow      | Number    | None        | Mutable    |
| f3dy | The Y position in a 3D flow      | Number    | None        | Mutable    |
| f3dz | The Z position in a 3D flow      | Number    | None        | Mutable    |

### Property `f3dd`

The depth (z-axis) in a 3D flow

#### Data Type

<details><summary><code>Number</code></summary>Represents a JSON number, whether
integer or floating point.</details>

#### Socket Type

<details><summary><code>None</code></summary>The property doesn't act as input or
output socket.</details>

#### Mutability

<details><summary><code>Mutable</code></summary>The property is mutable.</details>

### Property `f3dh`

The height (y-axis) in a 3D flow

#### Data Type

<details><summary><code>Number</code></summary>Represents a JSON number, whether
integer or floating point.</details>

#### Socket Type

<details><summary><code>None</code></summary>The property doesn't act as input or
output socket.</details>

#### Mutability

<details><summary><code>Mutable</code></summary>The property is mutable.</details>

### Property `f3dw`

The width (x-axis) in a 3D flow

#### Data Type

<details><summary><code>Number</code></summary>Represents a JSON number, whether
integer or floating point.</details>

#### Socket Type

<details><summary><code>None</code></summary>The property doesn't act as input or
output socket.</details>

#### Mutability

<details><summary><code>Mutable</code></summary>The property is mutable.</details>

### Property `f3dx`

The X position in a 3D flow

#### Data Type

<details><summary><code>Number</code></summary>Represents a JSON number, whether
integer or floating point.</details>

#### Socket Type

<details><summary><code>None</code></summary>The property doesn't act as input or
output socket.</details>

#### Mutability

<details><summary><code>Mutable</code></summary>The property is mutable.</details>

### Property `f3dy`

The Y position in a 3D flow

#### Data Type

<details><summary><code>Number</code></summary>Represents a JSON number, whether
integer or floating point.</details>

#### Socket Type

<details><summary><code>None</code></summary>The property doesn't act as input or
output socket.</details>

#### Mutability

<details><summary><code>Mutable</code></summary>The property is mutable.</details>

### Property `f3dz`

The Z position in a 3D flow

#### Data Type

<details><summary><code>Number</code></summary>Represents a JSON number, whether
integer or floating point.</details>

#### Socket Type

<details><summary><code>None</code></summary>The property doesn't act as input or
output socket.</details>

#### Mutability

<details><summary><code>Mutable</code></summary>The property is mutable.</details>

## JSON Schema

[https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/flow/Flow3D.schema.json]()

```json
{
  "$id": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/flow/Flow3D.schema.json",
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "description": "The position (x,y,z) of the entity or relation on a three dimensional flow (in-game visual scripting).",
  "properties": {
    "$id": {
      "default": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/flow/Flow3D.schema.json",
      "description": "The schema identifier",
      "type": "string"
    },
    "f3dd": {
      "description": "The depth (z-axis) in a 3D flow",
      "type": "number"
    },
    "f3dh": {
      "description": "The height (y-axis) in a 3D flow",
      "type": "number"
    },
    "f3dw": {
      "description": "The width (x-axis) in a 3D flow",
      "type": "number"
    },
    "f3dx": {
      "description": "The X position in a 3D flow",
      "type": "number"
    },
    "f3dy": {
      "description": "The Y position in a 3D flow",
      "type": "number"
    },
    "f3dz": {
      "description": "The Z position in a 3D flow",
      "type": "number"
    }
  },
  "required": [
    "f3dw",
    "f3dd",
    "f3dx",
    "f3dh",
    "f3dz",
    "f3dy"
  ],
  "title": "reactive_graph::flow::Flow3D",
  "type": "object"
}
```
