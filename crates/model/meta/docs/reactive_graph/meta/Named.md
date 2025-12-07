# Component `Named`

## Fully Qualified Namespace

`reactive_graph::meta::Named`

## Description

The entity or relation has a name.

## Properties

| name | description                         | data_type | socket_type | mutability |
|------|-------------------------------------|-----------|-------------|------------|
| name | The name of an entity or a relation | String    | None        | Mutable    |

### Property `name`

The name of an entity or a relation

#### Data Type

<details><summary><code>String</code></summary>Represents a JSON string.</details>

#### Socket Type

<details><summary><code>None</code></summary>The property doesn't act as input or
output socket.</details>

#### Mutability

<details><summary><code>Mutable</code></summary>The property is mutable.</details>

## JSON Schema

[https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/meta/Named.schema.json]()

```json
{
  "$id": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/meta/Named.schema.json",
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "description": "The entity or relation has a name.",
  "properties": {
    "$id": {
      "default": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/meta/Named.schema.json",
      "description": "The schema identifier",
      "type": "string"
    },
    "name": {
      "description": "The name of an entity or a relation",
      "type": "string"
    }
  },
  "required": [
    "name"
  ],
  "title": "reactive_graph::meta::Named",
  "type": "object"
}
```
