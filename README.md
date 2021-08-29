# Inexor Reactive Graph Flow

### About Inexor

https://inexor.org/

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

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">]()
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/workflow/status/aschaeffer/inexor-rgf-shared-bidule/Inexor%20Reactive%20Semantic%20Entity%20Component%20System">](https://github.com/aschaeffer/inexor-rgf-shared-bidule/actions?query=workflow%3AInexor%20Reactive%20Semantic%20Entity%20Component%20System)
[<img src="https://img.shields.io/github/license/aschaeffer/inexor-rgf-shared-bidule">](https://github.com/aschaeffer/inexor-rgf-shared-bidule/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

[<img src="https://img.shields.io/github/contributors/aschaeffer/inexor-rgf-shared-bidule">]()
[<img src="https://img.shields.io/github/downloads/aschaeffer/inexor-rgf-shared-bidule/total?color=brightgreen">]()
[<img src="https://img.shields.io/github/last-commit/aschaeffer/inexor-rgf-shared-bidule">]()
[<img src="https://img.shields.io/github/issues/aschaeffer/inexor-rgf-shared-bidule">]()
[<img src="https://img.shields.io/github/languages/code-size/aschaeffer/inexor-rgf-shared-bidule">]()

[<img src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-shared-bidule/main/docs/images/inexor_2.png">]()

This module provides a shared library: Bidule

This is a fork of https://github.com/phaazon/bidule

Original readme below





# bidule, a Rust FRP crate

**bidule** is a very simple FRP library built over functional concepts. The basic, most core concept
is the `Stream<Sig>`, which is a *stream of typed signals*. A stream of signals will get a *signal*
as input and will broadcast it downwards. You can compose streams with each other with very simple
combinators, such as `map`, `filter`, `filter_map`, `zip`, `unzip`, `merge`, `fold`, `sink`, etc.

**bidule** is intended to be used directly as-is and can be the starting point of any higher
abstracted FRP-driven programming (e.g. video game, GUI, animation, etc.).

Feel free to have a look at the documentation for a better understanding on how everything composes
and work.
