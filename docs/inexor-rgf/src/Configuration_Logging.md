# Configure Logging

Edit `config/logging.toml`

## Appender

### Stdout

Writes to stdout.

```toml
[appenders.stdout]
kind = "console"

[appenders.stdout.encoder]
pattern = "{d(%H:%M:%S%.3f)} [{l}] [{T}] {M}:{L} - {m}{n}\r"
```

```admonish tip "Documentation of the date format library"
[Date Formatting Syntax](https://docs.rs/chrono/0.4.11/chrono/format/strftime/index.html)
```

### File

Writes to file.

```toml
[appenders.file-application]
kind = "file"
path = "log/inexor-rgf-application.log"

[appenders.file-application.encoder]
pattern = "{d(%Y-%m-%d %H:%M:%S%.3f)} [{l}] [{T}] {M}:{L} - {m}{n}"
```

## Default log level

Set the default logging level to "debug" and attach the `stdout` and `file-application` appender to the root.

```toml
[root]
level = "debug"
appenders = [ "stdout", "file-application" ]
```

## Per Log levels

Set the log level for a specific module:

```toml
[loggers."inexor_rgf_application::implementation::component_manager_impl"]
level = "info"
```

```admonish tip "Documentation of the logging library"
[How to configure the loggers](https://docs.rs/log4rs/1.0.0/log4rs/)
```

## Additive

Route log events sent to the `inexor_rgf_plugin_mqtt::behaviour::relation::mqtt_subscribes` logger
to the `file-plugin-mqtt` appender, and *not* the normal appenders installed at the root.

```toml
[loggers."inexor_rgf_plugin_mqtt::behaviour::relation::mqtt_subscribes"]
level = "debug"
appenders = [ "file-plugin-mqtt" ]
additive = false
```
