# Inexor Reactive Graph Flow

| Project | Module | Sub-Module | Functionality | Tests |
| --- | --- | --- | --- | --- |
| Reactive Graph Flow | Core | Plugins | Incomplete | 0% |

### About Inexor

<a href="https://inexor.org/">
<img align="right" width="200" height="200" src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-core-plugins/main/docs/images/inexor_2.png">
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

This module provides the plugin mechanism of the reactive graph flow. The main application and all plugins will depend on this.

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/workflow/status/aschaeffer/inexor-rgf-core-plugins/Rust">](https://github.com/aschaeffer/inexor-rgf-core-plugins/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/aschaeffer/inexor-rgf-core-plugins">]()
[<img src="https://img.shields.io/github/languages/code-size/aschaeffer/inexor-rgf-core-plugins">]()
[<img src="https://img.shields.io/codecov/c/github/aschaeffer/inexor-rgf-core-plugins">](https://app.codecov.io/gh/aschaeffer/inexor-rgf-core-plugins)

[<img src="https://img.shields.io/github/license/aschaeffer/inexor-rgf-core-plugins">](https://github.com/aschaeffer/inexor-rgf-core-plugins/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### The Plugin System

* Components can be provided as plugin
* Entity Types can be provided as plugin
* Relation Types can be provided as plugin

#### How to implement a plugin

* TODO

#### How to load a plugin at runtime or configure which plugins shall be loaded at startup

* TODO

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/
