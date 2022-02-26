# Development / Build

## Install build tools (rust and rustup)

### Linux / Raspberry Pi / MacOS

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```admonish info "Rustup"
[rustup](https://rustup.rs/) is an installer for the systems programming language Rust.
```

### Windows

Please follow the instructions for [Installing rustup on Windows](https://rust-lang.github.io/rustup/installation/other.html)

## Install nightly toolchain

Once you have rustup up and running, please install the `nightly` toolchain.

```shell
rustup update nightly
```

## Clone the repository

```shell
git clone https://github.com/aschaeffer/inexor-rgf-application.git
```

## Build the application

Build in development mode:

```shell
cd inexor-rgf-application
cargo build
```

Build in release mode (takes longer, smaller binaries):

```shell
cd inexor-rgf-application
cargo build --release
```

```admonish info "Cargo"
[Cargo](https://doc.rust-lang.org/cargo/index.html) is the Rust package manager. Cargo downloads your Rust package's
dependencies, compiles your packages, makes distributable packages, and uploads them to [crates.io](https://crates.io),
the Rust community’s package registry.
```

## Run the application

```shell
cargo run
```

or:

```shell
cargo run --release
```

## Build plugins

### Checkout and build the plugin

 ```shell
 cd ..
 git clone https://github.com/aschaeffer/inexor-rgf-plugin-mqtt.git
 cd inexor-rgf-plugin-mqtt
 ```

Build in development mode:

 ```shell
 cargo build
 ```

Build in release mode:

```shell
cargo build --release
```

### Configure plugin

Edit `config/plugins.toml` and add a section for the plugin. The name must match the crate name of the plugin. Specify
the path to the dynamically linked library. The path can be either absolute or relative to the working directory of the
application.

```toml
[[plugin]]
name = "inexor-rgf-plugin-mqtt"
active = true
path = "../inexor-rgf-plugin-mqtt/target/debug/libinexor_rgf_plugin_mqtt.so"
```

```admonish tip "Artifact Location"
Release builds are located in `target/release` instead of `target/debug`
```

```admonish info "Configure plugins"
Please consult the documentation for [configuring plugins](./Configuration_Plugins.md)
```
