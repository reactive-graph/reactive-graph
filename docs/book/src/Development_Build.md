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

Please follow the instructions
for [Installing rustup on Windows](https://rust-lang.github.io/rustup/installation/other.html)

## Install nightly toolchain

Once you have rustup up and running, please install the `nightly` toolchain.

```shell
rustup update nightly
```

## Clone the repository

```shell
git clone https://github.com/reactive-graph/reactive-graph.git
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

## Checkout new version and rebuild

1. Fetch the latest version from git:
    ```shell
    git pull
    ```
2. (Optional) If you want a completely clean build
    ```shell
    cargo clean
    ```
3. Update dependencies
    ```shell
    cargo update
    ```
4. Build the application
    ```shell
    cargo build
    ```
5. Repeat this for all plugins

```admonish warning "Plugin API version must match"
The application and the plugins must be compiled with the same version of the Plugin API!
```

## Build plugins

### Checkout and build the plugin

 ```shell
 cd ..
 git clone https://github.com/reactive-graph/plugin-mqtt.git
 cd plugin-mqtt
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
name = "plugin-mqtt"
active = true
path = "../plugin-mqtt/target/debug/libinexor_rgf_plugin_mqtt.so"
```

```admonish tip "Artifact Location"
Release builds are located in `target/release` instead of `target/debug`
```

```admonish info "Configure plugins"
Please consult the documentation for [configuring plugins](./Configuration_Plugins.md)
```

### Plugin Compatibility

```log
17:18:18.642 [ERROR] [main] inexor_rgf_application::implementation::plugin_registry_impl:198 - Cannot load plugin ../reactive-graph-plugin-flow-manager/target/debug/libinexor_rgf_plugin_flow_manager.so because of a compiler version mismatch: rustc 1.61.0-nightly (expected: 1.63.0-nightly)
```

```admonish warning "Rust Compiler"
The plugins have to be compiled by the same rust compiler.
```
