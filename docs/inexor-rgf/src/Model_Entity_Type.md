# Model: Entity Type

An entity type defines the characteristics that are common to all entity instances. In particular, an entity type
defines which components it is combined from. Additional properties can also be defined.

## Data Model

| Field       | DataType                                       | Description                                                                         | Example       |
|-------------|------------------------------------------------|-------------------------------------------------------------------------------------|---------------|
| Name        | String                                         | The name of the entity type                                                         | player        |
| Namespace   | String                                         | The namespace                                                                       | logical_gates |
| Description | String                                         | Textual description of the entity type                                              |               |
| Components  | Vec<[Component](./Model_Component.md)>         | The components which composes the entity type. These provides additional properties |               |
| Properties  | Vec<[Property Type](./Model_Property_Type.md)> | The additional properties on entity instances                                       |               |
| Extensions  | Vec<[Extension](./Model_Extension.md)>         | A list of extensions which contains additional information                          |               |

## ER Diagram

```mermaid
erDiagram
    Entity-Type {
        string name
        string namespace
        string description
    }
    Entity-Instance {
        string id
        string label
        string description
    }
    Extension {
        string name
        JSON extension
    }
    Component {
        string name
        string description
    }
    Property-Type {
        string name
        string description
        enum DataType
        enum SocketType
    }
    Relation-Type {
        string name
        string fullname
        string namespace
        string description
    }
    Entity-Type o{--}o Component : composes
    Entity-Type ||--}o Property-Type : defines
    Relation-Type ||--}o Property-Type : defines
    Component ||--}o Property-Type : defines
    Entity-Type ||--}o Relation-Type : outbound
    Entity-Type ||--}o Relation-Type : inbound
    Relation-Type o{--}o Component : composes
    Entity-Type ||--}o Extension : has
    Relation-Type ||--}o Extension : has
    Property-Type ||--}o Extension : has
    Entity-Instance ||--}o Entity-Type : is-a
```

## GraphQL

```admonish tip "GraphQL"
* [GraphQL Queries and Mutations](./GraphQL_API_Entity_Types.md)
```
