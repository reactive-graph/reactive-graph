# Component `Command`

## Fully Qualified Namespace

`reactive_graph::command::Command`

## Description

A command which can be executed. The command has a name and can have command arguments.

## Properties

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

[https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/command/Command.schema.json]()

```json
{
  "$id": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/command/Command.schema.json",
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "description": "A command which can be executed. The command has a name and can have command arguments.",
  "properties": {
    "$id": {
      "default": "https://schema.reactive-graph.io/schema/json/dynamic_graph/types/component/reactive_graph/command/Command.schema.json",
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
    "namespace": {
      "description": "The command namespace",
      "type": "string"
    }
  },
  "required": [
    "args",
    "cmd_ignore",
    "cmd_result",
    "command",
    "help",
    "namespace"
  ],
  "title": "reactive_graph::command::Command",
  "type": "object"
}
```
