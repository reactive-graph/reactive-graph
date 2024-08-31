# Plugin: Value

This plugin provides value components. Value components contains a single property of a specific data type.
[State components](./Plugins_State.md) build on value components.

## Components

| *Component*          | *Properties* | *Data Type* | *Socket Type* | Description                           |
|----------------------|--------------|-------------|---------------|---------------------------------------|
|                      |
| value_boolean        | value        | boolean     | output        | A boolean value                       |
| value_number         | value        | number      | output        | A numeric value                       |
| value_string         | value        | string      | output        | A string value                        |
| value_array          | value        | array       | output        | A array value                         |
| value_object         | value        | object      | output        | A object value                        | 
|                      |
| value_debugger_debug |              |             |               | Debugger for values (log level debug) |
| value_debugger_trace |              |             |               | Debugger for values (log level trace) |

## Entity Types

| Name          | Components    | Description     |
|---------------|---------------|-----------------|
| value_array   | value_array   | A array value   |
| value_boolean | value_boolean | A boolean value |
| value_number  | value_number  | A numeric value |
| value_string  | value_string  | A string value  |
| value_object  | value_object  | A object value  |

## Platform Compatibility

| Platform | Compatibility |
|----------|:-------------:|
| Linux    |       ✓       |
| MacOS    |       ✓       |
| Windows  |       ✓       |

## Repositories

| Name                        | Repository                                                                                                                                       |
|-----------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------|
| reactive-graph-plugin-value | [https://github.com/reactive-graph/plugins-core/tree/main/plugins/value](https://github.com/reactive-graph/plugins-core/tree/main/plugins/value) |

## See also

* [States](./Plugins_State.md)
