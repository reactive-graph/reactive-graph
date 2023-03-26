# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.9.0-3] - 2023-03-26

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
- Type System: Allow merging types into existing types and plugin type providers are now merging instead of ignoring changes
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

- Plugin-API: Removed dependencies for faster builds and smaller plugin binaries: `actix-http`, `actix-web`, `async-std`, `query_interface`, `inexor-rgf-core-reactive`
