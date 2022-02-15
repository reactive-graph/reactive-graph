# Configure Plugins

Edit `config/plugins.toml`

You can activate or deactivate plugins with the setting `active`.

The path can be relative to the working directory or absolute.

The plugins are initialized in the order of definition!

## Linux

```toml
[[plugin]]
name = "inexor-rgf-plugin-base"
active = true
path = "../inexor-rgf-plugin-base/target/debug/libinexor_rgf_plugin_base.so"

[[plugin]]
name = "inexor-rgf-plugin-mqtt"
active = true
path = "../inexor-rgf-plugin-mqtt/target/debug/libinexor_rgf_plugin_mqtt.so"
```

### Please note

* The filename of the linked library is prefixed with `lib`
* On linux the file extension is `so`
* The path separators are forward slashes `/`

## Windows

```toml
[[plugin]]
name = "inexor-rgf-plugin-base"
active = true
path = "..\\inexor-rgf-plugin-base\\target\\debug\\inexor_rgf_plugin_base.dll"

[[plugin]]
name = "inexor-rgf-plugin-mqtt"
active = true
path = "..\\inexor-rgf-plugin-mqtt\\target/debug\\inexor_rgf_plugin_mqtt.dll"
```

### Please note

* The filename of the linked library is **not** prefixed with `lib`
* On Windows the file extension is `dll`
* The path separators are backslashes and must be escaped `\\`
