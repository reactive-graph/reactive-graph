# Development / Supported Platforms

The inexor reactive graph flow is completely platform-agnostic, which allows it to run on various operating systems.

### Microsoft Windows

We support x64 Microsoft Windows 8, 8.1 and 10.

We have build instructions for Windows.

### Linux

We support x86_64 / arm7 / aarch Linux. Chances are that it works with other targets, too, but haven't been tested.

We have specific build instructions for Ubuntu and Raspberry Pi 2-4.

```admonish tip title = "libc"
The continuous integration is configured to use `ubuntu-20.04` in order to support older libc versions.
> target: x86_64-unknown-linux-gnu
> os: ubuntu-20.04
```

### macOS and iOS

We do not support macOS or iOS yet.

Though continuous integration builds are run and release artifacts are published.

### Android

We also do not support Android yet.

### Cross Compilation

It's possible to cross compile for another targets using [cross-rs](https://github.com/cross-rs/cross).

```shell
cargo install cross
```

| Host  | Target                 | Target Name                   | Command                                                             |
|-------|------------------------|-------------------------------|---------------------------------------------------------------------|
| Linux | ARM64 Linux            | aarch64-unknown-linux-gnu     | `cross +nightly build --release --target aarch64-unknown-linux-gnu` |
| Linux | ARMv7 Linux, hardfloat | armv7-unknown-linux-gnueabihf | `cross +nightly build --release --target aarch64-unknown-linux-gnu` |
| Linux | Windows 64-bit MinGW   | x86_64-pc-windows-gnu         | `cross +nightly build --release --target x86_64-pc-windows-gnu`     |
| Linux | 64-bit macOS           | x86_64-apple-darwin           | `cross +nightly build --release --target x86_64-apple-darwin`       |

#### Binutils

| target                        | Ubuntu binutils package      |
|-------------------------------|------------------------------|
| aarch64-unknown-linux-gnu     | binutils-aarch64-linux-gnu   |
| armv7-unknown-linux-gnueabihf | binutils-arm-linux-gnueabihf |
| x86_64-pc-windows-gnu         | binutils-mingw-w64-x86-64    |

```admonish tip title = "Ubuntu packages for binutils"
Run `apt-cache search binutils` for getting a list of possible binutils packages. 
```

```admonish tip title = "Cargo Configuration"
The binutils for specific targets are configured at .cargo/config.toml
```

#### Rust Platform Support

```admonish tip title = "Rust Platform Support"
Please check out the [platform support of the rust language](https://doc.rust-lang.org/nightly/rustc/platform-support.html).
```
