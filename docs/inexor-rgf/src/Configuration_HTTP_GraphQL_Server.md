# Configure HTTP/GraphQL server

1. Edit `config/graphql.toml`

## Bind address and port

Configure the bind address and port:

```toml
hostname = "localhost"
port = 31415
```

```admonish tip "Hostname"
If you bind on localhost the GraphQL server is not reachable from outside. Use the hostname or ip which is
reachable from outside.
```

## Secure connections (HTTPS/WSS)

Enable HTTPS/WSS:

```toml
secure = true
```

## Shutdown timeout

The following setting Timeout for graceful worker shutdown in seconds.

After receiving a stop signal, workers have this much time to finish serving requests. Workers
still alive after the timeout are force dropped. By default, shutdown timeout is set to 30 seconds.

```toml
shutdown_timeout = 3
```

```admonish tip "Development"
If you have to restart often during development, set this to a low value.
```

## Number of workers

This setting specifies the number of worker threads. The default number of worker threads is the
number of physical CPU cores available.

```toml
workers = 16
```

## Default web resource provider

Plugins can provide web resources using a [WebResourceProvider](./Plugin_System_Web_Resource_Provider.md).
The URLs are prefixed with the base path of the web resource provider. For example, the plugin
`binary` provides a `WebResourceProvider` with the base path `binary`. Therefore, the URL starts
with `http(s)://hostname:31415/binary/`.

The `default_base_path` defines which web resource provider is the default by its base path. This
means that the URL `https://hostname:31415/entities/uuid/property_name` will be handled by the
WebResourceProvider binary and is doing the same as
`https://hostname:31415/binary/entities/uuid/property_name`.

```toml
default_base_path = "binary"
```

```admonish tip "Default Base Path"
In particular, this is very useful for web applications which shall handle the root URL:
https://hostname:31415/
```

## Logging

You can enable or disable logging of HTTP/GraphQL requests and specify the log format. If no
format is specified the default log format is used.

```toml
[logging]
enabled = true
format = "%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"
```

```admonish tip "List of Log Format Variables"
https://docs.rs/actix-web/latest/actix_web/middleware/struct.Logger.html#format
```
