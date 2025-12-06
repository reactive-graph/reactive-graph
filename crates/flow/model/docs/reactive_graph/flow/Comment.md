# EntityType `Comment`

## Fully Qualified Namespace

`reactive_graph::flow::Comment`

## Description

A simple comment

## Components

| Component                      | Description                                                                                            | Properties                                                                                                  |
|--------------------------------|--------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------|
| `reactive_graph::flow::Flow2D` | The position (x,y) of the entity or relation on a two dimensional flow.                                | <ul compact><li>`f2dh`</li><li>`f2dw`</li><li>`f2dx`</li><li>`f2dy`</li></ul>                               |
| `reactive_graph::flow::Flow3D` | The position (x,y,z) of the entity or relation on a three dimensional flow (in-game visual scripting). | <ul compact><li>`f3dd`</li><li>`f3dh`</li><li>`f3dw`</li><li>`f3dx`</li><li>`f3dy`</li><li>`f3dz`</li></ul> |

## Properties

| name    | description | data_type | socket_type | mutability |
|---------|-------------|-----------|-------------|------------|
| comment | Comment     | String    | None        | Mutable    |

### Property `comment`

Comment

#### Data Type

<details><summary><code>String</code></summary>Represents a JSON string.</details>

#### Socket Type

<details><summary><code>None</code></summary>The property doesn't act as input or
output socket.</details>

#### Mutability

<details><summary><code>Mutable</code></summary>The property is mutable.</details>

## Properties from components

| name | description                      | data_type | socket_type | mutability |
|------|----------------------------------|-----------|-------------|------------|
| f2dh | The height (y-axis) in a 2D flow | Number    | None        | Mutable    |
| f2dw | The width (x-axis) in a 2D flow  | Number    | None        | Mutable    |
| f2dx | The X position in a 2D flow      | Number    | None        | Mutable    |
| f2dy | The Y position in a 2D flow      | Number    | None        | Mutable    |
| f3dd | The depth (z-axis) in a 3D flow  | Number    | None        | Mutable    |
| f3dh | The height (y-axis) in a 3D flow | Number    | None        | Mutable    |
| f3dw | The width (x-axis) in a 3D flow  | Number    | None        | Mutable    |
| f3dx | The X position in a 3D flow      | Number    | None        | Mutable    |
| f3dy | The Y position in a 3D flow      | Number    | None        | Mutable    |
| f3dz | The Z position in a 3D flow      | Number    | None        | Mutable    |

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

[https://schema.reactive-graph.io/schema/json/dynamic_graph/types/entity/reactive_graph/flow/Comment.schema.json]()

```json
{
  "$id": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/entity/reactive_graph/flow/Comment.schema.json",
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "description": "A simple comment",
  "properties": {
    "$id": {
      "default": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/entity/reactive_graph/flow/Comment.schema.json",
      "description": "The schema identifier",
      "type": "string"
    },
    "comment": {
      "description": "Comment",
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
    },
    "id": {
      "format": "uuid",
      "type": "string"
    }
  },
  "required": [
    "f3dd",
    "f3dy",
    "f2dx",
    "f3dw",
    "f3dz",
    "f2dw",
    "f3dx",
    "f2dh",
    "f3dh",
    "comment",
    "f2dy",
    "id"
  ],
  "title": "reactive_graph::flow::Comment",
  "type": "object"
}
```
