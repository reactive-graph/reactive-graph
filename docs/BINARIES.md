# Binary File Size

## Release Profiles

* https://doc.rust-lang.org/book/ch14-01-release-profiles.html

```shell
# Build binary with dev profile
cargo build
```

```shell
# Build binary with release profile
cargo build --release
```

## Optimization Levels

* https://doc.rust-lang.org/cargo/reference/profiles.html

## Strip Symbols

* https://doc.rust-lang.org/cargo/reference/unstable.html#profile-strip-option

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
