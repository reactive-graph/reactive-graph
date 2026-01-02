# RelationType `HasComment`

## Fully Qualified Namespace

`reactive_graph::flow::HasComment`

## Description

Any entity within a flow can be commented

## Outbound Entity

`*`

## Inbound Entity

`reactive_graph::flow::Comment`

## JSON Schema

[https://schema.reactive-graph.io/schema/json/dynamic_graph/types/relation/reactive_graph/flow/HasComment.schema.json]()

```json
{
  "$id": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/relation/reactive_graph/flow/HasComment.schema.json",
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "description": "Any entity within a flow can be commented",
  "properties": {
    "$id": {
      "default": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/relation/reactive_graph/flow/HasComment.schema.json",
      "description": "The schema identifier",
      "type": "string"
    },
    "inbound_id": {
      "format": "uuid",
      "type": "string"
    },
    "instance_id": {
      "type": "string"
    },
    "outbound_id": {
      "format": "uuid",
      "type": "string"
    }
  },
  "required": [
    "outbound_id",
    "instance_id",
    "inbound_id"
  ],
  "title": "reactive_graph::flow::HasComment",
  "type": "object"
}
```
