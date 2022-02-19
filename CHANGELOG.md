# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased]

### Added

- Documentation: Added initial documentation as book (using mdBook)
- GraphQL: Navigate from components to entity types and to relation types
- GraphQL: Navigate from a property instance to the property type
- GraphQL: Shortcut labels
- GraphQL: Filter flows by flow type

### Changed

- GraphQL: Rename mutation types (Remove prefix GraphQl)
- Make extensions a separate type (used in components, entity types and relation types)
- GraphQL: Rename type_name to type
- GraphQL: Stream property instances in subscriptions instead of handcrafted JSONs
- Documentation: Convert code of conduct to markdown
- Documentation: Moved changelog into book

### Removed

## [0.5.0] - 2021-01-20

### Added

- Extended plugin API (find, import & export)
- Search for type names with wildcards

### Changed

- Refactored dependency injection library
- Updated dependencies of the dependency injection library

### Removed

- Removed dependencies for faster builds and smaller binaries (#8): `actix-http`, `actix-web`, `async-std`, `query_interface`, `inexor-rgf-core-reactive`
