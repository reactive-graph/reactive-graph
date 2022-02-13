# 1. About Inexor

* Inexor will be a new first-person shooter game which is based on a new octree-based game engine.
* Inexor focuses on classic gameplay as we've seen in Cube2 or the Quake series.
* Inexor will be written from ground up new in C++17 and Rust.
* You can contribute anything you want: code, content, ideas...
* Inexor and all its content is 100% open source!

## Definitions

### Reactive (Behaviour / Programming)

* Every property is a stream not only data
* Streams can be subscribed and published

### Graph (Semantic / Directed / Property)

As first engine of the world Inexor introduces a real graph as foundation of the engine.

Main benefits are:

* A universal data structure for everything
* Relations are first class citizens
* Benefit from types and instances which makes things intuitive
* Benefit from navigation which is fast and intuitive
* Properties can store not only certain primitive data but complete documents

## Flow (Control Flow)

* Connectors connect the properties of entity instances or relation instances
* The control flow logic is based on top of the graph (see above) and reactive programming (see above)
* Flows will contain most of the logic:
  * game modes will be provided by flows
  * map logic will be provided by flows
