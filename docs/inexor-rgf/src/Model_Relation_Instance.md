# Model: Relation Instance

A relation instance is a connection between an outbound [entity instance](./Model_Entity_Instance.md) and an inbound
entity instance. The relation instance has a type, the relation type. The [relation type](./Model_Relation_Type.md)
defines which entity type the outbound entity type and which entity type the inbound entity type must have.

## Data Model

| Field       | DataType                                               | Description                                  |
|-------------|--------------------------------------------------------|----------------------------------------------|
| Outbound    | [EntityInstance](./Model_Entity_Instance.md)           | The outbound entity instance                 |
| Type        | [RelationType](./Model_Relation_Type.md)               | The relation type                            |
| Inbound     | [EntityInstance](./Model_Entity_Instance.md)           | The inbound entity instance                  |
| Description | String                                                 | Textual description of the relation instance |
| Properties  | Vec<[Property Instance](./Model_Property_Instance.md)> | The properties                               |
| Components  | Vec<String>                                            | The currently applied components             |
| Behaviours  | Vec<String>                                            | The currently applied behaviours             |

## Graph

```mermaid
graph LR;
    A(Outbound Entity Instance)===>|"Relation Instance"|B(Inbound Entity Instance);
```

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
    Relation-Type {
        string name
        string fullname
        string namespace
        string description
    }
    Relation-Instance {
        string name
        string fullname
        string namespace
        string description
    }
    Property-Type {
        string name
        string description
        enum DataType
        enum SocketType
    }
    Property-Instance {
        string name
        JSON value
    }
    Entity-Instance ||--}o Property-Instance : stores
    Entity-Instance o{--}o Relation-Instance : outbound
    Entity-Instance o{--}o Relation-Instance : inbound
    Relation-Instance ||--}o Property-Instance : stores
    Relation-Instance o{--|| Relation-Type : is-a
    Property-Instance o{--|| Property-Type : is-a
    Entity-Instance o{--|| Entity-Type : is-a
    Entity-Type ||--}o Property-Type : defines
    Relation-Type ||--}o Property-Type : defines
    Entity-Type ||--}o Relation-Type : outbound
    Entity-Type ||--}o Relation-Type : inbound
```

## GraphQL

```admonish tip "GraphQL"
* [GraphQL Queries and Mutations](./GraphQL_API_Relation_Instances.md)
```
