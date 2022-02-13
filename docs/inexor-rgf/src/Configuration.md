# 3. Configuration

## Configure Logging

1. Edit `config/logging.toml`
2. In the section `loggers` add or modify the logger for a specific module

```toml
[loggers."inexor_rgf_plugin_mqtt::behaviour::relation::mqtt_subscribes"]
level = "debug"
appenders = [ "file-plugin-mqtt" ]
additive = false
```

## Configure HTTP/GraphQL server

1. Edit `config/graphql.toml`
2. Configure the hostname and port

```toml
hostname = "localhost"
port = 31415
```
