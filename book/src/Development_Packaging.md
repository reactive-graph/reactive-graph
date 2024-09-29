# Development / Packaging

## cargo (Rust Package Manager)

```shell
cargo publish NOT_YET_AVAILABLE
```

## Debian

We are creating debian packages using [cargo-deb](https://crates.io/crates/cargo-deb).

The configuration of the debian package is defined in `Cargo.toml`:

```toml
[package.metadata.deb]
name = "reactive-graph"
maintainer-scripts = "debian/maintainer-scripts"
assets = [
    ["target/release/reactive-graph", "usr/bin/reactive-graph", "755"],
    # more assets omitted...
]

[package.metadata.deb.systemd-units]
unit-name = "reactive-graph@"
enable = true
restart-after-upgrade = true
```

```admonish tip title = "Installation instructions for Debian"
For installation instructions for the debian packages please see [Installation](./Installation.md)
```

## snap (Linux)

* https://snapcraft.io/docs/rust-plugin
* https://snapcraft.io/docs/snap-confinement

### Configuration Files

* `snapcraft.yaml`
* `rust-toolchain.toml`

### Create Package

```shell
snapcraft
```

### Install Package

The snap have to be installed with `--devmode`.

```shell
sudo snap install --devmode reactive-graph_0.0.0_amd64.snap
```

### Package Information

```shell
snap info --verbose reactive-graph
```

### `snapcraft.yaml` Examples

* https://github.com/lenna-project/lenna-cli/blob/7c31c71d1dd060f0c922b3f8b5e87833b5c45600/snapcraft.yaml
* https://github.com/mimblewimble/packaging/blob/af8f34c3a3055be8907a7a2c98cbf63e23e792e3/snap/snapcraft.yaml

## RPM

(TODO)

* https://crates.io/crates/cargo-rpm

## Arch

(TODO)

* https://crates.io/crates/cargo-arch

## (Windows)

(TODO)

* https://crates.io/crates/msi
