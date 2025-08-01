# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

### Changed

### Fixed

### Distribution

### Infrastructure

## [0.10.0-alpha-5] - 2025-08-01

> [!NOTE]
> Improves plugin support in docker containers

### Added

- CLI: Added arguments `--danger_accept_invalid_certs` and `--danger_accept_invalid_hostnames`

### Changed

- Plugins: Improved error message if moving plugin from `plugins/deploy` to `plugins/installed` failed

### Fixed

- Docker: Run configurations for docker containers mount the plugin deploy folder of the workspace on the container (note: this works only if you're running a
  single container)
- Docker: Plugin system now correctly handles deploying plugins when the deploy folder is mounted as volume

### Distribution

- Docker: Dockerfile now uses ubuntu 24.04 as runtime image
- Docker: Added Dockerfile.debian which uses bookworm-slim as base image
- Docker: Added docker-compose which contains an init container for installing plugins (respects arch + configurable with a tag/version)

### Infrastructure

- The standard library plugins are now provided via [GitHub Releases](https://github.com/reactive-graph/std/releases/tag/nightly)

## [0.10.0-alpha-4] - 2025-07-28

### Added

- CLI: Added entity instances to flow types
- CLI: Added introspection queries
- CLI: Added command for getting the JSON Schema for each type in the type system
- Client: Added request method for getting the JSON Schema for each type in the type system
- Server CLI: Print GraphQL schemas
- Server CLI: Print JSON schemas
- GraphQL: Added entity instances to flow types
- GraphQL: Sort fields and enums
- GraphQL: Generate static schema SDL files during build
- GraphQL: Added resolver for the dynamic JSON schema for each type in the type system
- GraphQL: Added resolver for the dynamic JSON schema identifier ($id) for each type in the type system
- Dynamic Graph: Sort components, entity types, relation types and properties
- Dynamic Graph: Export reactive entities, relations and flows using dynamic schemas
- Dynamic Graph: Type System Metrics
- Dynamic Graph: Refactored field resolvers of the entity types
- Dynamic Graph: Refactored object resolvers of the flow types
- JSON Schema: Include an unique JSON schema identifier ($id) in the JSON schemas
- JSON Schema: Provide an $id field when serializing types and instances
- JSON Schema: Type definitions and instance definitions can refer to the JSON schema using an $id field
- JSON Schema: Generate dynamic schemas for all registered types in the type system
- Reactive: Added delegates for reactive flow to its wrapper entity
- Reactive: Added method to get reactive flow by entity type
- Reactive: Implemented json serialization for reactive entities and reactive relations
- Type System: Added traits JsonSchemaGetter and JsonSchemaIdGetter

### Changed

- Extended binary info with rustc_host_triple and rustc_commit_date
- Renamed test-utils to utils-test
- Configure lints on workspace level
- Replaced lazy_static with LazyLock
- CLI: Updated shell completions
- Dynamic Graph: Refactored GraphQL objects
- JSON schema: Refactored REST services
- Reactive: When creating a reactive flow from type a id can be provided

### Fixed

### Distribution

- Docker: Make debian bookworm slim the base image in order to allow plugins loading within the docker image

### Infrastructure

- CI: Synchronize labels from config file
- CI: Publish graphql and json schemas to https://schema.reactive-graph.io/
- CI: Check for breaking changes in the GraphQL schemas

## [0.10.0-alpha-3] - 2025-04-13

### Added

- Interfaces: Provide JSON schemas of the type system and instance system via GraphQL API
- Interfaces: Query for JSON Schemas of the type system and instance system via the Rust Client
- CLI: Print the JSON Schemas of the type system and instance system
- CLI: Manage flow types
- CLI: Manage flow instances
- GraphQL: Manage flow types
- GraphQL: Manage flow instances

### Changed

- Migrated to Rust Edition 2024
- Fixed clippy lints for Rust 1.85.0

### Fixed

- `#[unsafe(no_mangle)]`

### Distribution

- Docker: Added static labels (org.opencontainers.image)
- Docker: Added multi platform docker image (amd64 + arm64)

### Infrastructure

- CI: Update rust nightly toolchain to nightly-2025-03-14
- CI: Added GitHub Actions runner for arm64 ubuntu-22.04-arm
- CI: Generate release binaries and debian packages for arm64 using the arm64 runner
- CI: Automatically label pull requests
- CI: Automatically mark pull requests as stale
- CI: Automatically assign assignee and reviewers

## [0.10.0-alpha-2] - 2024-10-27

### Added

- CLI: Provisioning config files `reactive-graph instances init`
- CLI: Generate and provisioning certificates using `reactive-graph instances generate-certificate`
- CLI: Manage repositories of a local instance using `reactive-graph instances repository`
- CLI: Set config values of a local instance using `reactive-graph instances config graphql --secure true`
- Distribution: Dockerfile and run configurations for docker
- Docs: Documented installation via docker

### Changed

- Docs: Moved book to root folder
- Docs: Applied Reactive Graph Design System

### Fixed

- CLI: Self-Updater

### Infrastructure

- CI: Update rust nightly toolchain to nightly-2024-10-19
- CI: Auto-merge dependabot PRs

## [0.10.0-alpha-1] - 2024-10-20

### Added

- CLI: Print or install shell completions (bash, fish, zsh, powershell) and man pages
- CLI: Self Update `reactive-graph update`
- CLI: Init instance `reactive-graph instances init <path>`

### Infrastructure

- CI: Create tag nightly after merge and successful build on main branch
- CI: Publish nightly releases to GitHub Releases
- CI: Sign nightly tag with GPG
- CI: New layout for debian packages
- CI: Added support for `i686-unknown-linux-gnu` (32-bit)
- CI: Added support for `armv7-unknown-linux-musleabihf` (MUSL)
- CI: Added support for `x86_64-pc-windows-gnu` (MinGW)
- CI: Added support for `powerpc-unknown-linux-gnu`, `powerpc64-unknown-linux-gnu` and `powerpc64le-unknown-linux-gnu`
- CI: Added support for `riscv64gc-unknown-linux-gnu`
- CI: Check in lock file for security audit workflow
- CI: Automatic labeling

## [0.10.0-alpha-0] - 2024-03-24

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
