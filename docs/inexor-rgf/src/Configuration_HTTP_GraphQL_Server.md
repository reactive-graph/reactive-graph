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
