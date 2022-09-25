# Model

The data model of Inexor Reactive Graph Flow can be divided into the type system and the instance system.

## Type System

The type system is similar to type systems in programming languages. An object is of a specific, previously defined
type.

The type says something about the semantic meaning. For example, just looking at the type name gives you an idea of what
a "player" or a "camera" is, for example.

The type system also specifies the data model of such a semantic object. For example, a player has a name, is in a
position, and you know how many frags he has scored. Therefore, it is defined that a player should have a property
called "name".

Types are also related to each other. For example, a teleport entrance has one or more teleport exits. This relationship
between the teleport input type and the teleport output type is called a relation type. The relation type also has a
semantic meaning. In this case, the relation type "teleports_to" means that a player who is directly at the teleport
entrance should be teleported to the teleport exit.

Another example is a team that has one player as a member. There is a relation type called "is_member_of" that leads
from the entity type team to the entity type relation. Note the direction of the relation type.

This becomes even more relevant when there can be multiple distinct relationships between two entity types. A player can
be a member of a team and he can be the team captain at the same time. The first type of relationship has already been
described, the second has the name "is_captain_of".

### ER Diagram

```mermaid
erDiagram
    Relation-Type
    Entity-Type
    Extension
    Component
    Property-Type
    Relation-Type ||--}o Component : composes
    Relation-Type o{--|| Entity-Type : outbound
    Relation-Type o{--|| Entity-Type : inbound
    Entity-Type ||--}o Component : composes
    Relation-Type ||--}o Property-Type : defines
    Entity-Type ||--}o Property-Type : defines
    Relation-Type ||--}o Property-Type : defines
    Component ||--}o Property-Type : defines
    Relation-Type ||--}o Extension : has
    Entity-Type ||--}o Extension : has
    Property-Type ||--}o Extension : has
```

### Example: Relation Type

```mermaid
graph LR
    E1(Entity Type<br>Teleporter)
    E2(Entity Type<br>Teledestination)
    R1(Relation Type<br>Teleports_To)
    E1--->|outbound|R1
    R1--->|inbound|E2
```

### Example: Relation Types & Entity Types

```mermaid
graph TB
    P(Player)
    C(Camera)
    F(Flag)
    T(Team)
    B(Base)
    S(Playerstart)
    C--->|looks_at|P
    C--->|looks_at|F
    C--->|looks_at|B
    P--->|is_member_of|T
    P--->|frags|P
    T--->|is_located_at|B
    T--->|owns|F
    B--->|provides|S
    P--->|has_spawned_at|S
```

## Instance System

Having described the type system, this section describes the instance system.

The type system describes what could exist.

A type defines how an instance should look like. An instance itself fills this type with life. There can be any number
of instances of a type. For example, there is the player named "peter" and the player named "penacka". In this case
there are two instances of the same type.

The following table shows that an instance and the corresponding type:

| Type          | Instance          | 
|---------------|-------------------|
| Entity Type   | Entity Instance   |
| Relation Type | Relation Instance |
| Entity Type   | Flow Instance     |

```mermaid
erDiagram
    Relation-Type
    Relation-Instance
    Entity-Type
    Entity-Instance
    Flow-Instance
    Relation-Instance o{--|| Relation-Type : is_a
    Entity-Instance o{--|| Entity-Type : is_a
    Flow-Instance o{--|| Entity-Type : is_a
```

### Example: Relation Instances & Entity Instances

```mermaid
graph TB
    P1(Player 1)
    P2(Player 2)
    C1(Camera 1)
    C2(Camera 2)
    F1(Flag 1)
    F2(Flag 2)
    F3(Flag 3)
    F4(Flag 4)
    T1(Team Good)
    T2(Team Evil)
    B1(Base Good)
    B2(Base Evil)
    ST11(Playerstart Good 1)
    ST12(Playerstart Good 2)
    ST21(Playerstart Evil 1)
    ST22(Playerstart Evil 2)
    C1--->|looks_at|P1
    C2--->|looks_at|F2
    P1--->|is_member_of|T1
    P2--->|is_member_of|T2
    P1--->|fragged|P2
    P2--->|fragged|P1
    T1--->|is_located_at|B1
    T2--->|is_located_at|B2
    T1--->|owns|F1
    T1--->|owns|F2
    T2--->|owns|F3
    T2--->|owns|F4
    B1--->|provides|ST11
    B1--->|provides|ST12
    B2--->|provides|ST21
    B2--->|provides|ST22
    P1--->|has_spawned_at|ST11
    P2--->|has_spawned_at|ST22
    P1--->|stole|F3
    P2--->|stole|F2
```
