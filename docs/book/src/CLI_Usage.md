# Command Line Interface - General Usage

## How to use the command line interface

```shell
$ reactive-graph client [OPTIONS] [COMMAND]
```

### Example

To get the instance information of the local server:

```shell
$ reactive-graph client instance-info get
```

To get the instance information of a remote server:

```shell
$ reactive-graph client --hostname=localhost --port=31415 instance-info get
```

## Commands

| Command          | Description                                               |
|------------------|-----------------------------------------------------------|
| execute-command  | Executes a command on the client                          |
| instance-info    | Prints information about the instance                     |
| plugins          | Manage plugins                                            |
| remotes          | Manage remotes                                            |
| shutdown         | Shutdown the runtime                                      |
| components       | Manage components                                         |
| entity-types     | Manage entity types                                       |
| relation-types   | Manage relation types                                     |
| entity-instances | Manage entity instances                                   |
| help             | Print this message or the help of the given subcommand(s) |

## Options

| Option                                            | Description                                                  |
|---------------------------------------------------|--------------------------------------------------------------|
| --hostname <HOSTNAME>                             | The hostname to connect to                                   |
| --port <PORT>                                     | The port to connect to                                       |
| --secure <SECURE>                                 | If true, connects via HTTPS [possible values: true, false]   |
| --endpoint-graphql <ENDPOINT_GRAPHQL>             | The graphql endpoint for the type system and instance system |
| --endpoint-dynamic-graph <ENDPOINT_DYNAMIC_GRAPH> | The graphql endpoint for the dynamic graph                   |
| --endpoint-runtime <ENDPOINT_RUNTIME>             | The graphql endpoint for the runtime                         |
| --endpoint-plugins <ENDPOINT_PLUGINS>             | The graphql endpoint for plugins                             |
| --bearer <BEARER>                                 | The authentication token                                     |
| -h, --help                                        | Print help                                                   |
| -V, --version                                     | Print version                                                |
