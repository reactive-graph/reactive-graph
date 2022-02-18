# Model: Extension

There is information about a type that is useful for certain cases, but at the same time is not needed in every case.
The data model of the types should remain lean and not be overloaded. Nevertheless, future changes and new
functionalities should be made possible. However, this should be done in a way without becoming incompatible. This
extended information is called extensions.

An extension consists of two fields. On the one hand the name of the extension and on the other hand a field with
unstructured data in JSON format. This means that the data model of the extension is not specified. At the same time,
however, it is possible to provide any conceivable information. Because a JSON value is used, any complex and nested
information is possible. It is possible to deserialize the JSON value into a concrete data model.

An example of an extension is the information on how a flow editor should draw a shape of an entity type. The extension
is called "flow_shape" and contains information about the color, size, label and alignment of the shape. This example
shows that an extension can potentially contain very complex data structures and at the same time that an extension
covers a case that is not relevant in all cases. For example, if the "Flow Editor" plugin is not installed or not
activated, the extension is present but not used.

## List of known extensions

| Extension            | Description                                                            |
|----------------------|------------------------------------------------------------------------|
| dublin-core          | Meta data                                                              |
| flow_editor_palette  | An entry for the palette of the flow editor                            |
| flow_editor_shape    | Definition of the shape of an entity instance in the flow editor       |
| type_graph_shape     | Definition of the shape of an entity type in the type graph            |
| instance_graph_shape | Definition of the shape of an entity instance of in the instance graph |

## Extension `dublin_core`

```json
    {
      "name": "dublin-core",
      "extension":{
        "title": "Decrement number",
        "subject": "Decrement number",
        "creator": "Hanack"
      }
    }
```

## Extension `flow_editor_palette`

```json
{
  "name": "flow_editor_palette",
  "extension": {
    "content": "Decrement",
    "styles": {
      "font-family": "Fira Code",
      "font-size": "12px",
      "padding": "5px"
    }
  }
}
```

## Extension `flow_editor_shape`

```json
{
  "name": "flow_editor_shape",
  "extension": {
    "width": 200,
    "socket": {
      "width": 60,
      "height": 30,
      "offset": 5
    },
    "offset": {
      "top": "socket.height",
      "bottom": "socket.height"
    },
    "elements": {
      "title": {
        "show": true,
        "type": "text",
        "content": "element.description",
        "position": {
          "left": 0,
          "top": 0,
          "width": "shape.width",
          "height": "socket.height"
        },
        "styles": {
          "font-size": "12px",
          "color": "black"
        }
      },
      "symbol": {
        "show": true,
        "type": "text",
        "content": "--",
        "position": {
          "left": 0,
          "top": 0,
          "width": "shape.width",
          "height": "shape.height"
        },
        "styles": {
          "fill": "steelblue",
          "font-size": "80px",
          "font-family": "Fira Code"
        }
      },
      "id": {
        "show": true,
        "type": "text",
        "content": "shape.id",
        "position": {
          "left": 0,
          "top": "shape.height-socket.height",
          "width": "shape.width",
          "height": "socket.height"
        },
        "styles": {
          "font-size": "9px",
          "color": "black"
        }
      }
    }
  }
}
```

## GraphQL

* [GraphQL Queries and Mutations](./GraphQL_API_Extension.md)
