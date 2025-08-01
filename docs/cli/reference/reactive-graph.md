# Command-Line Help for `reactive-graph`

This document contains the help content for the `reactive-graph` command-line program.

**Command Overview:**

* [`reactive-graph`↴](#reactive-graph)
* [`reactive-graph shell-completions`↴](#reactive-graph-shell-completions)
* [`reactive-graph shell-completions print`↴](#reactive-graph-shell-completions-print)
* [`reactive-graph shell-completions install`↴](#reactive-graph-shell-completions-install)
* [`reactive-graph man-pages`↴](#reactive-graph-man-pages)
* [`reactive-graph man-pages print`↴](#reactive-graph-man-pages-print)
* [`reactive-graph man-pages install`↴](#reactive-graph-man-pages-install)
* [`reactive-graph print-markdown-help`↴](#reactive-graph-print-markdown-help)
* [`reactive-graph info`↴](#reactive-graph-info)
* [`reactive-graph daemon`↴](#reactive-graph-daemon)
* [`reactive-graph graphql-schema`↴](#reactive-graph-graphql-schema)
* [`reactive-graph graphql-schema reactive-graph-schema`↴](#reactive-graph-graphql-schema-reactive-graph-schema)
* [`reactive-graph graphql-schema dynamic-graph-schema`↴](#reactive-graph-graphql-schema-dynamic-graph-schema)
* [`reactive-graph graphql-schema reactive-graph-plugin-schema`↴](#reactive-graph-graphql-schema-reactive-graph-plugin-schema)
* [`reactive-graph graphql-schema reactive-graph-runtime-schema`↴](#reactive-graph-graphql-schema-reactive-graph-runtime-schema)
* [`reactive-graph json-schema`↴](#reactive-graph-json-schema)
* [`reactive-graph json-schema types`↴](#reactive-graph-json-schema-types)
* [`reactive-graph json-schema types components`↴](#reactive-graph-json-schema-types-components)
* [`reactive-graph json-schema types entities`↴](#reactive-graph-json-schema-types-entities)
* [`reactive-graph json-schema types relations`↴](#reactive-graph-json-schema-types-relations)
* [`reactive-graph json-schema types flows`↴](#reactive-graph-json-schema-types-flows)
* [`reactive-graph json-schema instances`↴](#reactive-graph-json-schema-instances)
* [`reactive-graph json-schema instances entities`↴](#reactive-graph-json-schema-instances-entities)
* [`reactive-graph json-schema instances relations`↴](#reactive-graph-json-schema-instances-relations)
* [`reactive-graph json-schema instances flows`↴](#reactive-graph-json-schema-instances-flows)
* [`reactive-graph execute-command`↴](#reactive-graph-execute-command)
* [`reactive-graph instance-info`↴](#reactive-graph-instance-info)
* [`reactive-graph instance-info get`↴](#reactive-graph-instance-info-get)
* [`reactive-graph plugins`↴](#reactive-graph-plugins)
* [`reactive-graph plugins list`↴](#reactive-graph-plugins-list)
* [`reactive-graph plugins search`↴](#reactive-graph-plugins-search)
* [`reactive-graph plugins get`↴](#reactive-graph-plugins-get)
* [`reactive-graph plugins dependencies`↴](#reactive-graph-plugins-dependencies)
* [`reactive-graph plugins dependents`↴](#reactive-graph-plugins-dependents)
* [`reactive-graph plugins start`↴](#reactive-graph-plugins-start)
* [`reactive-graph plugins stop`↴](#reactive-graph-plugins-stop)
* [`reactive-graph plugins restart`↴](#reactive-graph-plugins-restart)
* [`reactive-graph plugins uninstall`↴](#reactive-graph-plugins-uninstall)
* [`reactive-graph remotes`↴](#reactive-graph-remotes)
* [`reactive-graph remotes list`↴](#reactive-graph-remotes-list)
* [`reactive-graph remotes add`↴](#reactive-graph-remotes-add)
* [`reactive-graph remotes remove`↴](#reactive-graph-remotes-remove)
* [`reactive-graph remotes remove-all`↴](#reactive-graph-remotes-remove-all)
* [`reactive-graph remotes update`↴](#reactive-graph-remotes-update)
* [`reactive-graph remotes update-all`↴](#reactive-graph-remotes-update-all)
* [`reactive-graph remotes fetch-remotes-from-remote`↴](#reactive-graph-remotes-fetch-remotes-from-remote)
* [`reactive-graph remotes fetch-remotes-from-all-remotes`↴](#reactive-graph-remotes-fetch-remotes-from-all-remotes)
* [`reactive-graph shutdown`↴](#reactive-graph-shutdown)
* [`reactive-graph components`↴](#reactive-graph-components)
* [`reactive-graph components list`↴](#reactive-graph-components-list)
* [`reactive-graph components get`↴](#reactive-graph-components-get)
* [`reactive-graph components list-properties`↴](#reactive-graph-components-list-properties)
* [`reactive-graph components list-extensions`↴](#reactive-graph-components-list-extensions)
* [`reactive-graph components get-json-schema`↴](#reactive-graph-components-get-json-schema)
* [`reactive-graph components create`↴](#reactive-graph-components-create)
* [`reactive-graph components delete`↴](#reactive-graph-components-delete)
* [`reactive-graph components add-property`↴](#reactive-graph-components-add-property)
* [`reactive-graph components remove-property`↴](#reactive-graph-components-remove-property)
* [`reactive-graph components add-extension`↴](#reactive-graph-components-add-extension)
* [`reactive-graph components remove-extension`↴](#reactive-graph-components-remove-extension)
* [`reactive-graph components update-description`↴](#reactive-graph-components-update-description)
* [`reactive-graph components json-schema`↴](#reactive-graph-components-json-schema)
* [`reactive-graph entity-types`↴](#reactive-graph-entity-types)
* [`reactive-graph entity-types list`↴](#reactive-graph-entity-types-list)
* [`reactive-graph entity-types get`↴](#reactive-graph-entity-types-get)
* [`reactive-graph entity-types list-properties`↴](#reactive-graph-entity-types-list-properties)
* [`reactive-graph entity-types list-extensions`↴](#reactive-graph-entity-types-list-extensions)
* [`reactive-graph entity-types list-components`↴](#reactive-graph-entity-types-list-components)
* [`reactive-graph entity-types get-json-schema`↴](#reactive-graph-entity-types-get-json-schema)
* [`reactive-graph entity-types create`↴](#reactive-graph-entity-types-create)
* [`reactive-graph entity-types delete`↴](#reactive-graph-entity-types-delete)
* [`reactive-graph entity-types add-property`↴](#reactive-graph-entity-types-add-property)
* [`reactive-graph entity-types remove-property`↴](#reactive-graph-entity-types-remove-property)
* [`reactive-graph entity-types add-extension`↴](#reactive-graph-entity-types-add-extension)
* [`reactive-graph entity-types remove-extension`↴](#reactive-graph-entity-types-remove-extension)
* [`reactive-graph entity-types add-component`↴](#reactive-graph-entity-types-add-component)
* [`reactive-graph entity-types remove-component`↴](#reactive-graph-entity-types-remove-component)
* [`reactive-graph entity-types update-description`↴](#reactive-graph-entity-types-update-description)
* [`reactive-graph entity-types json-schema`↴](#reactive-graph-entity-types-json-schema)
* [`reactive-graph relation-types`↴](#reactive-graph-relation-types)
* [`reactive-graph relation-types list`↴](#reactive-graph-relation-types-list)
* [`reactive-graph relation-types get`↴](#reactive-graph-relation-types-get)
* [`reactive-graph relation-types list-properties`↴](#reactive-graph-relation-types-list-properties)
* [`reactive-graph relation-types list-extensions`↴](#reactive-graph-relation-types-list-extensions)
* [`reactive-graph relation-types list-components`↴](#reactive-graph-relation-types-list-components)
* [`reactive-graph relation-types get-json-schema`↴](#reactive-graph-relation-types-get-json-schema)
* [`reactive-graph relation-types create`↴](#reactive-graph-relation-types-create)
* [`reactive-graph relation-types delete`↴](#reactive-graph-relation-types-delete)
* [`reactive-graph relation-types add-property`↴](#reactive-graph-relation-types-add-property)
* [`reactive-graph relation-types remove-property`↴](#reactive-graph-relation-types-remove-property)
* [`reactive-graph relation-types add-extension`↴](#reactive-graph-relation-types-add-extension)
* [`reactive-graph relation-types remove-extension`↴](#reactive-graph-relation-types-remove-extension)
* [`reactive-graph relation-types add-component`↴](#reactive-graph-relation-types-add-component)
* [`reactive-graph relation-types remove-component`↴](#reactive-graph-relation-types-remove-component)
* [`reactive-graph relation-types update-description`↴](#reactive-graph-relation-types-update-description)
* [`reactive-graph relation-types json-schema`↴](#reactive-graph-relation-types-json-schema)
* [`reactive-graph flow-types`↴](#reactive-graph-flow-types)
* [`reactive-graph flow-types list`↴](#reactive-graph-flow-types-list)
* [`reactive-graph flow-types get`↴](#reactive-graph-flow-types-get)
* [`reactive-graph flow-types list-variables`↴](#reactive-graph-flow-types-list-variables)
* [`reactive-graph flow-types list-extensions`↴](#reactive-graph-flow-types-list-extensions)
* [`reactive-graph flow-types get-json-schema`↴](#reactive-graph-flow-types-get-json-schema)
* [`reactive-graph flow-types create`↴](#reactive-graph-flow-types-create)
* [`reactive-graph flow-types delete`↴](#reactive-graph-flow-types-delete)
* [`reactive-graph flow-types add-variable`↴](#reactive-graph-flow-types-add-variable)
* [`reactive-graph flow-types remove-variable`↴](#reactive-graph-flow-types-remove-variable)
* [`reactive-graph flow-types add-extension`↴](#reactive-graph-flow-types-add-extension)
* [`reactive-graph flow-types remove-extension`↴](#reactive-graph-flow-types-remove-extension)
* [`reactive-graph flow-types update-description`↴](#reactive-graph-flow-types-update-description)
* [`reactive-graph flow-types add-entity-instance`↴](#reactive-graph-flow-types-add-entity-instance)
* [`reactive-graph flow-types remove-entity-instance`↴](#reactive-graph-flow-types-remove-entity-instance)
* [`reactive-graph flow-types json-schema`↴](#reactive-graph-flow-types-json-schema)
* [`reactive-graph entity-instances`↴](#reactive-graph-entity-instances)
* [`reactive-graph entity-instances list`↴](#reactive-graph-entity-instances-list)
* [`reactive-graph entity-instances get`↴](#reactive-graph-entity-instances-get)
* [`reactive-graph entity-instances get-by-label`↴](#reactive-graph-entity-instances-get-by-label)
* [`reactive-graph entity-instances list-properties`↴](#reactive-graph-entity-instances-list-properties)
* [`reactive-graph entity-instances get-property`↴](#reactive-graph-entity-instances-get-property)
* [`reactive-graph entity-instances set-property`↴](#reactive-graph-entity-instances-set-property)
* [`reactive-graph entity-instances add-property`↴](#reactive-graph-entity-instances-add-property)
* [`reactive-graph entity-instances remove-property`↴](#reactive-graph-entity-instances-remove-property)
* [`reactive-graph entity-instances list-components`↴](#reactive-graph-entity-instances-list-components)
* [`reactive-graph entity-instances add-component`↴](#reactive-graph-entity-instances-add-component)
* [`reactive-graph entity-instances remove-component`↴](#reactive-graph-entity-instances-remove-component)
* [`reactive-graph entity-instances create`↴](#reactive-graph-entity-instances-create)
* [`reactive-graph entity-instances delete`↴](#reactive-graph-entity-instances-delete)
* [`reactive-graph entity-instances json-schema`↴](#reactive-graph-entity-instances-json-schema)
* [`reactive-graph relation-instances`↴](#reactive-graph-relation-instances)
* [`reactive-graph relation-instances list`↴](#reactive-graph-relation-instances-list)
* [`reactive-graph relation-instances get`↴](#reactive-graph-relation-instances-get)
* [`reactive-graph relation-instances list-properties`↴](#reactive-graph-relation-instances-list-properties)
* [`reactive-graph relation-instances get-property`↴](#reactive-graph-relation-instances-get-property)
* [`reactive-graph relation-instances set-property`↴](#reactive-graph-relation-instances-set-property)
* [`reactive-graph relation-instances add-property`↴](#reactive-graph-relation-instances-add-property)
* [`reactive-graph relation-instances remove-property`↴](#reactive-graph-relation-instances-remove-property)
* [`reactive-graph relation-instances list-components`↴](#reactive-graph-relation-instances-list-components)
* [`reactive-graph relation-instances add-component`↴](#reactive-graph-relation-instances-add-component)
* [`reactive-graph relation-instances remove-component`↴](#reactive-graph-relation-instances-remove-component)
* [`reactive-graph relation-instances create`↴](#reactive-graph-relation-instances-create)
* [`reactive-graph relation-instances delete`↴](#reactive-graph-relation-instances-delete)
* [`reactive-graph relation-instances json-schema`↴](#reactive-graph-relation-instances-json-schema)
* [`reactive-graph flow-instances`↴](#reactive-graph-flow-instances)
* [`reactive-graph flow-instances list`↴](#reactive-graph-flow-instances-list)
* [`reactive-graph flow-instances get`↴](#reactive-graph-flow-instances-get)
* [`reactive-graph flow-instances get-by-label`↴](#reactive-graph-flow-instances-get-by-label)
* [`reactive-graph flow-instances create-from-type`↴](#reactive-graph-flow-instances-create-from-type)
* [`reactive-graph flow-instances delete`↴](#reactive-graph-flow-instances-delete)
* [`reactive-graph flow-instances json-schema`↴](#reactive-graph-flow-instances-json-schema)
* [`reactive-graph introspection`↴](#reactive-graph-introspection)
* [`reactive-graph introspection reactive-graph`↴](#reactive-graph-introspection-reactive-graph)
* [`reactive-graph introspection dynamic-graph`↴](#reactive-graph-introspection-dynamic-graph)
* [`reactive-graph introspection reactive-graph-runtime`↴](#reactive-graph-introspection-reactive-graph-runtime)
* [`reactive-graph introspection reactive-graph-plugins`↴](#reactive-graph-introspection-reactive-graph-plugins)
* [`reactive-graph instances`↴](#reactive-graph-instances)
* [`reactive-graph instances config`↴](#reactive-graph-instances-config)
* [`reactive-graph instances config graphql`↴](#reactive-graph-instances-config-graphql)
* [`reactive-graph instances config instance`↴](#reactive-graph-instances-config-instance)
* [`reactive-graph instances config plugins`↴](#reactive-graph-instances-config-plugins)
* [`reactive-graph instances generate-certificate`↴](#reactive-graph-instances-generate-certificate)
* [`reactive-graph instances init`↴](#reactive-graph-instances-init)
* [`reactive-graph instances plugins`↴](#reactive-graph-instances-plugins)
* [`reactive-graph instances plugins install`↴](#reactive-graph-instances-plugins-install)
* [`reactive-graph instances plugins uninstall`↴](#reactive-graph-instances-plugins-uninstall)
* [`reactive-graph instances repository`↴](#reactive-graph-instances-repository)
* [`reactive-graph instances repository init`↴](#reactive-graph-instances-repository-init)
* [`reactive-graph instances repository remove`↴](#reactive-graph-instances-repository-remove)
* [`reactive-graph update`↴](#reactive-graph-update)
* [`reactive-graph update info`↴](#reactive-graph-update-info)
* [`reactive-graph update list`↴](#reactive-graph-update-list)

## `reactive-graph`

Reactive Graph is a reactive runtime based on a graph database, empowering everyone to build reliable and efficient software.

**Usage:** `reactive-graph [OPTIONS] [DANGER_ACCEPT_INVALID_HOSTNAMES] [COMMAND]`

###### **Subcommands:**

* `shell-completions` — Prints or installs Shell completions
* `man-pages` — Prints or installs man pages
* `print-markdown-help` — Prints the markdown help to stdout
* `info` — Prints info about this binary
* `daemon` — Runs the server as daemon
* `graphql-schema` — Prints the GraphQL schema and exits
* `json-schema` — Prints the JSON schema and exits
* `execute-command` — Executes a command on the client
* `instance-info` — Prints information about the instance
* `plugins` — Manage plugins
* `remotes` — Manage remotes
* `shutdown` — Shutdown the runtime
* `components` — Manage components
* `entity-types` — Manage entity types
* `relation-types` — Manage entity types
* `flow-types` — Manage entity types
* `entity-instances` — Manage entity instances
* `relation-instances` — Manage relation instances
* `flow-instances` — Manage flow instances
* `introspection` — Execute GraphQL introspection queries
* `instances` — Manage instances
* `update` — Update the Reactive Graph binary

###### **Arguments:**

* `<DANGER_ACCEPT_INVALID_HOSTNAMES>` — Controls the use of hostname verification.

   Defaults to `false`.

   Warning: You should think very carefully before you use this method. If hostname verification is not used, any valid certificate for any site will be trusted for use from any other. This introduces a significant vulnerability to man-in-the-middle attacks.

  Possible values: `true`, `false`


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

* `--client-hostname <CLIENT_HOSTNAME>` — The hostname to connect to
* `--client-port <CLIENT_PORT>` — The port to connect to
* `--client-secure <CLIENT_SECURE>` — If true, connects via HTTPS

  Possible values: `true`, `false`

* `--endpoint-graphql <ENDPOINT_GRAPHQL>` — The endpoint to use
* `--endpoint-dynamic-graph <ENDPOINT_DYNAMIC_GRAPH>` — The endpoint to use
* `--endpoint-runtime <ENDPOINT_RUNTIME>` — The endpoint to use
* `--endpoint-plugins <ENDPOINT_PLUGINS>` — The endpoint to use
* `--bearer <BEARER>` — The authentication token
* `--danger-accept-invalid-certs <DANGER_ACCEPT_INVALID_CERTS>` — Controls the use of certificate validation.

   Defaults to `false`.

   Warning: You should think very carefully before using this method. If invalid certificates are trusted, *any* certificate for *any* site will be trusted for use. This includes expired certificates. This introduces significant vulnerabilities, and should only be used as a last resort.

  Possible values: `true`, `false`

* `-i`, `--interactive` — Enter the interactive client mode



## `reactive-graph shell-completions`

Prints or installs Shell completions

**Usage:** `reactive-graph shell-completions <COMMAND>`

###### **Subcommands:**

* `print` — Prints the shell completions to stdout
* `install` — Installs the shell completions



## `reactive-graph shell-completions print`

Prints the shell completions to stdout

**Usage:** `reactive-graph shell-completions print <SHELL>`

###### **Arguments:**

* `<SHELL>` — The shell

  Possible values: `bash`, `elvish`, `fish`, `powershell`, `zsh`




## `reactive-graph shell-completions install`

Installs the shell completions

**Usage:** `reactive-graph shell-completions install <SHELL>`

###### **Arguments:**

* `<SHELL>` — The shell

  Possible values: `bash`, `elvish`, `fish`, `powershell`, `zsh`




## `reactive-graph man-pages`

Prints or installs man pages

**Usage:** `reactive-graph man-pages <COMMAND>`

###### **Subcommands:**

* `print` — Prints the man pages to stdout
* `install` — Installs the man pages



## `reactive-graph man-pages print`

Prints the man pages to stdout

**Usage:** `reactive-graph man-pages print`



## `reactive-graph man-pages install`

Installs the man pages

**Usage:** `reactive-graph man-pages install`



## `reactive-graph print-markdown-help`

Prints the markdown help to stdout

**Usage:** `reactive-graph print-markdown-help`



## `reactive-graph info`

Prints info about this binary

**Usage:** `reactive-graph info [OPTIONS]`

###### **Options:**

* `--output-format <OUTPUT_FORMAT>` — The output format

  Possible values: `table`, `html-table`, `markdown-table`, `count`, `json`, `json5`, `toml`




## `reactive-graph daemon`

Runs the server as daemon

**Usage:** `reactive-graph daemon [OPTIONS]`

###### **Options:**

* `--daemon-name <DAEMON_NAME>` — Sets the name of the daemon
* `--daemon-pid <DAEMON_PID>` — The location of the daemon PID file. By default, no PID file will be created
* `--daemon-working-directory <DAEMON_WORKING_DIRECTORY>` — The working directory of the daemon
* `--daemon-stdout <DAEMON_STDOUT>` — Stdout will be written into this file
* `--daemon-stderr <DAEMON_STDERR>` — Stderr will be written into this file
* `--daemon-user <DAEMON_USER>` — If set will drop privileges to the specified user. Note: Both must be given: user and group
* `--daemon-group <DAEMON_GROUP>` — If set will drop privileges to the specified group. Note: Both must be given: user and group



## `reactive-graph graphql-schema`

Prints the GraphQL schema and exits

**Usage:** `reactive-graph graphql-schema <COMMAND>`

###### **Subcommands:**

* `reactive-graph-schema` — Prints the GraphQL schema of the reactive graph
* `dynamic-graph-schema` — Prints the GraphQL schema of the dynamic graph
* `reactive-graph-plugin-schema` — Prints the GraphQL schema of the plugin system of the reactive graph
* `reactive-graph-runtime-schema` — Prints the GraphQL schema of the runtime of the reactive graph



## `reactive-graph graphql-schema reactive-graph-schema`

Prints the GraphQL schema of the reactive graph

**Usage:** `reactive-graph graphql-schema reactive-graph-schema`



## `reactive-graph graphql-schema dynamic-graph-schema`

Prints the GraphQL schema of the dynamic graph

**Usage:** `reactive-graph graphql-schema dynamic-graph-schema`



## `reactive-graph graphql-schema reactive-graph-plugin-schema`

Prints the GraphQL schema of the plugin system of the reactive graph

**Usage:** `reactive-graph graphql-schema reactive-graph-plugin-schema`



## `reactive-graph graphql-schema reactive-graph-runtime-schema`

Prints the GraphQL schema of the runtime of the reactive graph

**Usage:** `reactive-graph graphql-schema reactive-graph-runtime-schema`



## `reactive-graph json-schema`

Prints the JSON schema and exits

**Usage:** `reactive-graph json-schema <COMMAND>`

###### **Subcommands:**

* `types` — Prints the JSON schema of the type system
* `instances` — Prints the JSON schema of the instance system



## `reactive-graph json-schema types`

Prints the JSON schema of the type system

**Usage:** `reactive-graph json-schema types <COMMAND>`

###### **Subcommands:**

* `components` — Prints the JSON schema of the component types
* `entities` — Prints the JSON schema of the entity types
* `relations` — Prints the JSON schema of the relation types
* `flows` — Prints the JSON schema of the flow types



## `reactive-graph json-schema types components`

Prints the JSON schema of the component types

**Usage:** `reactive-graph json-schema types components`



## `reactive-graph json-schema types entities`

Prints the JSON schema of the entity types

**Usage:** `reactive-graph json-schema types entities`



## `reactive-graph json-schema types relations`

Prints the JSON schema of the relation types

**Usage:** `reactive-graph json-schema types relations`



## `reactive-graph json-schema types flows`

Prints the JSON schema of the flow types

**Usage:** `reactive-graph json-schema types flows`



## `reactive-graph json-schema instances`

Prints the JSON schema of the instance system

**Usage:** `reactive-graph json-schema instances <COMMAND>`

###### **Subcommands:**

* `entities` — Prints the JSON schema of the entity instances
* `relations` — Prints the JSON schema of the relation instances
* `flows` — Prints the JSON schema of the flow instances



## `reactive-graph json-schema instances entities`

Prints the JSON schema of the entity instances

**Usage:** `reactive-graph json-schema instances entities`



## `reactive-graph json-schema instances relations`

Prints the JSON schema of the relation instances

**Usage:** `reactive-graph json-schema instances relations`



## `reactive-graph json-schema instances flows`

Prints the JSON schema of the flow instances

**Usage:** `reactive-graph json-schema instances flows`



## `reactive-graph execute-command`

Executes a command on the client

**Usage:** `reactive-graph execute-command <COMMAND_NAME> [COMMAND_ARGUMENTS]...`

###### **Arguments:**

* `<COMMAND_NAME>` — The command name
* `<COMMAND_ARGUMENTS>` — The command arguments



## `reactive-graph instance-info`

Prints information about the instance

**Usage:** `reactive-graph instance-info <COMMAND>`

###### **Subcommands:**

* `get` — Get instance information



## `reactive-graph instance-info get`

Get instance information

**Usage:** `reactive-graph instance-info get`



## `reactive-graph plugins`

Manage plugins

**Usage:** `reactive-graph plugins <COMMAND>`

###### **Subcommands:**

* `list` — Lists all plugins
* `search` — Search for plugins by name, state or stem
* `get` — Prints a single plugin
* `dependencies` — Depends on
* `dependents` — Dependent plugins
* `start` — Starts a plugin
* `stop` — Stops a plugin
* `restart` — Restarts a plugin
* `uninstall` — Uninstall a plugin



## `reactive-graph plugins list`

Lists all plugins

**Usage:** `reactive-graph plugins list`



## `reactive-graph plugins search`

Search for plugins by name, state or stem

**Usage:** `reactive-graph plugins search [OPTIONS]`

###### **Options:**

* `--name <NAME>` — The plugin name
* `--state <STATE>` — The plugin state
* `--stem <STEM>` — The plugin file stem



## `reactive-graph plugins get`

Prints a single plugin

**Usage:** `reactive-graph plugins get <NAME>`

###### **Arguments:**

* `<NAME>` — The plugin name



## `reactive-graph plugins dependencies`

Depends on

**Usage:** `reactive-graph plugins dependencies <NAME>`

###### **Arguments:**

* `<NAME>` — The plugin name



## `reactive-graph plugins dependents`

Dependent plugins

**Usage:** `reactive-graph plugins dependents <NAME>`

###### **Arguments:**

* `<NAME>` — The plugin name



## `reactive-graph plugins start`

Starts a plugin

**Usage:** `reactive-graph plugins start <NAME>`

###### **Arguments:**

* `<NAME>` — The plugin name



## `reactive-graph plugins stop`

Stops a plugin

**Usage:** `reactive-graph plugins stop <NAME>`

###### **Arguments:**

* `<NAME>` — The plugin name



## `reactive-graph plugins restart`

Restarts a plugin

**Usage:** `reactive-graph plugins restart <NAME>`

###### **Arguments:**

* `<NAME>` — The plugin name



## `reactive-graph plugins uninstall`

Uninstall a plugin

**Usage:** `reactive-graph plugins uninstall <NAME>`

###### **Arguments:**

* `<NAME>` — The plugin name



## `reactive-graph remotes`

Manage remotes

**Usage:** `reactive-graph remotes <COMMAND>`

###### **Subcommands:**

* `list` — Lists the remotes
* `add` — Adds a remote
* `remove` — Removes a remote
* `remove-all` — Removes all remotes
* `update` — Updates a remote
* `update-all` — Updates all remotes
* `fetch-remotes-from-remote` — Fetches the remotes from the given remote
* `fetch-remotes-from-all-remotes` — Fetches all remotes from all remotes



## `reactive-graph remotes list`

Lists the remotes

**Usage:** `reactive-graph remotes list`



## `reactive-graph remotes add`

Adds a remote

**Usage:** `reactive-graph remotes add [OPTIONS] --hostname <HOSTNAME>`

###### **Options:**

* `--hostname <HOSTNAME>` — The hostname
* `--port <PORT>` — The port
* `--secure <SECURE>` — The protocol

  Possible values: `true`, `false`




## `reactive-graph remotes remove`

Removes a remote

**Usage:** `reactive-graph remotes remove [OPTIONS] --hostname <HOSTNAME>`

###### **Options:**

* `--hostname <HOSTNAME>` — The hostname
* `--port <PORT>` — The port
* `--secure <SECURE>` — The protocol

  Possible values: `true`, `false`




## `reactive-graph remotes remove-all`

Removes all remotes

**Usage:** `reactive-graph remotes remove-all`



## `reactive-graph remotes update`

Updates a remote

**Usage:** `reactive-graph remotes update [OPTIONS] --hostname <HOSTNAME>`

###### **Options:**

* `--hostname <HOSTNAME>` — The hostname
* `--port <PORT>` — The port
* `--secure <SECURE>` — The protocol

  Possible values: `true`, `false`




## `reactive-graph remotes update-all`

Updates all remotes

**Usage:** `reactive-graph remotes update-all`



## `reactive-graph remotes fetch-remotes-from-remote`

Fetches the remotes from the given remote

**Usage:** `reactive-graph remotes fetch-remotes-from-remote [OPTIONS] --hostname <HOSTNAME>`

###### **Options:**

* `--hostname <HOSTNAME>` — The hostname
* `--port <PORT>` — The port
* `--secure <SECURE>` — The protocol

  Possible values: `true`, `false`




## `reactive-graph remotes fetch-remotes-from-all-remotes`

Fetches all remotes from all remotes

**Usage:** `reactive-graph remotes fetch-remotes-from-all-remotes`



## `reactive-graph shutdown`

Shutdown the runtime

**Usage:** `reactive-graph shutdown`



## `reactive-graph components`

Manage components

**Usage:** `reactive-graph components [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `list` — List all components
* `get` — Prints a single component
* `list-properties` — List the properties of a component
* `list-extensions` — List the extensions of a component
* `get-json-schema` — Prints the JSON Schema of a component
* `create` — Creates a new component
* `delete` — Deletes a component
* `add-property` — Adds a property to a component
* `remove-property` — Removes a property from a component
* `add-extension` — Adds an extension to a component
* `remove-extension` — Removes an extension from a component
* `update-description` — Updates the description of a component
* `json-schema` — Prints the JSON Schema of components

###### **Options:**

* `-o`, `--output-format <OUTPUT_FORMAT>`

  Possible values: `table`, `html-table`, `markdown-table`, `count`, `json`, `json5`, `toml`




## `reactive-graph components list`

List all components

**Usage:** `reactive-graph components list`



## `reactive-graph components get`

Prints a single component

**Usage:** `reactive-graph components get <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name



## `reactive-graph components list-properties`

List the properties of a component

**Usage:** `reactive-graph components list-properties <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name



## `reactive-graph components list-extensions`

List the extensions of a component

**Usage:** `reactive-graph components list-extensions <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name



## `reactive-graph components get-json-schema`

Prints the JSON Schema of a component

**Usage:** `reactive-graph components get-json-schema <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name



## `reactive-graph components create`

Creates a new component

**Usage:** `reactive-graph components create <NAMESPACE> <NAME> [DESCRIPTION]`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name
* `<DESCRIPTION>` — The component description



## `reactive-graph components delete`

Deletes a component

**Usage:** `reactive-graph components delete <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name



## `reactive-graph components add-property`

Adds a property to a component

**Usage:** `reactive-graph components add-property <NAMESPACE> <NAME> <PROPERTY_NAME> <DATA_TYPE> <SOCKET_TYPE> <MUTABILITY> [DESCRIPTION]`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name
* `<PROPERTY_NAME>` — The name of the property
* `<DATA_TYPE>` — The data type of the property
* `<SOCKET_TYPE>` — The socket type of the property
* `<MUTABILITY>` — If the property is mutable or not
* `<DESCRIPTION>` — Description of the property



## `reactive-graph components remove-property`

Removes a property from a component

**Usage:** `reactive-graph components remove-property <NAMESPACE> <NAME> <PROPERTY_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name
* `<PROPERTY_NAME>` — The name of the property



## `reactive-graph components add-extension`

Adds an extension to a component

**Usage:** `reactive-graph components add-extension <NAMESPACE> <NAME> <EXTENSION_NAMESPACE> <EXTENSION_NAME> <DESCRIPTION> <EXTENSION>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name
* `<EXTENSION_NAMESPACE>` — The extension namespace
* `<EXTENSION_NAME>` — The extension name
* `<DESCRIPTION>` — Textual description of the extension
* `<EXTENSION>` — The extension as JSON representation



## `reactive-graph components remove-extension`

Removes an extension from a component

**Usage:** `reactive-graph components remove-extension <NAMESPACE> <NAME> <EXTENSION_NAMESPACE> <EXTENSION_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name
* `<EXTENSION_NAMESPACE>` — The extension namespace
* `<EXTENSION_NAME>` — The extension name



## `reactive-graph components update-description`

Updates the description of a component

**Usage:** `reactive-graph components update-description <NAMESPACE> <NAME> <DESCRIPTION>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name
* `<DESCRIPTION>` — The description to update



## `reactive-graph components json-schema`

Prints the JSON Schema of components

**Usage:** `reactive-graph components json-schema`



## `reactive-graph entity-types`

Manage entity types

**Usage:** `reactive-graph entity-types [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `list` — List all entity types
* `get` — Prints a single entity type
* `list-properties` — List the properties of an entity type
* `list-extensions` — List the extensions of an entity type
* `list-components` — List the components of an entity type
* `get-json-schema` — Prints the JSON Schema of an entity type
* `create` — Creates a new entity type
* `delete` — Deletes a entity type
* `add-property` — Adds a property to an entity type
* `remove-property` — Removes a property from an entity type
* `add-extension` — Adds an extension to an entity type
* `remove-extension` — Removes an extension from an entity type
* `add-component` — Adds a component to an entity type
* `remove-component` — Removes a component from an entity type
* `update-description` — Updates the description of an entity type
* `json-schema` — Prints the JSON Schema of entity types

###### **Options:**

* `-o`, `--output-format <OUTPUT_FORMAT>`

  Possible values: `table`, `html-table`, `markdown-table`, `count`, `json`, `json5`, `toml`




## `reactive-graph entity-types list`

List all entity types

**Usage:** `reactive-graph entity-types list`



## `reactive-graph entity-types get`

Prints a single entity type

**Usage:** `reactive-graph entity-types get <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name



## `reactive-graph entity-types list-properties`

List the properties of an entity type

**Usage:** `reactive-graph entity-types list-properties <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name



## `reactive-graph entity-types list-extensions`

List the extensions of an entity type

**Usage:** `reactive-graph entity-types list-extensions <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name



## `reactive-graph entity-types list-components`

List the components of an entity type

**Usage:** `reactive-graph entity-types list-components <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name



## `reactive-graph entity-types get-json-schema`

Prints the JSON Schema of an entity type

**Usage:** `reactive-graph entity-types get-json-schema <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name



## `reactive-graph entity-types create`

Creates a new entity type

**Usage:** `reactive-graph entity-types create <NAMESPACE> <NAME> [DESCRIPTION]`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<DESCRIPTION>` — The entity type description



## `reactive-graph entity-types delete`

Deletes a entity type

**Usage:** `reactive-graph entity-types delete <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name



## `reactive-graph entity-types add-property`

Adds a property to an entity type

**Usage:** `reactive-graph entity-types add-property <NAMESPACE> <NAME> <PROPERTY_NAME> <DATA_TYPE> <SOCKET_TYPE> <MUTABILITY> [DESCRIPTION]`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<PROPERTY_NAME>` — The name of the property
* `<DATA_TYPE>` — The data type of the property
* `<SOCKET_TYPE>` — The socket type of the property
* `<MUTABILITY>` — If the property is mutable or not
* `<DESCRIPTION>` — Description of the property



## `reactive-graph entity-types remove-property`

Removes a property from an entity type

**Usage:** `reactive-graph entity-types remove-property <NAMESPACE> <NAME> <PROPERTY_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<PROPERTY_NAME>` — The name of the property



## `reactive-graph entity-types add-extension`

Adds an extension to an entity type

**Usage:** `reactive-graph entity-types add-extension <NAMESPACE> <NAME> <EXTENSION_NAMESPACE> <EXTENSION_NAME> <DESCRIPTION> <EXTENSION>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<EXTENSION_NAMESPACE>` — The extension namespace
* `<EXTENSION_NAME>` — The extension name
* `<DESCRIPTION>` — Textual description of the extension
* `<EXTENSION>` — The extension as JSON representation



## `reactive-graph entity-types remove-extension`

Removes an extension from an entity type

**Usage:** `reactive-graph entity-types remove-extension <NAMESPACE> <NAME> <EXTENSION_NAMESPACE> <EXTENSION_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<EXTENSION_NAMESPACE>` — The extension namespace
* `<EXTENSION_NAME>` — The extension name



## `reactive-graph entity-types add-component`

Adds a component to an entity type

**Usage:** `reactive-graph entity-types add-component <NAMESPACE> <NAME> <COMPONENT_NAMESPACE> <COMPONENT_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<COMPONENT_NAMESPACE>` — The component namespace
* `<COMPONENT_NAME>` — The component name



## `reactive-graph entity-types remove-component`

Removes a component from an entity type

**Usage:** `reactive-graph entity-types remove-component <NAMESPACE> <NAME> <COMPONENT_NAMESPACE> <COMPONENT_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<COMPONENT_NAMESPACE>` — The component namespace
* `<COMPONENT_NAME>` — The component name



## `reactive-graph entity-types update-description`

Updates the description of an entity type

**Usage:** `reactive-graph entity-types update-description <NAMESPACE> <NAME> <DESCRIPTION>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<DESCRIPTION>` — The description to update



## `reactive-graph entity-types json-schema`

Prints the JSON Schema of entity types

**Usage:** `reactive-graph entity-types json-schema`



## `reactive-graph relation-types`

Manage entity types

**Usage:** `reactive-graph relation-types [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `list` — List all relation types
* `get` — Prints a single relation type
* `list-properties` — List the properties of an relation type
* `list-extensions` — List the extensions of an relation type
* `list-components` — List the components of an relation type
* `get-json-schema` — Prints the JSON Schema of an relation type
* `create` — Creates a new relation type
* `delete` — Deletes a relation type
* `add-property` — Adds a property to a relation type
* `remove-property` — Removes a property from a relation type
* `add-extension` — Adds an extension to a relation type
* `remove-extension` — Removes an extension from a relation type
* `add-component` — Adds a component to a relation type
* `remove-component` — Removes a component from a relation type
* `update-description` — Updates the description of a relation type
* `json-schema` — Prints the JSON Schema of relation types

###### **Options:**

* `-o`, `--output-format <OUTPUT_FORMAT>`

  Possible values: `table`, `html-table`, `markdown-table`, `count`, `json`, `json5`, `toml`




## `reactive-graph relation-types list`

List all relation types

**Usage:** `reactive-graph relation-types list`



## `reactive-graph relation-types get`

Prints a single relation type

**Usage:** `reactive-graph relation-types get <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name



## `reactive-graph relation-types list-properties`

List the properties of an relation type

**Usage:** `reactive-graph relation-types list-properties <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name



## `reactive-graph relation-types list-extensions`

List the extensions of an relation type

**Usage:** `reactive-graph relation-types list-extensions <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name



## `reactive-graph relation-types list-components`

List the components of an relation type

**Usage:** `reactive-graph relation-types list-components <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name



## `reactive-graph relation-types get-json-schema`

Prints the JSON Schema of an relation type

**Usage:** `reactive-graph relation-types get-json-schema <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name



## `reactive-graph relation-types create`

Creates a new relation type

**Usage:** `reactive-graph relation-types create <OUTBOUND_TYPE_NAMESPACE> <OUTBOUND_TYPE_NAME> <NAMESPACE> <NAME> <INBOUND_TYPE_NAMESPACE> <INBOUND_TYPE_NAME> [DESCRIPTION]`

###### **Arguments:**

* `<OUTBOUND_TYPE_NAMESPACE>` — The outbound entity type namespace
* `<OUTBOUND_TYPE_NAME>` — The outbound entity type name
* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<INBOUND_TYPE_NAMESPACE>` — The inbound entity type namespace
* `<INBOUND_TYPE_NAME>` — The inbound entity type name
* `<DESCRIPTION>` — The relation type description



## `reactive-graph relation-types delete`

Deletes a relation type

**Usage:** `reactive-graph relation-types delete <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name



## `reactive-graph relation-types add-property`

Adds a property to a relation type

**Usage:** `reactive-graph relation-types add-property <NAMESPACE> <NAME> <PROPERTY_NAME> <DATA_TYPE> <SOCKET_TYPE> <MUTABILITY> [DESCRIPTION]`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<PROPERTY_NAME>` — The name of the property
* `<DATA_TYPE>` — The data type of the property
* `<SOCKET_TYPE>` — The socket type of the property
* `<MUTABILITY>` — If the property is mutable or not
* `<DESCRIPTION>` — Description of the property



## `reactive-graph relation-types remove-property`

Removes a property from a relation type

**Usage:** `reactive-graph relation-types remove-property <NAMESPACE> <NAME> <PROPERTY_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<PROPERTY_NAME>` — The name of the property



## `reactive-graph relation-types add-extension`

Adds an extension to a relation type

**Usage:** `reactive-graph relation-types add-extension <NAMESPACE> <NAME> <EXTENSION_NAMESPACE> <EXTENSION_NAME> <DESCRIPTION> <EXTENSION>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<EXTENSION_NAMESPACE>` — The extension namespace
* `<EXTENSION_NAME>` — The extension name
* `<DESCRIPTION>` — Textual description of the extension
* `<EXTENSION>` — The extension as JSON representation



## `reactive-graph relation-types remove-extension`

Removes an extension from a relation type

**Usage:** `reactive-graph relation-types remove-extension <NAMESPACE> <NAME> <EXTENSION_NAMESPACE> <EXTENSION_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<EXTENSION_NAMESPACE>` — The extension namespace
* `<EXTENSION_NAME>` — The extension name



## `reactive-graph relation-types add-component`

Adds a component to a relation type

**Usage:** `reactive-graph relation-types add-component <NAMESPACE> <NAME> <COMPONENT_NAMESPACE> <COMPONENT_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<COMPONENT_NAMESPACE>` — The component namespace
* `<COMPONENT_NAME>` — The component name



## `reactive-graph relation-types remove-component`

Removes a component from a relation type

**Usage:** `reactive-graph relation-types remove-component <NAMESPACE> <NAME> <COMPONENT_NAMESPACE> <COMPONENT_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<COMPONENT_NAMESPACE>` — The component namespace
* `<COMPONENT_NAME>` — The component name



## `reactive-graph relation-types update-description`

Updates the description of a relation type

**Usage:** `reactive-graph relation-types update-description <NAMESPACE> <NAME> <DESCRIPTION>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<DESCRIPTION>` — The description to update



## `reactive-graph relation-types json-schema`

Prints the JSON Schema of relation types

**Usage:** `reactive-graph relation-types json-schema`



## `reactive-graph flow-types`

Manage entity types

**Usage:** `reactive-graph flow-types [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `list` — List all flow types
* `get` — Prints a single flow type
* `list-variables` — List the variables of a flow type
* `list-extensions` — List the extensions of a flow type
* `get-json-schema` — Prints the JSON Schema of a flow type
* `create` — Creates a new flow type
* `delete` — Deletes a flow type
* `add-variable` — Adds a property to a flow type
* `remove-variable` — Removes a property from a flow type
* `add-extension` — Adds an extension to a flow type
* `remove-extension` — Removes an extension from a flow type
* `update-description` — Updates the description of a flow type
* `add-entity-instance` — Adds a new entity instance to a flow type
* `remove-entity-instance` — Removes an entity instance to a flow type
* `json-schema` — Prints the JSON Schema of flow types

###### **Options:**

* `-o`, `--output-format <OUTPUT_FORMAT>`

  Possible values: `table`, `html-table`, `markdown-table`, `count`, `json`, `json5`, `toml`




## `reactive-graph flow-types list`

List all flow types

**Usage:** `reactive-graph flow-types list`



## `reactive-graph flow-types get`

Prints a single flow type

**Usage:** `reactive-graph flow-types get <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name



## `reactive-graph flow-types list-variables`

List the variables of a flow type

**Usage:** `reactive-graph flow-types list-variables <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name



## `reactive-graph flow-types list-extensions`

List the extensions of a flow type

**Usage:** `reactive-graph flow-types list-extensions <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name



## `reactive-graph flow-types get-json-schema`

Prints the JSON Schema of a flow type

**Usage:** `reactive-graph flow-types get-json-schema <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name



## `reactive-graph flow-types create`

Creates a new flow type

**Usage:** `reactive-graph flow-types create <NAMESPACE> <NAME> <ENTITY_TYPE_NAMESPACE> <ENTITY_TYPE_NAME> <WRAPPER_ENTITY_INSTANCE_ID> [DESCRIPTION] [WRAPPER_ENTITY_INSTANCE_DESCRIPTION]`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name
* `<ENTITY_TYPE_NAMESPACE>` — The namespace of the entity type of the wrapper entity instance
* `<ENTITY_TYPE_NAME>` — The type name of the entity type of the wrapper entity instance
* `<WRAPPER_ENTITY_INSTANCE_ID>` — The id of the wrapper entity instance
* `<DESCRIPTION>` — The flow type description
* `<WRAPPER_ENTITY_INSTANCE_DESCRIPTION>` — The description of the wrapper entity instance



## `reactive-graph flow-types delete`

Deletes a flow type

**Usage:** `reactive-graph flow-types delete <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name



## `reactive-graph flow-types add-variable`

Adds a property to a flow type

**Usage:** `reactive-graph flow-types add-variable <NAMESPACE> <NAME> <PROPERTY_NAME> <DATA_TYPE> <SOCKET_TYPE> <MUTABILITY> [DESCRIPTION]`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name
* `<PROPERTY_NAME>` — The name of the property
* `<DATA_TYPE>` — The data type of the property
* `<SOCKET_TYPE>` — The socket type of the property
* `<MUTABILITY>` — If the property is mutable or not
* `<DESCRIPTION>` — Description of the property



## `reactive-graph flow-types remove-variable`

Removes a property from a flow type

**Usage:** `reactive-graph flow-types remove-variable <NAMESPACE> <NAME> <VARIABLE_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name
* `<VARIABLE_NAME>` — The name of the variable



## `reactive-graph flow-types add-extension`

Adds an extension to a flow type

**Usage:** `reactive-graph flow-types add-extension <NAMESPACE> <NAME> <EXTENSION_NAMESPACE> <EXTENSION_NAME> <DESCRIPTION> <EXTENSION>`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name
* `<EXTENSION_NAMESPACE>` — The extension namespace
* `<EXTENSION_NAME>` — The extension name
* `<DESCRIPTION>` — Textual description of the extension
* `<EXTENSION>` — The extension as JSON representation



## `reactive-graph flow-types remove-extension`

Removes an extension from a flow type

**Usage:** `reactive-graph flow-types remove-extension <NAMESPACE> <NAME> <EXTENSION_NAMESPACE> <EXTENSION_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name
* `<EXTENSION_NAMESPACE>` — The extension namespace
* `<EXTENSION_NAME>` — The extension name



## `reactive-graph flow-types update-description`

Updates the description of a flow type

**Usage:** `reactive-graph flow-types update-description <NAMESPACE> <NAME> <DESCRIPTION>`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name
* `<DESCRIPTION>` — The description to update



## `reactive-graph flow-types add-entity-instance`

Adds a new entity instance to a flow type

**Usage:** `reactive-graph flow-types add-entity-instance [OPTIONS] <NAMESPACE> <NAME> <ENTITY_TYPE_NAMESPACE> <ENTITY_TYPE_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name
* `<ENTITY_TYPE_NAMESPACE>` — The entity type namespace
* `<ENTITY_TYPE_NAME>` — The entity type name

###### **Options:**

* `-i`, `--id <ID>` — The entity instance id
* `-d`, `--description <DESCRIPTION>` — The entity instance description
* `-p`, `--properties <PROPERTIES>` — The entity instance properties



## `reactive-graph flow-types remove-entity-instance`

Removes an entity instance to a flow type

**Usage:** `reactive-graph flow-types remove-entity-instance <NAMESPACE> <NAME> <ID>`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name
* `<ID>` — The entity instance to remove from the flow type



## `reactive-graph flow-types json-schema`

Prints the JSON Schema of flow types

**Usage:** `reactive-graph flow-types json-schema`



## `reactive-graph entity-instances`

Manage entity instances

**Usage:** `reactive-graph entity-instances [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `list` — List all entity instances
* `get` — Prints a single entity instance
* `get-by-label` — Prints a single entity instance
* `list-properties` — Lists the properties of an entity instance
* `get-property` — Prints the value of a property of an entity instance
* `set-property` — Sets the value of a property of an entity instance
* `add-property` — Adds a new property to an entity instance
* `remove-property` — Removes a property from an entity instance
* `list-components` — Lists the components of an entity instance
* `add-component` — Adds a component to an entity instance
* `remove-component` — Removes a component from an entity instance
* `create` — Creates a new entity type
* `delete` — CLI argument which identifies an entity instance by its id
* `json-schema` — Prints the JSON Schema of entity instances

###### **Options:**

* `-o`, `--output-format <OUTPUT_FORMAT>`

  Possible values: `table`, `html-table`, `markdown-table`, `count`, `json`, `json5`, `toml`




## `reactive-graph entity-instances list`

List all entity instances

**Usage:** `reactive-graph entity-instances list [OPTIONS]`

###### **Options:**

* `--namespace <NAMESPACE>` — The entity type namespace
* `-n`, `--name <NAME>` — The entity type name
* `-i`, `--id <ID>` — The id of the entity instance
* `-l`, `--label <LABEL>` — The label of the entity instance
* `-p`, `--properties <PROPERTIES>` — The properties to search for
* `-c`, `--components <COMPONENTS>` — The components to search for



## `reactive-graph entity-instances get`

Prints a single entity instance

**Usage:** `reactive-graph entity-instances get <ID>`

###### **Arguments:**

* `<ID>` — The id of the entity instance



## `reactive-graph entity-instances get-by-label`

Prints a single entity instance

**Usage:** `reactive-graph entity-instances get-by-label <LABEL>`

###### **Arguments:**

* `<LABEL>` — The label of the reactive instance



## `reactive-graph entity-instances list-properties`

Lists the properties of an entity instance

**Usage:** `reactive-graph entity-instances list-properties <ID>`

###### **Arguments:**

* `<ID>` — The id of the entity instance



## `reactive-graph entity-instances get-property`

Prints the value of a property of an entity instance

**Usage:** `reactive-graph entity-instances get-property <ID> <PROPERTY_NAME>`

###### **Arguments:**

* `<ID>` — The id of the entity instance
* `<PROPERTY_NAME>` — The name of the property



## `reactive-graph entity-instances set-property`

Sets the value of a property of an entity instance

**Usage:** `reactive-graph entity-instances set-property <ID> <NAME> <VALUE>`

###### **Arguments:**

* `<ID>` — The id of the reactive instance
* `<NAME>` — The name of the property
* `<VALUE>` — The new JSON value of the property.

   'true' is boolean true, '"true"' is the string "true"



## `reactive-graph entity-instances add-property`

Adds a new property to an entity instance

**Usage:** `reactive-graph entity-instances add-property <ID> <PROPERTY_NAME> <DATA_TYPE> <SOCKET_TYPE> <MUTABILITY> [DESCRIPTION]`

###### **Arguments:**

* `<ID>` — The id of the reactive instance
* `<PROPERTY_NAME>` — The name of the property
* `<DATA_TYPE>` — The data type of the property
* `<SOCKET_TYPE>` — The socket type of the property
* `<MUTABILITY>` — If the property is mutable or not
* `<DESCRIPTION>` — Description of the property



## `reactive-graph entity-instances remove-property`

Removes a property from an entity instance

**Usage:** `reactive-graph entity-instances remove-property <ID> <PROPERTY_NAME>`

###### **Arguments:**

* `<ID>` — The id of the entity instance
* `<PROPERTY_NAME>` — The name of the property



## `reactive-graph entity-instances list-components`

Lists the components of an entity instance

**Usage:** `reactive-graph entity-instances list-components <ID>`

###### **Arguments:**

* `<ID>` — The id of the entity instance



## `reactive-graph entity-instances add-component`

Adds a component to an entity instance

**Usage:** `reactive-graph entity-instances add-component <ID> <NAMESPACE> <NAME>`

###### **Arguments:**

* `<ID>` — The id of the reactive instance
* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name



## `reactive-graph entity-instances remove-component`

Removes a component from an entity instance

**Usage:** `reactive-graph entity-instances remove-component <ID> <NAMESPACE> <NAME>`

###### **Arguments:**

* `<ID>` — The id of the reactive instance
* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name



## `reactive-graph entity-instances create`

Creates a new entity type

**Usage:** `reactive-graph entity-instances create [OPTIONS] <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name

###### **Options:**

* `-i`, `--id <ID>` — The entity instance id
* `-d`, `--description <DESCRIPTION>` — The entity instance description
* `-p`, `--properties <PROPERTIES>` — The entity instance properties



## `reactive-graph entity-instances delete`

CLI argument which identifies an entity instance by its id

**Usage:** `reactive-graph entity-instances delete <ID>`

###### **Arguments:**

* `<ID>` — The id of the entity instance



## `reactive-graph entity-instances json-schema`

Prints the JSON Schema of entity instances

**Usage:** `reactive-graph entity-instances json-schema`



## `reactive-graph relation-instances`

Manage relation instances

**Usage:** `reactive-graph relation-instances [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `list` — List all relation instances
* `get` — Prints a single relation instance
* `list-properties` — Lists the properties of a relation instance
* `get-property` — Prints the value of a property of a relation instance
* `set-property` — Sets the value of a property of a relation instance
* `add-property` — Adds a new property to a relation instance
* `remove-property` — Removes a property from a relation instance
* `list-components` — Lists the components of a relation instance
* `add-component` — Adds a component to a relation instance
* `remove-component` — Removes a component from a relation instance
* `create` — Creates a new relation type
* `delete` — CLI argument which identifies an relation instance by its id
* `json-schema` — Prints the JSON Schema of relation instances

###### **Options:**

* `-o`, `--output-format <OUTPUT_FORMAT>`

  Possible values: `table`, `html-table`, `markdown-table`, `count`, `json`, `json5`, `toml`




## `reactive-graph relation-instances list`

List all relation instances

**Usage:** `reactive-graph relation-instances list [OPTIONS]`

###### **Options:**

* `--outbound-id <OUTBOUND_ID>` — The id of the outbound entity instance
* `--namespace <NAMESPACE>` — The relation type namespace
* `-n`, `--name <NAME>` — The relation type name
* `-i`, `--inbound-id <INBOUND_ID>` — The id of the inbound entity instance
* `-p`, `--properties <PROPERTIES>` — The properties to search for
* `-c`, `--components <COMPONENTS>` — The components to search for



## `reactive-graph relation-instances get`

Prints a single relation instance

**Usage:** `reactive-graph relation-instances get --outbound-id <OUTBOUND_ID> --inbound-id <INBOUND_ID> <NAMESPACE> <NAME> <INSTANCE_ID>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<INSTANCE_ID>` — The instance id

###### **Options:**

* `--outbound-id <OUTBOUND_ID>` — The id of the outbound entity instance
* `-i`, `--inbound-id <INBOUND_ID>` — The id of the inbound entity instance



## `reactive-graph relation-instances list-properties`

Lists the properties of a relation instance

**Usage:** `reactive-graph relation-instances list-properties --outbound-id <OUTBOUND_ID> --inbound-id <INBOUND_ID> <NAMESPACE> <NAME> <INSTANCE_ID>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<INSTANCE_ID>` — The instance id

###### **Options:**

* `--outbound-id <OUTBOUND_ID>` — The id of the outbound entity instance
* `-i`, `--inbound-id <INBOUND_ID>` — The id of the inbound entity instance



## `reactive-graph relation-instances get-property`

Prints the value of a property of a relation instance

**Usage:** `reactive-graph relation-instances get-property --outbound-id <OUTBOUND_ID> --inbound-id <INBOUND_ID> <NAMESPACE> <NAME> <INSTANCE_ID> <PROPERTY_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<INSTANCE_ID>` — The instance id
* `<PROPERTY_NAME>` — The name of the property

###### **Options:**

* `--outbound-id <OUTBOUND_ID>` — The id of the outbound entity instance
* `-i`, `--inbound-id <INBOUND_ID>` — The id of the inbound entity instance



## `reactive-graph relation-instances set-property`

Sets the value of a property of a relation instance

**Usage:** `reactive-graph relation-instances set-property --outbound-id <OUTBOUND_ID> --inbound-id <INBOUND_ID> <NAMESPACE> <NAME> <INSTANCE_ID> <PROPERTY_NAME> <PROPERTY_VALUE>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<INSTANCE_ID>` — The instance id
* `<PROPERTY_NAME>` — The name of the property
* `<PROPERTY_VALUE>` — The JSON value of the property.

   'true' is boolean true, '"true"' is the string "true"

###### **Options:**

* `--outbound-id <OUTBOUND_ID>` — The id of the outbound entity instance
* `-i`, `--inbound-id <INBOUND_ID>` — The id of the inbound entity instance



## `reactive-graph relation-instances add-property`

Adds a new property to a relation instance

**Usage:** `reactive-graph relation-instances add-property --outbound-id <OUTBOUND_ID> --inbound-id <INBOUND_ID> <NAMESPACE> <NAME> <INSTANCE_ID> <PROPERTY_NAME> <DATA_TYPE> <SOCKET_TYPE> <MUTABILITY> [DESCRIPTION]`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<INSTANCE_ID>` — The instance id
* `<PROPERTY_NAME>` — The name of the property
* `<DATA_TYPE>` — The data type of the property
* `<SOCKET_TYPE>` — The socket type of the property
* `<MUTABILITY>` — If the property is mutable or not
* `<DESCRIPTION>` — Description of the property

###### **Options:**

* `--outbound-id <OUTBOUND_ID>` — The id of the outbound entity instance
* `-i`, `--inbound-id <INBOUND_ID>` — The id of the inbound entity instance



## `reactive-graph relation-instances remove-property`

Removes a property from a relation instance

**Usage:** `reactive-graph relation-instances remove-property --outbound-id <OUTBOUND_ID> --inbound-id <INBOUND_ID> <NAMESPACE> <NAME> <INSTANCE_ID> <PROPERTY_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<INSTANCE_ID>` — The instance id
* `<PROPERTY_NAME>` — The name of the property

###### **Options:**

* `--outbound-id <OUTBOUND_ID>` — The id of the outbound entity instance
* `-i`, `--inbound-id <INBOUND_ID>` — The id of the inbound entity instance



## `reactive-graph relation-instances list-components`

Lists the components of a relation instance

**Usage:** `reactive-graph relation-instances list-components --outbound-id <OUTBOUND_ID> --inbound-id <INBOUND_ID> <NAMESPACE> <NAME> <INSTANCE_ID>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<INSTANCE_ID>` — The instance id

###### **Options:**

* `--outbound-id <OUTBOUND_ID>` — The id of the outbound entity instance
* `-i`, `--inbound-id <INBOUND_ID>` — The id of the inbound entity instance



## `reactive-graph relation-instances add-component`

Adds a component to a relation instance

**Usage:** `reactive-graph relation-instances add-component --outbound-id <OUTBOUND_ID> --inbound-id <INBOUND_ID> <NAMESPACE> <NAME> <INSTANCE_ID> <COMPONENT_NAMESPACE> <COMPONENT_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<INSTANCE_ID>` — The instance id
* `<COMPONENT_NAMESPACE>` — The component namespace
* `<COMPONENT_NAME>` — The component name

###### **Options:**

* `--outbound-id <OUTBOUND_ID>` — The id of the outbound entity instance
* `-i`, `--inbound-id <INBOUND_ID>` — The id of the inbound entity instance



## `reactive-graph relation-instances remove-component`

Removes a component from a relation instance

**Usage:** `reactive-graph relation-instances remove-component --outbound-id <OUTBOUND_ID> --inbound-id <INBOUND_ID> <NAMESPACE> <NAME> <INSTANCE_ID> <COMPONENT_NAMESPACE> <COMPONENT_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<INSTANCE_ID>` — The instance id
* `<COMPONENT_NAMESPACE>` — The component namespace
* `<COMPONENT_NAME>` — The component name

###### **Options:**

* `--outbound-id <OUTBOUND_ID>` — The id of the outbound entity instance
* `-i`, `--inbound-id <INBOUND_ID>` — The id of the inbound entity instance



## `reactive-graph relation-instances create`

Creates a new relation type

**Usage:** `reactive-graph relation-instances create [OPTIONS] --outbound-id <OUTBOUND_ID> --inbound-id <INBOUND_ID> <NAMESPACE> <NAME> <INSTANCE_ID>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<INSTANCE_ID>` — The instance id

###### **Options:**

* `--outbound-id <OUTBOUND_ID>` — The id of the outbound entity instance
* `-i`, `--inbound-id <INBOUND_ID>` — The id of the inbound entity instance
* `-d`, `--description <DESCRIPTION>` — The relation instance description
* `-p`, `--properties <PROPERTIES>` — The relation instance properties



## `reactive-graph relation-instances delete`

CLI argument which identifies an relation instance by its id

**Usage:** `reactive-graph relation-instances delete --outbound-id <OUTBOUND_ID> --inbound-id <INBOUND_ID> <NAMESPACE> <NAME> <INSTANCE_ID>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<INSTANCE_ID>` — The instance id

###### **Options:**

* `--outbound-id <OUTBOUND_ID>` — The id of the outbound entity instance
* `-i`, `--inbound-id <INBOUND_ID>` — The id of the inbound entity instance



## `reactive-graph relation-instances json-schema`

Prints the JSON Schema of relation instances

**Usage:** `reactive-graph relation-instances json-schema`



## `reactive-graph flow-instances`

Manage flow instances

**Usage:** `reactive-graph flow-instances [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `list` — List all flow instances
* `get` — Prints a single flow instance
* `get-by-label` — Prints a single flow instance
* `create-from-type` — Creates a new flow from the given type
* `delete` — CLI argument which identifies a flow instance by its id
* `json-schema` — Prints the JSON Schema of flow instances

###### **Options:**

* `-o`, `--output-format <OUTPUT_FORMAT>`

  Possible values: `table`, `html-table`, `markdown-table`, `count`, `json`, `json5`, `toml`




## `reactive-graph flow-instances list`

List all flow instances

**Usage:** `reactive-graph flow-instances list [OPTIONS]`

###### **Options:**

* `--namespace <NAMESPACE>` — The entity type namespace
* `-n`, `--name <NAME>` — The entity type name
* `-i`, `--id <ID>` — The id of the entity instance
* `-l`, `--label <LABEL>` — The label of the entity instance



## `reactive-graph flow-instances get`

Prints a single flow instance

**Usage:** `reactive-graph flow-instances get <ID>`

###### **Arguments:**

* `<ID>` — The id of the flow instance



## `reactive-graph flow-instances get-by-label`

Prints a single flow instance

**Usage:** `reactive-graph flow-instances get-by-label <LABEL>`

###### **Arguments:**

* `<LABEL>` — The label of the reactive instance



## `reactive-graph flow-instances create-from-type`

Creates a new flow from the given type

**Usage:** `reactive-graph flow-instances create-from-type [OPTIONS] <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name

###### **Options:**

* `-i`, `--id <ID>` — The id of the flow instance and the wrapper entity instance
* `-v`, `--variables <VARIABLES>` — The entity instance properties
* `-p`, `--properties <PROPERTIES>` — The entity instance properties



## `reactive-graph flow-instances delete`

CLI argument which identifies a flow instance by its id

**Usage:** `reactive-graph flow-instances delete <ID>`

###### **Arguments:**

* `<ID>` — The id of the flow instance



## `reactive-graph flow-instances json-schema`

Prints the JSON Schema of flow instances

**Usage:** `reactive-graph flow-instances json-schema`



## `reactive-graph introspection`

Execute GraphQL introspection queries

**Usage:** `reactive-graph introspection <COMMAND>`

###### **Subcommands:**

* `reactive-graph` — Get the GraphQL schema of the reactive graph
* `dynamic-graph` — Get the GraphQL schema of the dynamic graph
* `reactive-graph-runtime` — Get the GraphQL schema of the reactive graph runtime
* `reactive-graph-plugins` — Get the GraphQL schema of the plugin system of reactive graph



## `reactive-graph introspection reactive-graph`

Get the GraphQL schema of the reactive graph

**Usage:** `reactive-graph introspection reactive-graph`



## `reactive-graph introspection dynamic-graph`

Get the GraphQL schema of the dynamic graph

**Usage:** `reactive-graph introspection dynamic-graph`



## `reactive-graph introspection reactive-graph-runtime`

Get the GraphQL schema of the reactive graph runtime

**Usage:** `reactive-graph introspection reactive-graph-runtime`



## `reactive-graph introspection reactive-graph-plugins`

Get the GraphQL schema of the plugin system of reactive graph

**Usage:** `reactive-graph introspection reactive-graph-plugins`



## `reactive-graph instances`

Manage instances

**Usage:** `reactive-graph instances [WORKING_DIRECTORY] <COMMAND>`

###### **Subcommands:**

* `config` — Configures a local instance,
* `generate-certificate` — Generates certificate of a local instance
* `init` — Initialize the filesystem structure of a new local instance
* `plugins` — Manage the plugins of a local instance
* `repository` — Manage the repositories of a local instance

###### **Arguments:**

* `<WORKING_DIRECTORY>` — The working directory of the instance. Defaults to the current directory



## `reactive-graph instances config`

Configures a local instance,

**Usage:** `reactive-graph instances config <COMMAND>`

###### **Subcommands:**

* `graphql` — Configures the GraphQL server
* `instance` — Configures the instance
* `plugins` — Configures the instance



## `reactive-graph instances config graphql`

Configures the GraphQL server

**Usage:** `reactive-graph instances config graphql [OPTIONS]`

###### **Options:**

* `--hostname <HOSTNAME>` — The hostname to bind the GraphQL HTTP server
* `--port <PORT>` — The port to bind the GraphQL HTTP server
* `--secure <SECURE>` — If true, HTTPS is enabled

  Possible values: `true`, `false`

* `--ssl-certificate-path <SSL_CERTIFICATE_PATH>` — The location of the certificate
* `--ssl-private-key-path <SSL_PRIVATE_KEY_PATH>` — The location of the private key
* `--shutdown-timeout <SHUTDOWN_TIMEOUT>` — Timeout for graceful workers shutdown in seconds. After receiving a stop signal, workers have this much time to finish serving requests. Workers still alive after the timeout are force dropped. By default, shutdown timeout sets to 30 seconds
* `-w`, `--workers <WORKERS>` — The number of workers to start. The default worker count is the number of physical CPU cores available
* `-c`, `--default-context-path <DEFAULT_CONTEXT_PATH>` — The default context path which redirects the root context to a web resource provider



## `reactive-graph instances config instance`

Configures the instance

**Usage:** `reactive-graph instances config instance [OPTIONS]`

###### **Options:**

* `-n`, `--instance-name <NAME>` — The name of the instance
* `-d`, `--instance-description <DESCRIPTION>` — The description of the instance



## `reactive-graph instances config plugins`

Configures the instance

**Usage:** `reactive-graph instances config plugins [OPTIONS]`

###### **Options:**

* `-x`, `--disable-all-plugins <DISABLE_ALL_PLUGINS>` — If true, all plugins will be disabled

  Possible values: `true`, `false`

* `-p`, `--disabled-plugins <DISABLED_PLUGINS>` — The list of plugins to disable
* `-P`, `--enabled-plugins <ENABLED_PLUGINS>` — The list of plugins to enable
* `--disable-hot-deploy <DISABLE_HOT_DEPLOY>` — If true, hot deployment will be disabled

  Possible values: `true`, `false`

* `--hot-deploy-location <HOT_DEPLOY_LOCATION>` — The folder which is watched for hot deployment
* `--install-location <INSTALL_LOCATION>` — The folder which plugins are installed permanently



## `reactive-graph instances generate-certificate`

Generates certificate of a local instance

**Usage:** `reactive-graph instances generate-certificate [COUNTRY_NAME] [ORGANIZATION_NAME] [COMMON_NAME]`

###### **Arguments:**

* `<COUNTRY_NAME>` — Country name
* `<ORGANIZATION_NAME>` — Organization name
* `<COMMON_NAME>` — Common name



## `reactive-graph instances init`

Initialize the filesystem structure of a new local instance

**Usage:** `reactive-graph instances init [OPTIONS] [COUNTRY_NAME] [ORGANIZATION_NAME] [COMMON_NAME]`

###### **Arguments:**

* `<COUNTRY_NAME>` — Country name
* `<ORGANIZATION_NAME>` — Organization name
* `<COMMON_NAME>` — Common name

###### **Options:**

* `--uid <UID>` — The numeric user id of the owner user
* `--gid <GID>` — The numeric group id of the owner group



## `reactive-graph instances plugins`

Manage the plugins of a local instance

**Usage:** `reactive-graph instances plugins <COMMAND>`

###### **Subcommands:**

* `install` — Installs a plugin
* `uninstall` — Uninstalls a plugin



## `reactive-graph instances plugins install`

Installs a plugin

**Usage:** `reactive-graph instances plugins install <PLUGIN_NAME>`

###### **Arguments:**

* `<PLUGIN_NAME>` — The name of the plugin



## `reactive-graph instances plugins uninstall`

Uninstalls a plugin

**Usage:** `reactive-graph instances plugins uninstall <PLUGIN_NAME>`

###### **Arguments:**

* `<PLUGIN_NAME>` — The name of the plugin



## `reactive-graph instances repository`

Manage the repositories of a local instance

**Usage:** `reactive-graph instances repository <COMMAND>`

###### **Subcommands:**

* `init` — Initializes a new local repository in a local instance
* `remove` — Removes a local repository



## `reactive-graph instances repository init`

Initializes a new local repository in a local instance

**Usage:** `reactive-graph instances repository init [OPTIONS] <LOCAL_NAME> [URL]`

###### **Arguments:**

* `<LOCAL_NAME>` — The local name of the repository
* `<URL>` — The remote URL of the repository

###### **Options:**

* `--uid <UID>` — The numeric user id of the owner user
* `--gid <GID>` — The numeric group id of the owner group



## `reactive-graph instances repository remove`

Removes a local repository

**Usage:** `reactive-graph instances repository remove <LOCAL_NAME> [FORCE]`

###### **Arguments:**

* `<LOCAL_NAME>` — The local name of the repository
* `<FORCE>` — If true, the default repository will be deleted

  Possible values: `true`, `false`




## `reactive-graph update`

Update the Reactive Graph binary

**Usage:** `reactive-graph update [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `info` — Shows information about the selected release
* `list` — Lists the releases

###### **Options:**

* `-n`, `--nightly` — Updates to the nightly release
* `-l`, `--latest` — Updates to the latest release. Currently, the latest release is the nightly release. This will change in the future
* `-c`, `--current` — Updates to the current release
* `-v`, `--version <VERSION>` — Updates to a specific version
* `--hide-download-progress` — Hides the download progress
* `--hide-output` — Hides the output
* `-q`, `--quiet` — Hides the download progress and the output
* `-y`, `--no-confirm` — Don't ask



## `reactive-graph update info`

Shows information about the selected release

**Usage:** `reactive-graph update info [OPTIONS]`

###### **Options:**

* `--output-format <OUTPUT_FORMAT>` — The output format

  Possible values: `table`, `html-table`, `markdown-table`, `count`, `json`, `json5`, `toml`




## `reactive-graph update list`

Lists the releases

**Usage:** `reactive-graph update list [OPTIONS]`

###### **Options:**

* `--output-format <OUTPUT_FORMAT>` — The output format

  Possible values: `table`, `html-table`, `markdown-table`, `count`, `json`, `json5`, `toml`




<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

