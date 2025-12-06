# Configure Plugins

Edit `config/plugins.toml`

### Disable the plugin system

You can disable / enable the plugin system entirely:

```toml
disabled = false
```

### Disable specific plugins

You can disable specific plugins

```toml
disabled_plugins = [
    "reactive-graph-plugin-arithmetic",
    "reactive-graph-plugin-numeric"
]
```

> [!IMPORTANT]
> Plugin Dependencies
> Some plugins depends on other plugins.
> Please note, that disabling a plugin may cause that other plugins cannot start!

### Enable specific plugins

You can enable specific plugins. This take precedence over `disabled_plugins` (which will have no effect).

```toml
enabled_plugins = [
    "reactive-graph-plugin-base",
    "reactive-graph-plugin-trigger",
    "reactive-graph-plugin-result",
    "reactive-graph-plugin-file",
    "reactive-graph-plugin-connector",
    "reactive-graph-plugin-json",
    "reactive-graph-plugin-taxonomy",
]
```
