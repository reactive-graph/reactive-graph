# Development / Binary File Size

## Release Profiles

```shell
# Build binary with dev profile
cargo build
```

```shell
# Build binary with release profile
cargo build --release
```

```admonish tip "Release Profiles"
For more information about release profiles, please refer to:
* [https://doc.rust-lang.org/book/ch14-01-release-profiles.html](https://doc.rust-lang.org/book/ch14-01-release-profiles.html)
```

```admonish tip "Optimization Levels"
For more information about rust compiler optimization levels, please have a look at
* [https://doc.rust-lang.org/cargo/reference/profiles.html#opt-level](https://doc.rust-lang.org/cargo/reference/profiles.html#opt-level)
```

```admonish tip "Strip Symbols"
For more information about strip symbols, please see
* [https://doc.rust-lang.org/cargo/reference/unstable.html#profile-strip-option](https://doc.rust-lang.org/cargo/reference/unstable.html#profile-strip-option)
```

## Application (exe)

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
# 12MB -> 7.8MB
lto = true
# 7.8MB -> 4.5MB
strip = "symbols"
```

## Plugin (.so/.dll)

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
# 12MB -> 7.8MB
lto = true
# 7.8MB -> 4.5MB
strip = "symbols"
```
