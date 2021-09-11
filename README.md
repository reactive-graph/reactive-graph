# Inexor Reactive Graph Flow

| Project | Module | Sub-Module | Functionality | Tests |
| --- | --- | --- | --- | --- |
| Reactive Graph Flow | Application | | <img src="https://img.shields.io/badge/state-refactoring-yellow"> | [<img src="https://img.shields.io/codecov/c/github/aschaeffer/inexor-rgf-application">](https://app.codecov.io/gh/aschaeffer/inexor-rgf-application) |

### About Inexor

<a href="https://inexor.org/">
<img align="right" width="200" height="200" src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-application/main/docs/images/inexor_2.png">
</a>

* Inexor will be a new first-person shooter game which is based on a new octree-based game engine.
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

### About this application

This repository provides the application which results in a binary.

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/workflow/status/aschaeffer/inexor-rgf-application/Rust">](https://github.com/aschaeffer/inexor-rgf-application/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/aschaeffer/inexor-rgf-application">]()
[<img src="https://img.shields.io/github/languages/code-size/aschaeffer/inexor-rgf-application">]()
[<img src="https://img.shields.io/codecov/c/github/aschaeffer/inexor-rgf-application">](https://app.codecov.io/gh/aschaeffer/inexor-rgf-application)

[<img src="https://img.shields.io/github/license/aschaeffer/inexor-rgf-application">](https://github.com/aschaeffer/inexor-rgf-application/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### Configure Logging

1. Edit `config/logging.yml`
2. In the section `loggers` add or modify the logger for a specific module
    ```yaml
    loggers:
       inexor_rgf_plugin_mqtt::behaviour::relation::mqtt_subscribes:
       level: info
    ```

#### Configure plugins

1. Edit `config/plugins.toml`
2. Add a section for the plugin. The name must match the crate name of the plugin. Specify the path to the
   dynamically linked library. The path can be absolute or relative to the working directory.

    ```toml
    [[plugin]]
    name = "inexor-rgf-plugin-mqtt"
    active = true
    path = "../inexor-rgf-plugin-mqtt/target/debug/libinexor_rgf_plugin_mqtt.so"
    ```

#### Configure HTTP/GraphQL server

1. Edit `config/graphql.toml`
2. Configure the hostname and port

    ```toml
    hostname = "localhost"
    port = 31415
    ```

#### Start the application

TODO

#### Architecture

TODO: Entity Manager, ...


#### GraphQL

The most important interface for interaction with the Reactive Graph Flow is GraphQL. The GraphQL interface is useful for
* Interaction with other instances (C2S, S2C, S2S, P2P)
  * Because of the use of uuids synchronization is possible
* User Interfaces
  * In-game-menu
  * Server admin UI
  * Flow-Editor (Create / modify / test flows without being in-game)
  * Websites (community, clan, content-repositories, ...)
  * Tools (procedural-texture-editor)
  * Third-party (export/import from/to third party software)

* As a flow designer I can define components which can be used as building block for entities using a GraphQL interface
* As a flow designer I can create entities with properties using a GraphQL interface
* As a flow designer I can create relations with properties using a GraphQL interface
* As a flow designer I can connect and disconnect two properties which data flows from one to the other using a GraphQL interface
* As a flow designer I can create flows using a GraphQL interface

#### Using Libraries

The libraries are carefully chosen:

- [Dependency Injection: waiter_di](https://crates.io/crates/waiter_di)
- [Embedded Graph Database: indradb](https://crates.io/crates/indradb)
- [FRP / Reactive Streams: bidule](https://crates.io/crates/bidule)
- [Logging: log4rs](https://crates.io/crates/log4rs)
- [HTTP Server: actix-web](https://crates.io/crates/actix-web)
- [GraphQL Server: async-graphql](https://crates.io/crates/async-graphql)
- [Serialization/Deserialization: serde_json](https://crates.io/crates/serde-json)

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

| | | |
| --- | --- | --- |
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-application/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses! |
