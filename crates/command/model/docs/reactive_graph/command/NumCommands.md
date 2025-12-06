# EntityType `NumCommands`

## Fully Qualified Namespace

`reactive_graph::command::NumCommands`

## Description

The number of commands

## Components

| Component                          | Description                                                                             | Properties                                                                                                                      |
|------------------------------------|-----------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------|
| `reactive_graph::command::Command` | A command which can be executed. The command has a name and can have command arguments. | <ul compact><li>`args`</li><li>`cmd_ignore`</li><li>`cmd_result`</li><li>`command`</li><li>`help`</li><li>`namespace`</li></ul> |

## Properties

| name | description | data_type | socket_type | mutability |
|------|-------------|-----------|-------------|------------|
| test |             | Bool      | None        | Mutable    |

### Property `test`

#### Data Type

<details><summary><code>Bool</code></summary>Represents a JSON boolean.</details>

#### Socket Type

<details><summary><code>None</code></summary>The property doesn't act as input or
output socket.</details>

#### Mutability

<details><summary><code>Mutable</code></summary>The property is mutable.</details>

## Properties from components

| name       | description                           | data_type | socket_type | mutability |
|------------|---------------------------------------|-----------|-------------|------------|
| args       | The command arguments                 | Array     | None        | Immutable  |
| cmd_ignore | blah                                  | Any       | None        | Mutable    |
| cmd_result | The result of the command             | Any       | Output      | Immutable  |
| command    | The command name                      | String    | None        | Immutable  |
| help       | Help text which explains the command. | String    | None        | Immutable  |
| namespace  | The command namespace                 | String    | None        | Immutable  |

### Property `args`

The command arguments

#### Data Type

<details><summary><code>Array</code></summary>Represents a JSON array.</details>

#### Socket Type

<details><summary><code>None</code></summary>The property doesn't act as input or
output socket.</details>

#### Mutability

<details><summary><code>Immutable</code></summary>The property is
immutable.</details>

### Property `cmd_ignore`

blah

#### Data Type

<details><summary><code>Any</code></summary>Represents any type
(relations).</details>

#### Socket Type

<details><summary><code>None</code></summary>The property doesn't act as input or
output socket.</details>

#### Mutability

<details><summary><code>Mutable</code></summary>The property is mutable.</details>

### Property `cmd_result`

The result of the command

#### Data Type

<details><summary><code>Any</code></summary>Represents any type
(relations).</details>

#### Socket Type

<details><summary><code>Output</code></summary>The property acts as output socket and
accepts outgoing connections.</details>

#### Mutability

<details><summary><code>Immutable</code></summary>The property is
immutable.</details>

### Property `command`

The command name

#### Data Type

<details><summary><code>String</code></summary>Represents a JSON string.</details>

#### Socket Type

<details><summary><code>None</code></summary>The property doesn't act as input or
output socket.</details>

#### Mutability

<details><summary><code>Immutable</code></summary>The property is
immutable.</details>

### Property `help`

Help text which explains the command.

#### Data Type

<details><summary><code>String</code></summary>Represents a JSON string.</details>

#### Socket Type

<details><summary><code>None</code></summary>The property doesn't act as input or
output socket.</details>

#### Mutability

<details><summary><code>Immutable</code></summary>The property is
immutable.</details>

### Property `namespace`

The command namespace

#### Data Type

<details><summary><code>String</code></summary>Represents a JSON string.</details>

#### Socket Type

<details><summary><code>None</code></summary>The property doesn't act as input or
output socket.</details>

#### Mutability

<details><summary><code>Immutable</code></summary>The property is
immutable.</details>

## JSON Schema

[https://schema.reactive-graph.io/schema/json/dynamic_graph/types/entity/reactive_graph/command/NumCommands.schema.json]()

```json
{
  "$id": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/entity/reactive_graph/command/NumCommands.schema.json",
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "description": "The number of commands",
  "properties": {
    "$id": {
      "default": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/entity/reactive_graph/command/NumCommands.schema.json",
      "description": "The schema identifier",
      "type": "string"
    },
    "args": {
      "description": "The command arguments",
      "type": "array"
    },
    "cmd_ignore": {
      "description": "blah",
      "type": [
        "null",
        "boolean",
        "number",
        "string",
        "array",
        "object"
      ]
    },
    "cmd_result": {
      "description": "The result of the command",
      "type": [
        "null",
        "boolean",
        "number",
        "string",
        "array",
        "object"
      ]
    },
    "command": {
      "description": "The command name",
      "type": "string"
    },
    "help": {
      "description": "Help text which explains the command.",
      "type": "string"
    },
    "id": {
      "format": "uuid",
      "type": "string"
    },
    "namespace": {
      "description": "The command namespace",
      "type": "string"
    },
    "test": {
      "type": "boolean"
    }
  },
  "required": [
    "command",
    "help",
    "cmd_ignore",
    "cmd_result",
    "test",
    "args",
    "namespace",
    "id"
  ],
  "title": "reactive_graph::command::NumCommands",
  "type": "object"
}
```
