# Inexor Reactive Graph Flow

| Project | Module | Sub-Module | Functionality | Tests Coverage |
| --- | --- | --- | --- | --- |
| Reactive Graph Flow | Core | Model | Bidule | TODO |

### About Inexor

<a href="https://inexor.org/">
<img align="right" width="200" height="200" src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-core-model/main/docs/images/inexor_2.png">
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

This module provides a very simple FRP library built over functional concepts (see README below). This is a fork of https://github.com/phaazon/bidule

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">]()
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/workflow/status/aschaeffer/inexor-rgf-core-bidule/Rust">](https://github.com/aschaeffer/inexor-rgf-core-bidule/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/aschaeffer/inexor-rgf-core-bidule">]()
[<img src="https://img.shields.io/github/languages/code-size/aschaeffer/inexor-rgf-core-bidule">]()
[<img src="https://img.shields.io/codecov/c/github/aschaeffer/inexor-rgf-core-bidule">]()

[<img src="https://img.shields.io/github/license/aschaeffer/inexor-rgf-core-bidule">](https://github.com/aschaeffer/inexor-rgf-core-model/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

# bidule, a Rust FRP crate

**bidule** is a very simple FRP library built over functional concepts. The basic, most core concept
is the `Stream<Sig>`, which is a *stream of typed signals*. A stream of signals will get a *signal*
as input and will broadcast it downwards. You can compose streams with each other with very simple
combinators, such as `map`, `filter`, `filter_map`, `zip`, `unzip`, `merge`, `fold`, `sink`, etc.

**bidule** is intended to be used directly as-is and can be the starting point of any higher
abstracted FRP-driven programming (e.g. video game, GUI, animation, etc.).

Feel free to have a look at the documentation for a better understanding on how everything composes
and work.

### Thanks to

* https://github.com/phaazon/bidule
* https://github.com/xd009642/tarpaulin
* https://codecov.io/
