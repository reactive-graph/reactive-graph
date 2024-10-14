# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.10.0-2] - 2024-09-27

### Added

- CLI: Print or install shell completions (bash, fish, zsh, powershell) and man pages
- CLI: Tooling subcommands
- CI: Check in lock file
- CI: Create tag nightly after merge and successful build on main branch
- CI: Publish nightly releases

## [0.10.0-1] - 2024-03-24

This is a highly technical release with a lot of major refactorings and design changes.

### Highlights

- Modularized code base
- API separation
- Replaced dependency injection framework

### Added

- Added plural types (`EntityTypes` for a set of `EntityType`s) + Changed API to use plural types
- Handle reactive instances ReactiveEntity + ReactiveRelation like smart pointers (is this an anti-pattern?)
- Introduced thiserror for lots of error types
- Added test mocks using default_test in order to write tests easier
- Added declarative reactive models (reactive-derive + TypedReactivePropertyContainer)
- Added RelationTypeProviderRegistry and FlowTypeProviderRegistry
- Provide and use preludes
- Support JSON, JSON5 and TOML formats for importing and exporting types
- Added configuration for the location to the SSL certificate and private key
- Remotes Management
- JSON-Schemas
- CLI REPL + Improved CLI

### Changed

- Use TypedBuilder in core-model + migrate all calls to the new builders
- Refactored reactive layer out of core-model
- Moved reactive from core-model to reactive
- Refactored hashmap and vec to dashmap and dashset
- Implemented lots of conversions between types and instances
- Refactored import export managers into own services to separate concerns
- Grouped services in a business layer
- Moved tests into implementation classes to follow rust coding guidelines
- Modernized lots of tests with mocks
- Renamed model to inexor_rgf_graph
- Plugins can use springtime dependency injection + Added declarative macros for type providers
- Extended behaviour API: BehaviourFactoryCreator + BehaviourFunctions
- Moved behaviour functionality into behaviour_api and behaviour impl modules
- Moved reactive functionality into reactive_api and reactive impl modules
- Refactored error methods in dynamic graph to regular error types with thiserror
- Made handle_web_resource async
- Web resource providers must have an ID
- Make import export managers fully async
- Improved error types for import and export types

### Removed

- Removed IndraDB

## [0.9.1-25] - 2023-07-20

### Highlights

- Remotes Management
- JSON-Schemas
- CLI REPL + Improved CLI
- Documentation

### Added

- Remotes Management
- JSON-Schemas for all types and instances
- CLI REPL
- GraphQL: Filter types by properties, extensions and/or components
- GraphQL: Added counts in types
- GraphQL: Filter plugins by dependencies or unsatisfied dependencies
- GraphQL: Uninstall plugin
- GraphQL: Test getting all plugins
- Plugin System: Whitelist plugins with new configuration enabled_plugins
- Documentation: New chapter about the system architecture
- Documentation: New chapter about the RuntimeBuilder
- Documentation: New chapter limitations
- Documentation: Cross compilation and supported platforms
- Documentation: Packaging for debian

### Changed

- All APIs: Delete methods returns boolean
- CLI: refactored CLI tables from the client to the CLI crate
- Runtime: Improved state management in runtime and RuntimeBuilder
- Plugin System: Configuration file accepts short names
- Documentation: Updated sections about the model
- Documentation: Updated lots of GraphQL examples
- Documentation: Updated installation instructions

### Fixed

- Don't block runtime shutdown at a specific situation

## [0.9.1-22] - 2023-05-07

### Added

- Publish debian package for x86_64, aarch64 and armv7

## [0.9.1-14] - 2023-05-07

### Added

- Publish binaries for more targets, including Raspberry Pi

## [0.9.1-4] - 2023-05-07

### Added

- Configure the application using environment variables
- Generate Debian Package

## [0.9.1] - 2023-05-01

### Added

- First public release on crates.io

## [0.9.0-15] - 2023-03-26

### Added

- Configuration: Configure the instance
- Configuration: Configure the graphql server
- Configuration: Configure the plugins
- Configuration: Added config manager
- Plugin Resolver: Disable all plugins
- Plugin Resolver: Disable specific plugins
- Plugin Repository Manager: Remove duplicate plugins in plugin installation folder
- Executable: Added cli argument for the instance name and description
- Executable: Added cli argument for disabling all plugins
- Executable: Added cli argument for disabling a specific plugin
- Executable: Added cli argument for disabling hot deployment
- Executable: Added cli argument for configuring the hostname and port of the GraphQL server
- Executable: Added cli argument for configuring the shutdown timeout
- Type System: Allow merging types into existing types and plugin type providers are now merging instead of ignoring
  changes
