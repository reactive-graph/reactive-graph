# Development / Build

## Install build tools (rust and rustup)

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Install nightly toolchain

```shell
rustup update nightly
```

## Clone the repository

```shell
git clone https://github.com/aschaeffer/inexor-rgf-application.git
```

## Build the application

```shell
cd inexor-rgf-application
cargo build
```

Build in release mode:

```shell
cd inexor-rgf-application
cargo build --release
```

## Run the application

```shell
cargo run
```

## Build plugins

### Checkout and build the plugin

 ```shell
 cd ..
 git clone https://github.com/aschaeffer/inexor-rgf-plugin-mqtt.git
 cd inexor-rgf-plugin-mqtt
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

* Use either a relative or absolute path
* The filename of the library is different on windows (inexor_rgf_plugin_mqtt.dll)
* Release builds are located in `target/release` instead of `target/debug`
