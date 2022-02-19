# Development / Code Formatting

We use `rustfmt`. rustfmt is a tool for formatting Rust code according to style guidelines.

## Install rustfmt

```shell
rustup component add rustfmt --toolchain nightly
```

## Check formatting

```shell
cargo fmt --all -- --check
```

## Reformat code

```shell
cargo fmt --all
```
