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

## Entity Types

Feed the streams with binary data from files.

| Name       | Property | Data Type | Socket Type | Description                                |
|------------|----------|-----------|-------------|--------------------------------------------|
|            |
| LoadBinary | filename | string    | input       | The filename to load the binary data from  |
|            | data     | string    | output      | Data-URL, BASE64 encoded                   |
| SaveBinary | filename | string    | input       | The filename to store the binary data into |
|            | data     | string    | input       | Data-URL, BASE64 encoded                   |

## Web Resources

Download and upload binary resources via HTTP.

| HTTP Method | Resource Pattern                     | HTTP Header(s)           | Description                                                                    |
|-------------|--------------------------------------|--------------------------|--------------------------------------------------------------------------------|
| GET         | /entities/{uuid}/properties/{name}   | Accept: image/png        | Converts the Data-URL into a binary data and returns it as web resource        |
| POST        | /entities/{uuid}/properties/{name}   | Content-Type: image/png  | Converts the binary data into a Data-URL and stores it into the given property |

Alternative using GraphQL: A web application encodes the binary data to BASE64 and create a Data-URL. Then it sends a mutation query for the given property.

## Repository

| Name                     | Repository                                             |
|--------------------------|--------------------------------------------------------|
| inexor-rgf-plugin-binary | https://github.com/aschaeffer/inexor-rgf-plugin-binary |

## Resources

* https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URIs
