# Development / Supported Platforms

**Reactive Graph** is completely platform-agnostic, which allows it to run on various operating systems.

### Microsoft Windows

We support x64 Microsoft Windows 8, 8.1 and 10.

We have build instructions for Windows.

### Linux

We support x86_64 / i686 / arm7 / aarch Linux. Chances are that it works with other targets, too, but haven't been tested.

We have specific build instructions for Ubuntu and Raspberry Pi 2-4.

```admonish tip title = "libc"
The continuous integration is configured to use `ubuntu-22.04` in order to support older libc versions.
> target: x86_64-unknown-linux-gnu
> os: ubuntu-22.04
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

#### Binutils and GCC

| target                        | Ubuntu binutils package        | Ubuntu GCC package        |
|-------------------------------|--------------------------------|---------------------------|
| aarch64-unknown-linux-gnu     | binutils-aarch64-linux-gnu     | gcc-aarch64-linux-gnu     |
| armv7-unknown-linux-gnueabihf | binutils-arm-none-gnueabihf    | gcc-arm-linux-gnueabihf   |
| i686-unknown-linux-gnu        | binutils-i686-linux-gnu        | gcc-i686-linux-gnu        |
| powerpc-unknown-linux-gnu     | binutils-powerpc-linux-gnu     | gcc-powerpc-linux-gnu     |
| powerpc64-unknown-linux-gnu   | binutils-powerpc64-linux-gnu   | gcc-powerpc64-linux-gnu   |
| powerpc64le-unknown-linux-gnu | binutils-powerpc64le-linux-gnu | gcc-powerpc64le-linux-gnu |
| riscv64gc-unknown-linux-gnu   | binutils-riscv64-linux-gnu     | gcc-riscv64-linux-gnu     |
| x86_64-pc-windows-gnu         | binutils-mingw-w64-x86-64      | gcc-mingw-w64-x86-64      |

```admonish tip title = "Ubuntu packages for binutils"
Run `apt-cache search binutils` for getting a list of possible binutils packages. 
```

```admonish tip title = "Cargo Configuration"
The binutils for specific targets are configured at .cargo/config.toml
```

#### Rust Platform Support

```admonish tip title = "Rust Platform Support"
Please check out the [platform support of the rust language](https://doc.rust-lang.org/nightly/rustc/platform-support.html).
[rustup components](https://rust-lang.github.io/rustup-components-history/x86_64-unknown-linux-gnu.html)
```

#### AWS Libcrypto Platform Support

```admonish tip title = "aws-lc-rs Platform Support"
Please check out the [platform support of aws-lc-rs](https://aws.github.io/aws-lc-rs/platform_support.html).
```

#### Setup Cross Toolchain GitHub Action

* https://github.com/taiki-e/setup-cross-toolchain-action

#### Debian: Supported Architectures

* https://wiki.debian.org/SupportedArchitectures
