# Plugin Binary

This plugin provides two entity types for loading binary data from a file into a property as
a Data-URL with BASE64 encoding and for storing da property which contains a Data-URL with
BASE64 encoding into a file.

This is the base for loading textures, sounds, maps and any other type of binary data.

## Data URL

Example of a data URL:

```
data:image/png;base64,
iVBORw0KGgoAAAANSUhEUgAAAAoAAAAKCAYAAACNMs+9AAAABGdBTUEAALGP
C/xhBQAAAAlwSFlzAAALEwAACxMBAJqcGAAAAAd0SU1FB9YGARc5KB0XV+IA
AAAddEVYdENvbW1lbnQAQ3JlYXRlZCB3aXRoIFRoZSBHSU1Q72QlbgAAAF1J
REFUGNO9zL0NglAAxPEfdLTs4BZM4DIO4C7OwQg2JoQ9LE1exdlYvBBeZ7jq
ch9//q1uH4TLzw4d6+ErXMMcXuHWxId3KOETnnXXV6MJpcq2MLaI97CER3N0
vr4MkhoXe0rZigAAAABJRU5ErkJggg==
```

## Components

| Name        | Property | Data Type | Socket Type | Description                                |
|-------------|----------|-----------|-------------|--------------------------------------------|
| binary_data | data_url | string    | none        | Data-URL, BASE64 encoded                   |

## Entity Types

Feed the streams with binary data from files.

| Name       | Component   | Property | Data Type | Socket Type | Description                                |
|------------|-------------|----------|-----------|-------------|--------------------------------------------|
|            |             |          |           |             |                                            |
| LoadBinary |             | filename | string    | input       | The filename to load the binary data from  |
|            | binary_data | data_url | string    | output      | Data-URL, BASE64 encoded                   |
| SaveBinary |             | filename | string    | input       | The filename to store the binary data into |
|            | binary_data | data_url | string    | input       | Data-URL, BASE64 encoded                   |

## Web Resources

Download binary resources via HTTP.

| HTTP Method | Resource Pattern                        | Description                                                              |
|-------------|-----------------------------------------|--------------------------------------------------------------------------|
| GET         | /binary/entities/{uuid}/{property_name} | Converts the Data-URL into a binary data and returns it as web resource  |
| GET         | /binary//entities/label/*label          | Converts the Data-URL into a binary data and returns it as web resource  |


### Examples

| Request                                                            | Entity Instance                           | Property Name |
|--------------------------------------------------------------------|-------------------------------------------|---------------|
| `GET /binary/entity/dce4bd25-7b25-4a6a-8567-5429a2b3a101/data_url` | id=`dce4bd25-7b25-4a6a-8567-5429a2b3a101` | `data_url`    |
| `GET /binary/entity/label/org/inexor/logo/data_url`                | label=`/org/inexor/logo/{:property}`      | `data_url`    |

## Platform Compatibility

| Platform | Compatibility |
|----------|---------------|
| Linux    | ✓             |
| MacOS    | ✓             |
| Windows  | ✓             |

## Repository

| Name                     | Repository                                             |
|--------------------------|--------------------------------------------------------|
| inexor-rgf-plugin-binary | https://github.com/aschaeffer/inexor-rgf-plugin-binary |

## Resources

* https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URIs
