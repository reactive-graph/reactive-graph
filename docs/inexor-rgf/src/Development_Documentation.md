# Development / Documentation

We use [mdBook](https://rust-lang.github.io/mdBook/index.html) for this documentation.

## Install

Please install these crates using cargo:

```shell
cargo install mdbook
cargo install mdbook-mermaid
cargo install mdbook-admonish
cargo install mdbook-preprocessor-graphql-playground
```

| Crate                                                                                                          | Description                                                           |
|----------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------|
| [mdbook](https://github.com/rust-lang/mdBook)                                                                  | Create book from markdown files. Like Gitbook but implemented in Rust |
| [mdbook-mermaid](https://github.com/badboy/mdbook-mermaid)                                                     | A preprocessor for mdbook to add mermaid support                      |
| [mdbook-admonish](https://github.com/tommilligan/mdbook-admonish)                                              | A preprocessor for mdbook to add Material Design admonishments        |
| [mdbook-preprocessor-graphql-playground](https://github.com/aschaeffer/mdbook-preprocessor-graphql-playground) | A preprocessor for mdbook to add GraphQL playgrounds                  |

## Build only

```shell
mdbook build
```

## Build and watch documentation locally

```shell
mdbook serve --open
```
