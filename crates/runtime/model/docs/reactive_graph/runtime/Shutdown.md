# EntityType `Shutdown`

## Fully Qualified Namespace

`reactive_graph::runtime::Shutdown`

## Description

Shutting down the runtime

## Components

| Component                       | Description                                                                                       | Properties                          |
|---------------------------------|---------------------------------------------------------------------------------------------------|-------------------------------------|
| `reactive_graph::core::Action`  | An action can be triggered                                                                        | <ul compact><li>`trigger`</li></ul> |
| `reactive_graph::core::Labeled` | The label is a hierarchical path with static segments, named parameters and catch-all parameters. | <ul compact><li>`label`</li></ul>   |

## Properties

| name  | description | data_type | socket_type | mutability |
|-------|-------------|-----------|-------------|------------|
| delay |             | Number    | None        | Mutable    |

### Property `delay`

#### Data Type

<details><summary><code>Number</code></summary>Represents a JSON number, whether
integer or floating point.</details>

#### Socket Type

<details><summary><code>None</code></summary>The property doesn't act as input or
output socket.</details>

#### Mutability

<details><summary><code>Mutable</code></summary>The property is mutable.</details>

## Properties from components

| name    | description                                                                       | data_type | socket_type | mutability |
|---------|-----------------------------------------------------------------------------------|-----------|-------------|------------|
| label   | Hierarchical path with static segments, named parameters and catch-all parameters | String    | None        | Mutable    |
| trigger | On receiving a boolean trigger the action will be executed                        | Bool      | Input       | Mutable    |

### Property `label`

Hierarchical path with static segments, named parameters and catch-all parameters

#### Data Type

<details><summary><code>String</code></summary>Represents a JSON string.</details>

#### Socket Type

<details><summary><code>None</code></summary>The property doesn't act as input or
output socket.</details>

#### Mutability

<details><summary><code>Mutable</code></summary>The property is mutable.</details>

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

[https://schema.reactive-graph.io/schema/json/dynamic_graph/types/entity/reactive_graph/runtime/Shutdown.schema.json]()

```json
{
  "$id": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/entity/reactive_graph/runtime/Shutdown.schema.json",
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "description": "Shutting down the runtime",
  "properties": {
    "$id": {
      "default": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/entity/reactive_graph/runtime/Shutdown.schema.json",
      "description": "The schema identifier",
      "type": "string"
    },
    "delay": {
      "type": "number"
    },
    "id": {
      "format": "uuid",
      "type": "string"
    },
    "label": {
      "description": "Hierarchical path with static segments, named parameters and catch-all parameters",
      "type": "string"
    },
    "trigger": {
      "description": "On receiving a boolean trigger the action will be executed",
      "type": "boolean"
    }
  },
  "required": [
    "delay",
    "label",
    "trigger",
    "id"
  ],
  "title": "reactive_graph::runtime::Shutdown",
  "type": "object"
}
```
