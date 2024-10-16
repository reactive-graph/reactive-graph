# Configure Remotes

Edit `config/remotes.toml`

## Hostname, port, protocol

The address of a remote is set using `hostname`, `port` and `secure`. If no port is given, the
default port `31415` is used.

`secure` defines if a secure connection shall be used (https or wss). By default, `secure` is false.

### Examples

1. https://demo.reactive-graph.io/graphql
    ```toml
    [[remotes]]
    hostname = "demo.reactive-graph.io"
    port = 443
    secure = true
    ```
2. http://localhost:31415/graphql
    ```toml
    [[remotes]]
    hostname = "localhost"
    ```

## Endpoint

Optionally, you can set the GraphQL endpoint. By default, the GraphQL endpoint is `/graphql`.

```toml
[[remotes]]
# hostname, port omitted
endpoint = "/graphql"
```

## User Agent

You can configure the user agent which shall be used for communication with the remote. By
default, the user agent is `reactive_graph`.

```toml
[[remotes]]
# hostname, port omitted
userAgent = "Noob Agent"
```

## Bearer

You can configure the bearer token which shall be used for communication with the remote. By default,
no bearer token is used.

```toml
[[remotes]]
# hostname, port omitted
bearer = "..."
```
