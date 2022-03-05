# Plugin: JSON

This plugin adds functionality to operate with complex data structures. Properties of entity instances or relation
instances can have different data types. It's possible to store even complex data using the data  types array and
object. This is handy if you receive data from an MQTT endpoint or if you want to represent  more complex data. But it
makes it also necessary to unpack or pack these data in order to operate with it.

#### Entity Types

| Name                 | Property          | Data Type   | Socket Type |
|----------------------|-------------------|-------------|-------------|
|                      |
| ArrayPush            | array             | array       | input       |
|                      | to_be_added_value | any         | input       |
|                      | result            | array       | output      |
|                      |
| ArrayPop             | array             | array       | input       |
|                      | result            | array       | output      |
|                      | removed_value     | any         | input       |
|                      |
| ArrayGetByIndex      | array             | array       | input       |
|                      | index             | number      | output      |
|                      | result            | any         | output      |
|                      |
| ObjectSetProperty    | object            | object      | input       |
|                      | property_name     | string      | input       |
|                      | property_value    | any         | input       |
|                      | result            | object      | output      |
|                      |
| ObjectRemoveProperty | object            | object      | input       |
|                      | property_name     | string      | input       |
|                      | result            | object      | output      |
|                      | removed_value     | any         | output      |
|                      |
| ObjectGetProperty    | object            | object      | input       |
|                      | property_name     | string      | input       |
|                      | result            | any         | output      |

## Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |

## Repository

| Name                   | Repository                                           |
|------------------------|------------------------------------------------------|
| inexor-rgf-plugin-json | https://github.com/aschaeffer/inexor-rgf-plugin-json |
