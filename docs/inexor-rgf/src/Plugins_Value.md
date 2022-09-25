# Plugin: Value

This plugin provides value and state components. States extends values with debounce mechanism. States are important for
several use cases, for example user interfaces and digital twins.

## State Management
The state management is an essential part of controlling external things which can change it's state by itself. An
example is a user interface checkbox, which can be toggled by the user. The state of the checkbox should be reflected in
the state property. On the other hand, the reactive graph flow should be able to change the state which should be
reflected by the user interface checkbox. This double-binding requires that the old internal state is stored and the new
state is compared with the old state. Debouncing the state is necessary to prevent feedback loops and undefined
behaviour.

## Use Cases

* User Interfaces: Checkboxes, Switches, ...
* States on remote systems (HTTP, GraphQL, MQTT)
* Digital Twins

## Components

| *Component*            | *Properties* | *Data Type* | *Socket Type* | Description                           |
|------------------------|--------------|-------------|---------------|---------------------------------------|
|                        |
| value_boolean          | value        | boolean     | output        | A boolean value                       |
| value_number           | value        | number      | output        | A numeric value                       |
| value_string           | value        | string      | output        | A string value                        |
| value_array            | value        | array       | output        | A array value                         |
| value_object           | value        | object      | output        | A object value                        | 
|                        |
| state_boolean          | state        | boolean     | none          | A boolean state                       |
|                        | set_state    | boolean     | input         |
| state_number           | state        | number      | none          | A numeric state                       |
|                        | set_state    | number      | input         |
| state_string           | state        | string      | none          | A string state                        |
|                        | set_state    | string      | input         |
| state_array            | state        | array       | none          | A array state                         |
|                        | set_state    | array       | input         |
| state_object           | state        | object      | none          | A object state                        |
|                        | set_state    | object      | input         |
|                        |
| value_debugger_debug   |              |             |               | Debugger for values (log level debug) |
| value_debugger_trace   |              |             |               | Debugger for values (log level trace) |
|                        |
| state_debugger_debug   |              |             |               | Debugger for states (log level debug) |
| state_debugger_trace   |              |             |               | Debugger for states (log level trace) |

## Entity Types

| Name          | Components    | Description     |
|---------------|---------------|-----------------|
|               |
| value_array   | value_array   | A array value   |
| value_boolean | value_boolean | A boolean value |
| value_number  | value_number  | A numeric value |
| value_string  | value_string  | A string value  |
| value_object  | value_object  | A object value  |
|               |
| state_array   | value_array   | A array state   |
|               | state_array   |                 |
| state_boolean | value_boolean | A boolean state |
|               | state_boolean |                 |
| state_number  | value_number  | A numeric state |
|               | state_boolean |                 |
| state_string  | value_string  | A string state  |
|               | state_boolean |                 |
| state_object  | value_object  | A object state  | 
|               | state_boolean |                 |

## Platform Compatibility

| Platform | Compatibility |
|----------|:-------------:|
| Linux    |       ✓       |
| MacOS    |       ✓       |
| Windows  |       ✓       |

## Repositories

| Name                    | Repository                                                                                                        |
|-------------------------|-------------------------------------------------------------------------------------------------------------------|
| inexor-rgf-plugin-value | [https://github.com/inexorgame/inexor-rgf-plugin-value](https://github.com/inexorgame/inexor-rgf-plugin-value)    |
