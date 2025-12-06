# EntityType `SimpleFlow`

## Fully Qualified Namespace

`reactive_graph::flow::SimpleFlow`

## Description

A simple generic flow with a single input and a single output

## Properties

| name   | description | data_type | socket_type | mutability |
|--------|-------------|-----------|-------------|------------|
| input  | Flow input  | Object    | Input       | Mutable    |
| output | Flow output | Object    | Output      | Mutable    |

### Property `input`

Flow input

#### Data Type

<details><summary><code>Object</code></summary>Represents a JSON object.</details>

#### Socket Type

<details><summary><code>Input</code></summary>The property acts as input socket and
accepts incoming connections.</details>

#### Mutability

<details><summary><code>Mutable</code></summary>The property is mutable.</details>

### Property `output`

Flow output

#### Data Type

<details><summary><code>Object</code></summary>Represents a JSON object.</details>

#### Socket Type

<details><summary><code>Output</code></summary>The property acts as output socket and
accepts outgoing connections.</details>

#### Mutability

<details><summary><code>Mutable</code></summary>The property is mutable.</details>

## JSON Schema

[https://schema.reactive-graph.io/schema/json/dynamic_graph/types/entity/reactive_graph/flow/SimpleFlow.schema.json]()

```json
{
  "$id": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/entity/reactive_graph/flow/SimpleFlow.schema.json",
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "description": "A simple generic flow with a single input and a single output",
  "properties": {
    "$id": {
      "default": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/entity/reactive_graph/flow/SimpleFlow.schema.json",
      "description": "The schema identifier",
      "type": "string"
    },
    "id": {
      "format": "uuid",
      "type": "string"
    },
    "input": {
      "description": "Flow input",
      "type": "object"
    },
    "output": {
      "description": "Flow output",
      "type": "object"
    }
  },
  "required": [
    "output",
    "input",
    "id"
  ],
  "title": "reactive_graph::flow::SimpleFlow",
  "type": "object"
}
```