- GraphQL: Simplify trigger action by providing a dedicated GraphQL mutation
- GraphQL: Allow sorting of type properties
- GraphQL: Allow sorting of type extensions

### Changed

- Workspace: Create mono repository
- Workspace: Migrate repository frp into workspace
- Workspace: Migrate repository model into workspace
- Workspace: Migrate repository builder into workspace
- Workspace: Migrate repository reactive into workspace
- Runtime: Refactor into runtime and binary
- Runtime: Use tokio as async runtime
- Plugin Resolver: Resolving plugins asynchronously
- Plugin Lifecycle: Make plugin activate and deactivate methods async
- GraphQL: Generate dynamic GraphQL schema using async-graphql 5.0.0

## [0.8.0] - 2022-02-01

### Added

- Plugins: Plugin resolver state machine
- Plugins: Plugin dependencies and plugin versioning
- Plugins: Plugin installation directory
- Plugins: Hot deployment
- Plugins: Gracefully shutdown and refresh plugins and their behaviours during deployment / hot deployment
- Plugins: GraphQL: List, start, stop and restart plugins
- Plugins: GraphQL: List unsatisfied dependencies
- Behaviours: Lifecycles (init, connect, disconnect, shutdown)
- Behaviours: A state machine manages the behaviour (connecting, disconnecting, reconnecting)
- Behaviours: Compute behaviour initially
- Behaviours: Validation layer for behaviours
- Behaviours: Factories can construct behaviour instances
- Behaviours: Handle behaviour construction errors
- Behaviours: Macro support for implementing behaviours
- Behaviours: Added GraphQL API for behaviours + Allow connecting and disconnecting behaviours manually (flow editor)
- Behaviours: Unit tests for lots of reactive behaviours
- Model: Introduce namespaces
- Model: Use concrete type system identifiers instead of strings
- API: Validate types on construction (validate that components or entity types are present)

### Changed

- Behaviours: Centralized behaviour storage (no more store behaviours in the plugins)
- Behaviours: Reimplemented lots of reactive behaviours
- Plugins: Split most plugins into a model and plugin crate
- API: Make service API and plugin API more consistent (same/similar method names + use type system identifiers)
- Tooling: rustfmt uses separate lines per import

## [0.7.0] - 2022-09-25

### Added

- Flow Types: Replicate and instantiate flows multiple times (like flow templates)
- Dynamic Graph: A second GraphQL api make the access to the instance system more intuitive
- Plugins: Avoid boilerplate using plugin macros
- Plugins: Metadata about the plugin
- Plugins: Dependency information
- Plugins: Expose GraphQL query service to plugins

### Changed

- Plugin: Dedicated error messages for the plugin API
- Plugin: Default trait implementations for the plugin API to reduce boilerplate
- Plugin: Return option none if a plugin doesn't use certain functionality

## [0.6.0] - 2022-02-26

### Added

- Documentation: Added initial documentation as book (using mdBook)
- GraphQL: Navigate from components to entity types and to relation types
- GraphQL: Navigate from a property instance to the property type
- GraphQL: Shortcut labels
- GraphQL: Filter flows by flow type
- GraphQL: Get flow by label
- GraphQL: Search instances by property value
- GraphQL: Added method to add/remove component to/from an existing instances
- GraphQL: Added filters for instances for applied components/behaviours
- Model: Components can have extensions
- Core: Provide required components and entity types in the application core
- Core: Event System for Type System and Instance System

### Changed

- GraphQL: Rename mutation types (Remove prefix GraphQl)
- GraphQL: Rename type_name to type
- GraphQL: Stream property instances in subscriptions instead of handcrafted JSONs
- GraphQL: Simplified label resolver
- GraphQL: Removed behaviour resolver from types
- Model: Make extensions a separate type (used in components, entity types and relation types)
- Model: Removed behaviours from types
- Model: Added list of applied components and behaviours to reactive instances
- Documentation: Convert code of conduct and changelog to markdown
- Documentation: Added code of conduct and changelog to book

## [0.5.0] - 2022-01-20

### Added

- Plugin API: Added find, import & export methods for all managers
- Core: Search for type names with wildcards

### Changed

- Refactored dependency injection library
- Updated dependencies of the dependency injection library

### Removed

- Plugin-API: Removed dependencies for faster builds and smaller plugin
  binaries: `actix-http`, `actix-web`, `async-std`, `query_interface`, `inexor-rgf-core-reactive`
