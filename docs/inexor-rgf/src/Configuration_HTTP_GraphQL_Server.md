# Configure HTTP/GraphQL server

1. Edit `config/graphql.toml`

## Bind address and port

Configure the bind address and port:

```toml
hostname = "localhost"
port = 31415
```

## Shutdown timeout

The following setting Timeout for graceful worker shutdown in seconds.

After receiving a stop signal, workers have this much time to finish serving requests. Workers
still alive after the timeout are force dropped. By default, shutdown timeout is set to 30 seconds.

```toml
shutdown_timeout = 3
```

## Number of workers

This setting specifies the number of worker threads. The default number of worker threads is the
number of physical CPU cores available.

```toml
workers = 16
```
