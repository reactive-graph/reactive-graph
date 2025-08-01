# Command-Line Help for `reactive-graph-server`

This document contains the help content for the `reactive-graph-server` command-line program.

**Command Overview:**

* [`reactive-graph-server`↴](#reactive-graph-server)
* [`reactive-graph-server shell-completions`↴](#reactive-graph-server-shell-completions)
* [`reactive-graph-server shell-completions print`↴](#reactive-graph-server-shell-completions-print)
* [`reactive-graph-server shell-completions install`↴](#reactive-graph-server-shell-completions-install)
* [`reactive-graph-server man-pages`↴](#reactive-graph-server-man-pages)
* [`reactive-graph-server man-pages print`↴](#reactive-graph-server-man-pages-print)
* [`reactive-graph-server man-pages install`↴](#reactive-graph-server-man-pages-install)
* [`reactive-graph-server print-markdown-help`↴](#reactive-graph-server-print-markdown-help)
* [`reactive-graph-server info`↴](#reactive-graph-server-info)
* [`reactive-graph-server daemon`↴](#reactive-graph-server-daemon)
* [`reactive-graph-server graphql-schema`↴](#reactive-graph-server-graphql-schema)
* [`reactive-graph-server graphql-schema reactive-graph-schema`↴](#reactive-graph-server-graphql-schema-reactive-graph-schema)
* [`reactive-graph-server graphql-schema dynamic-graph-schema`↴](#reactive-graph-server-graphql-schema-dynamic-graph-schema)
* [`reactive-graph-server graphql-schema reactive-graph-plugin-schema`↴](#reactive-graph-server-graphql-schema-reactive-graph-plugin-schema)
* [`reactive-graph-server graphql-schema reactive-graph-runtime-schema`↴](#reactive-graph-server-graphql-schema-reactive-graph-runtime-schema)
* [`reactive-graph-server json-schema`↴](#reactive-graph-server-json-schema)
* [`reactive-graph-server json-schema types`↴](#reactive-graph-server-json-schema-types)
* [`reactive-graph-server json-schema types components`↴](#reactive-graph-server-json-schema-types-components)
* [`reactive-graph-server json-schema types entities`↴](#reactive-graph-server-json-schema-types-entities)
* [`reactive-graph-server json-schema types relations`↴](#reactive-graph-server-json-schema-types-relations)
* [`reactive-graph-server json-schema types flows`↴](#reactive-graph-server-json-schema-types-flows)
* [`reactive-graph-server json-schema instances`↴](#reactive-graph-server-json-schema-instances)
* [`reactive-graph-server json-schema instances entities`↴](#reactive-graph-server-json-schema-instances-entities)
* [`reactive-graph-server json-schema instances relations`↴](#reactive-graph-server-json-schema-instances-relations)
* [`reactive-graph-server json-schema instances flows`↴](#reactive-graph-server-json-schema-instances-flows)

## `reactive-graph-server`

Reactive Graph is a reactive runtime based on a graph database, empowering everyone to build reliable and efficient software.

**Usage:** `reactive-graph-server [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `shell-completions` — Prints or installs Shell completions
* `man-pages` — Prints or installs man pages
* `print-markdown-help` — Prints the markdown help to stdout
* `info` — Prints info about this binary
* `daemon` — Runs the server as daemon
* `graphql-schema` — Prints the GraphQL schema and exits
* `json-schema` — Prints the JSON schema and exits

###### **Options:**

* `--logging-config <LOGGING_CONFIG>` — The logging config location
* `--instance-config <INSTANCE_CONFIG>` — The instance config location
* `--graphql-config <GRAPHQL_CONFIG>` — The GraphQL config location
* `--plugins-config <PLUGINS_CONFIG>` — The plugins config location
* `-n`, `--instance-name <NAME>` — The name of the instance
* `-d`, `--instance-description <DESCRIPTION>` — The description of the instance
* `--hostname <HOSTNAME>` — The hostname to bind the GraphQL HTTP server
* `--port <PORT>` — The port to bind the GraphQL HTTP server
* `--secure <SECURE>` — If true, HTTPS is enabled

  Possible values: `true`, `false`

* `--ssl-certificate-path <SSL_CERTIFICATE_PATH>` — The location of the certificate
* `--ssl-private-key-path <SSL_PRIVATE_KEY_PATH>` — The location of the private key
* `--shutdown-timeout <SHUTDOWN_TIMEOUT>` — Timeout for graceful workers shutdown in seconds. After receiving a stop signal, workers have this much time to finish serving requests. Workers still alive after the timeout are force dropped. By default, shutdown timeout sets to 30 seconds
* `-w`, `--workers <WORKERS>` — The number of workers to start. The default worker count is the number of physical CPU cores available
* `-c`, `--default-context-path <DEFAULT_CONTEXT_PATH>` — The default context path which redirects the root context to a web resource provider
* `-x`, `--disable-all-plugins <DISABLE_ALL_PLUGINS>` — If true, all plugins will be disabled

  Possible values: `true`, `false`

* `-p`, `--disabled-plugins <DISABLED_PLUGINS>` — The list of plugins to disable
* `-P`, `--enabled-plugins <ENABLED_PLUGINS>` — The list of plugins to enable
* `--disable-hot-deploy <DISABLE_HOT_DEPLOY>` — If true, hot deployment will be disabled

  Possible values: `true`, `false`

* `--hot-deploy-location <HOT_DEPLOY_LOCATION>` — The folder which is watched for hot deployment
* `--install-location <INSTALL_LOCATION>` — The folder which plugins are installed permanently
* `--stop-immediately <STOP_IMMEDIATELY>` — If true, the runtime does not wait before exiting

  Possible values: `true`, `false`

* `-q`, `--quiet <QUIET>` — If true, logging is disabled completely

  Possible values: `true`, `false`




## `reactive-graph-server shell-completions`

Prints or installs Shell completions

**Usage:** `reactive-graph-server shell-completions <COMMAND>`

###### **Subcommands:**

* `print` — Prints the shell completions to stdout
* `install` — Installs the shell completions



## `reactive-graph-server shell-completions print`

Prints the shell completions to stdout

**Usage:** `reactive-graph-server shell-completions print <SHELL>`

###### **Arguments:**

* `<SHELL>` — The shell

  Possible values: `bash`, `elvish`, `fish`, `powershell`, `zsh`




## `reactive-graph-server shell-completions install`

Installs the shell completions

**Usage:** `reactive-graph-server shell-completions install <SHELL>`

###### **Arguments:**

* `<SHELL>` — The shell

  Possible values: `bash`, `elvish`, `fish`, `powershell`, `zsh`




## `reactive-graph-server man-pages`

Prints or installs man pages

**Usage:** `reactive-graph-server man-pages <COMMAND>`

###### **Subcommands:**

* `print` — Prints the man pages to stdout
* `install` — Installs the man pages



## `reactive-graph-server man-pages print`

Prints the man pages to stdout

**Usage:** `reactive-graph-server man-pages print`



## `reactive-graph-server man-pages install`

Installs the man pages

**Usage:** `reactive-graph-server man-pages install`



## `reactive-graph-server print-markdown-help`

Prints the markdown help to stdout

**Usage:** `reactive-graph-server print-markdown-help`



## `reactive-graph-server info`

Prints info about this binary

**Usage:** `reactive-graph-server info [OPTIONS]`

###### **Options:**

* `--output-format <OUTPUT_FORMAT>` — The output format

  Possible values: `table`, `html-table`, `markdown-table`, `count`, `json`, `json5`, `toml`




## `reactive-graph-server daemon`

Runs the server as daemon

**Usage:** `reactive-graph-server daemon [OPTIONS]`

###### **Options:**

* `--daemon-name <DAEMON_NAME>` — Sets the name of the daemon
* `--daemon-pid <DAEMON_PID>` — The location of the daemon PID file. By default, no PID file will be created
* `--daemon-working-directory <DAEMON_WORKING_DIRECTORY>` — The working directory of the daemon
* `--daemon-stdout <DAEMON_STDOUT>` — Stdout will be written into this file
* `--daemon-stderr <DAEMON_STDERR>` — Stderr will be written into this file
* `--daemon-user <DAEMON_USER>` — If set will drop privileges to the specified user. Note: Both must be given: user and group
* `--daemon-group <DAEMON_GROUP>` — If set will drop privileges to the specified group. Note: Both must be given: user and group



## `reactive-graph-server graphql-schema`

Prints the GraphQL schema and exits

**Usage:** `reactive-graph-server graphql-schema <COMMAND>`

###### **Subcommands:**

* `reactive-graph-schema` — Prints the GraphQL schema of the reactive graph
* `dynamic-graph-schema` — Prints the GraphQL schema of the dynamic graph
* `reactive-graph-plugin-schema` — Prints the GraphQL schema of the plugin system of the reactive graph
* `reactive-graph-runtime-schema` — Prints the GraphQL schema of the runtime of the reactive graph



## `reactive-graph-server graphql-schema reactive-graph-schema`

Prints the GraphQL schema of the reactive graph

**Usage:** `reactive-graph-server graphql-schema reactive-graph-schema`



## `reactive-graph-server graphql-schema dynamic-graph-schema`

Prints the GraphQL schema of the dynamic graph

**Usage:** `reactive-graph-server graphql-schema dynamic-graph-schema`



## `reactive-graph-server graphql-schema reactive-graph-plugin-schema`

Prints the GraphQL schema of the plugin system of the reactive graph

**Usage:** `reactive-graph-server graphql-schema reactive-graph-plugin-schema`



## `reactive-graph-server graphql-schema reactive-graph-runtime-schema`

Prints the GraphQL schema of the runtime of the reactive graph

**Usage:** `reactive-graph-server graphql-schema reactive-graph-runtime-schema`



## `reactive-graph-server json-schema`

Prints the JSON schema and exits

**Usage:** `reactive-graph-server json-schema <COMMAND>`

###### **Subcommands:**

* `types` — Prints the JSON schema of the type system
* `instances` — Prints the JSON schema of the instance system



## `reactive-graph-server json-schema types`

Prints the JSON schema of the type system

**Usage:** `reactive-graph-server json-schema types <COMMAND>`

###### **Subcommands:**

* `components` — Prints the JSON schema of the component types
* `entities` — Prints the JSON schema of the entity types
* `relations` — Prints the JSON schema of the relation types
* `flows` — Prints the JSON schema of the flow types



## `reactive-graph-server json-schema types components`

Prints the JSON schema of the component types

**Usage:** `reactive-graph-server json-schema types components`



## `reactive-graph-server json-schema types entities`

Prints the JSON schema of the entity types

**Usage:** `reactive-graph-server json-schema types entities`



## `reactive-graph-server json-schema types relations`

Prints the JSON schema of the relation types

**Usage:** `reactive-graph-server json-schema types relations`



## `reactive-graph-server json-schema types flows`

Prints the JSON schema of the flow types

**Usage:** `reactive-graph-server json-schema types flows`



## `reactive-graph-server json-schema instances`

Prints the JSON schema of the instance system

**Usage:** `reactive-graph-server json-schema instances <COMMAND>`

###### **Subcommands:**

* `entities` — Prints the JSON schema of the entity instances
* `relations` — Prints the JSON schema of the relation instances
* `flows` — Prints the JSON schema of the flow instances



## `reactive-graph-server json-schema instances entities`

Prints the JSON schema of the entity instances

**Usage:** `reactive-graph-server json-schema instances entities`



## `reactive-graph-server json-schema instances relations`

Prints the JSON schema of the relation instances

**Usage:** `reactive-graph-server json-schema instances relations`



## `reactive-graph-server json-schema instances flows`

Prints the JSON schema of the flow instances

**Usage:** `reactive-graph-server json-schema instances flows`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

