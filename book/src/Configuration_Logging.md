# Configure Logging

Edit `config/logging.toml`

## Appender

### Stdout

Writes to stdout.

```toml
[appenders.stdout]
kind = "console"

[appenders.stdout.encoder]
pattern = "{d(%H:%M:%S%.3f)} [{l}] [{T}] {M}:{L} - {m}{n}"
```

> [!TIP]
> Documentation of the date format library
> [Date Formatting Syntax](https://docs.rs/chrono/0.4.11/chrono/format/strftime/index.html)

> [!TIP]
> On linux the systemd unit captures the stdout. Do not use any special characters or colors.

### File

Writes to file.

```toml
[appenders.file-application]
kind = "file"
path = "log/reactive-graph.log"

[appenders.file-application.encoder]
pattern = "{d(%Y-%m-%d %H:%M:%S%.3f)} [{l}] [{T}] {M}:{L} - {m}{n}"
```

## Default log level

Set the default logging level to "debug" and attach the `stdout` and `file-application` appender to the root.

```toml
[root]
level = "debug"
appenders = ["stdout", "file-application"]
```

## Per Log levels

Set the log level for a specific module:

```toml
[loggers."reactive_graph_type_system_impl::component_manager_impl"]
level = "info"
```

> [!TIP]
> Documentation of the logging library
> [How to configure the loggers](https://docs.rs/log4rs/1.0.0/log4rs/)

## Additive

Route log events sent to the `reactive_graph_plugin_service_impl::plugin_resolver_impl` logger
to the `plugin_resolver` appender, and *not* the normal appenders installed at the root.

```toml
[loggers."reactive_graph_plugin_service_impl::plugin_resolver_impl"]
level = "debug"
appenders = ["plugin_resolver"]
additive = false
```
