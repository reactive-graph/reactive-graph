# 1.1 Design Goals

## Plugin System

* Small core (outsource most functionality into plugins)
* The application needs to be restarted as little as possible
* The application has to be changed as seldom as possible
* Parts of the application can be reloaded or reloaded
* The file size of the application remains small and only what is necessary is required

## Core Application Goals

* A graph database for managing vertexes (aka entity) and edges (aka relation)
* Management of the type system (components, entity, relation)
* Management of the instances (entity, relation, flow)
* Management of reactive entities (ReactiveEntity, ReactiveRelation, ReactiveFlow)
* Management of plugins (configuration via TOML)
* Provide a GraphQL interface with access to the type system and the instances
* Lifecycle Management
* Logging

## Unique configuration

* Use TOML for configuration wherever possible
* The application can be adapted to the needs by configuration. Which plugins are loaded, can be determined. The intended use of the core application is not specified in advance. The core application can be used to implement a wide variety of applications without supplying unnecessary ballast

## Core Application Non-Goals

* Commands
* Configurations for purposes other than logging, plugins, GraphQL server
* System-Variables
* Meta data
* Implementation of certain reactive behaviors: e.g. logic gates, arithmetic gates
* Implementation of game modes, maps, mods
* Synchronization
* Flow-Editor
* Graphics-Rendering
* Octree-Editor
