# Inexor Reactive Graph Flow

| Project             | Module | Sub-Module | Functionality                                                        | Test Coverage                                                                                                                                          |
|---------------------|--------|------------|----------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reactive Graph Flow | Core   | Plugins    | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-core-plugins">](https://app.codecov.io/gh/inexorgame/inexor-rgf-core-plugins) |

### About Inexor

<a href="https://inexor.org/">
<img align="right" width="200" height="200" src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-core-plugins/main/docs/images/inexor_2.png">
</a>

* Inexor will be a new first person shooter game which is based on a new octree-based game engine.
* Inexor focuses on classic gameplay as we've seen in Cube2 or the Quake series.
* Inexor will be written from ground up new in C++17 and Rust.
* You can contribute anything you want: code, content, ideas..
* Inexor and all its content is 100% open source!

### About Inexor Reactive Graph Flow

The Inexor Reactive Graph Flow (RGF) manages reactive flows based on a graph database. The main interface is GraphQL.

* Semantic: Graph database with entities and relationships as first class citizens
* Reactive: entities and relationships are/can be reactive: If the input has been altered the entity processes its new state
* Interoperable: Use GraphQL for queries and mutations
* Extendable: Built in type system: components, entity types and relation types
* Memory efficient: Rust
* Fast: Rust
* Secure: Rust

### About this module

This module provides the plugin mechanism of the Inexor Reactive Graph Flow. The
[main application](https://github.com/inexorgame/inexor-rgf-application) and all
plugins will depend on this.

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/workflow/status/inexorgame/inexor-rgf-core-plugins/Rust">](https://github.com/inexorgame/inexor-rgf-core-plugins/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/inexorgame/inexor-rgf-core-plugins">]()
[<img src="https://img.shields.io/github/languages/code-size/inexorgame/inexor-rgf-core-plugins">]()
[<img src="https://img.shields.io/codecov/c/github/inexorgame/inexor-rgf-core-plugins">](https://app.codecov.io/gh/inexorgame/inexor-rgf-core-plugins)

[<img src="https://img.shields.io/github/license/inexorgame/inexor-rgf-core-plugins">](https://github.com/inexorgame/inexor-rgf-core-plugins/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### The Plugin System

One of the main goals of the Inexor Reactive Graph Flow that is as dynamic and universal as possible in order to create
new games, new maps, new logics. It borrows a lot of modern ideas to archive this goal:

* Flowgraph (like Node Red)
* Graph Databases (like Neo4J)
* Entity Component System
* Behaviour Driven Design
* Automatic Data Serialization / Deserialization
* Query Language: GraphQL
* 3D Visual Scripting (invented by Inexor!)

Therefore, entity component system is very flexible at runtime:

* Add new, modify or remove types (Components, Entity Type, Relation Type)
* Add new, modify or remove instances (Entity Instance, Relation Instance)
* Connect or disconnect properties (within Entity Instances or Relation Instances)

So, the data model is as flexible as possible. But the behaviour of `Entity Instance`s and `Relation Instance`s have to
be implemented in beforehand and requires compilation. For example, you could define a new `Entity Type` named `Random
Number Generator` but it simply doesn't do anything. In order to bring entities of this type to life, the behaviour of
the type would have to be implemented (and compiled into the binary).

To archive a true dynamic environment that can be extended with new behaviour, it is therefore necessary to offer a way
to add and remove behaviour to code that is not yet written. Therefore, we provide a plugin system.

A plugin is able to define new `Component`s, `Entity Type`s and `Relation Type`s. It is able to construct new `Entity
Instance`s, `Relation Instance`s and `Flow`s. And it provides new `Entity Behaviour`s and `Relation Behaviour`s. The
plugins are compiled separately as a dynamically linked library and are loaded at runtime.

As a bonus, the plugin system also enables the Inexor Reactive Graph Flow application to be as small as possible. If the
core of the application is small it is potentially useful for other use cases like IOT oder control software for
robotics - the plugins can provide such functionality and some functionalities are the very same (logical gates).

| Definition                           | Description                                                                                                                                                                                                     |
|--------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Component                            | Building block for the behaviour of an entity type or an relation type. Defines properties.                                                                                                                     |
| Entity Type, Relation Type           | Defines components to be used by the entity type or the relation type. Defines more properties.                                                                                                                 |
| Entity Behaviour, Relation Behaviour | The implementation of a specific behaviour of an entity or a relation of a specific type.                                                                                                                       |
| Flow                                 | The flow contains entity instances and relation instances and provides connected functionality. A flow is not flat. It can contain other flows. Therefore it's possible to reuse flows for different scenarios. |

TODO: Flow Type: Cause flows contains entity instances and relation instances, we need something like a template for a Flow. Like "Flow Type: Player" which is the template for "Flow Instance: Player 1" and "Flow Instance: Player 2"

#### Plugins can extend the type system of the Reactive Graph Flow

* As a developer I can provide a component using a plugin
* As a developer I can provide an entity type using a plugin
* As a developer I can provide a relation type using a plugin
(Future: Flow Types can be provided by a plugin)

#### Plugins can extend the behaviour of entities and relations

* As a developer I can provide an entity behaviour using a plugin
* As a developer I can provide a relation behaviour using a plugin

#### Plugins can provide connected functionality or bigger systems by providing flows

* As a developer I can provide a flow using a plugin

#### How to implement a plugin

* TODO

#### How to load a plugin at runtime or configure which plugins shall be loaded at startup

* TODO

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

|                                                                                                                                                                                                                             |           |                                                                    |
|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|--------------------------------------------------------------------|
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/inexorgame/inexor-rgf-core-plugins/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses!  |
