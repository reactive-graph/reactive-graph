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
    "inexor-rgf-plugin-arithmetic",
    "inexor-rgf-plugin-numeric"
]
```

```admonish bug "Plugin Dependencies"
Some plugins depends on other plugins.

Please note, that disabling a plugin may cause that other plugins cannot start!
```
