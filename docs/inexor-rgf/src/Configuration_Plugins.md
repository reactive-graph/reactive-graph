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

### Enable specific plugins

You can enable specific plugins. This take precedence over `disabled_plugins` (which will have no effect).

```toml
enabled_plugins = [
    "inexor-rgf-plugin-base",
    "inexor-rgf-plugin-trigger",
    "inexor-rgf-plugin-result",
    "inexor-rgf-plugin-file",
    "inexor-rgf-plugin-connector",
    "inexor-rgf-plugin-json",
    "inexor-rgf-plugin-taxonomy",
]
```
