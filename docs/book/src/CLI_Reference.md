# Command-Line Help for `reactive-graph`

This document contains the help content for the `reactive-graph` command-line program.

**Command Overview:**

* [`reactive-graph`↴](#reactive-graph)
* [`reactive-graph client`↴](#reactive-graph-client)
* [`reactive-graph client execute-command`↴](#reactive-graph-client-execute-command)
* [`reactive-graph client instance-info`↴](#reactive-graph-client-instance-info)
* [`reactive-graph client instance-info get`↴](#reactive-graph-client-instance-info-get)
* [`reactive-graph client plugins`↴](#reactive-graph-client-plugins)
* [`reactive-graph client plugins list`↴](#reactive-graph-client-plugins-list)
* [`reactive-graph client plugins search`↴](#reactive-graph-client-plugins-search)
* [`reactive-graph client plugins get`↴](#reactive-graph-client-plugins-get)
* [`reactive-graph client plugins dependencies`↴](#reactive-graph-client-plugins-dependencies)
* [`reactive-graph client plugins dependents`↴](#reactive-graph-client-plugins-dependents)
* [`reactive-graph client plugins start`↴](#reactive-graph-client-plugins-start)
* [`reactive-graph client plugins stop`↴](#reactive-graph-client-plugins-stop)
* [`reactive-graph client plugins restart`↴](#reactive-graph-client-plugins-restart)
* [`reactive-graph client plugins uninstall`↴](#reactive-graph-client-plugins-uninstall)
* [`reactive-graph client remotes`↴](#reactive-graph-client-remotes)
* [`reactive-graph client remotes list`↴](#reactive-graph-client-remotes-list)
* [`reactive-graph client remotes add`↴](#reactive-graph-client-remotes-add)
* [`reactive-graph client remotes remove`↴](#reactive-graph-client-remotes-remove)
* [`reactive-graph client remotes remove-all`↴](#reactive-graph-client-remotes-remove-all)
* [`reactive-graph client remotes update`↴](#reactive-graph-client-remotes-update)
* [`reactive-graph client remotes update-all`↴](#reactive-graph-client-remotes-update-all)
* [`reactive-graph client remotes fetch-remotes-from-remote`↴](#reactive-graph-client-remotes-fetch-remotes-from-remote)
* [`reactive-graph client remotes fetch-remotes-from-all-remotes`↴](#reactive-graph-client-remotes-fetch-remotes-from-all-remotes)
* [`reactive-graph client shutdown`↴](#reactive-graph-client-shutdown)
* [`reactive-graph client components`↴](#reactive-graph-client-components)
* [`reactive-graph client components list`↴](#reactive-graph-client-components-list)
* [`reactive-graph client components get`↴](#reactive-graph-client-components-get)
* [`reactive-graph client components list-properties`↴](#reactive-graph-client-components-list-properties)
* [`reactive-graph client components list-extensions`↴](#reactive-graph-client-components-list-extensions)
* [`reactive-graph client components create`↴](#reactive-graph-client-components-create)
* [`reactive-graph client components delete`↴](#reactive-graph-client-components-delete)
* [`reactive-graph client components add-property`↴](#reactive-graph-client-components-add-property)
* [`reactive-graph client components remove-property`↴](#reactive-graph-client-components-remove-property)
* [`reactive-graph client components add-extension`↴](#reactive-graph-client-components-add-extension)
* [`reactive-graph client components remove-extension`↴](#reactive-graph-client-components-remove-extension)
* [`reactive-graph client components update-description`↴](#reactive-graph-client-components-update-description)
* [`reactive-graph client entity-types`↴](#reactive-graph-client-entity-types)
* [`reactive-graph client entity-types list`↴](#reactive-graph-client-entity-types-list)
* [`reactive-graph client entity-types get`↴](#reactive-graph-client-entity-types-get)
* [`reactive-graph client entity-types list-properties`↴](#reactive-graph-client-entity-types-list-properties)
* [`reactive-graph client entity-types list-extensions`↴](#reactive-graph-client-entity-types-list-extensions)
* [`reactive-graph client entity-types list-components`↴](#reactive-graph-client-entity-types-list-components)
* [`reactive-graph client entity-types create`↴](#reactive-graph-client-entity-types-create)
* [`reactive-graph client entity-types delete`↴](#reactive-graph-client-entity-types-delete)
* [`reactive-graph client entity-types add-property`↴](#reactive-graph-client-entity-types-add-property)
* [`reactive-graph client entity-types remove-property`↴](#reactive-graph-client-entity-types-remove-property)
* [`reactive-graph client entity-types add-extension`↴](#reactive-graph-client-entity-types-add-extension)
* [`reactive-graph client entity-types remove-extension`↴](#reactive-graph-client-entity-types-remove-extension)
* [`reactive-graph client entity-types add-component`↴](#reactive-graph-client-entity-types-add-component)
* [`reactive-graph client entity-types remove-component`↴](#reactive-graph-client-entity-types-remove-component)
* [`reactive-graph client entity-types update-description`↴](#reactive-graph-client-entity-types-update-description)
* [`reactive-graph client relation-types`↴](#reactive-graph-client-relation-types)
* [`reactive-graph client relation-types list`↴](#reactive-graph-client-relation-types-list)
* [`reactive-graph client relation-types get`↴](#reactive-graph-client-relation-types-get)
* [`reactive-graph client relation-types list-properties`↴](#reactive-graph-client-relation-types-list-properties)
* [`reactive-graph client relation-types list-extensions`↴](#reactive-graph-client-relation-types-list-extensions)
* [`reactive-graph client relation-types list-components`↴](#reactive-graph-client-relation-types-list-components)
* [`reactive-graph client relation-types create`↴](#reactive-graph-client-relation-types-create)
* [`reactive-graph client relation-types delete`↴](#reactive-graph-client-relation-types-delete)
* [`reactive-graph client relation-types add-property`↴](#reactive-graph-client-relation-types-add-property)
* [`reactive-graph client relation-types remove-property`↴](#reactive-graph-client-relation-types-remove-property)
* [`reactive-graph client relation-types add-extension`↴](#reactive-graph-client-relation-types-add-extension)
* [`reactive-graph client relation-types remove-extension`↴](#reactive-graph-client-relation-types-remove-extension)
* [`reactive-graph client relation-types add-component`↴](#reactive-graph-client-relation-types-add-component)
* [`reactive-graph client relation-types remove-component`↴](#reactive-graph-client-relation-types-remove-component)
* [`reactive-graph client relation-types update-description`↴](#reactive-graph-client-relation-types-update-description)
* [`reactive-graph client entity-instances`↴](#reactive-graph-client-entity-instances)
* [`reactive-graph client entity-instances list`↴](#reactive-graph-client-entity-instances-list)
* [`reactive-graph client entity-instances get`↴](#reactive-graph-client-entity-instances-get)

## `reactive-graph`

Reactive Graph Flow

**Usage:** `reactive-graph [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `client` — Connects to a client

###### **Options:**

* `--logging-config <LOGGING_CONFIG>` — The logging config location
* `--instance-config <INSTANCE_CONFIG>` — The instance config location
* `--graphql-config <GRAPHQL_CONFIG>` — The GraphQL config location
* `--plugins-config <PLUGINS_CONFIG>` — The plugins config location
* `-n`, `--instance-name <INSTANCE_NAME>` — The name of the instance
* `-d`, `--instance-description <INSTANCE_DESCRIPTION>` — The description of the instance
* `--hostname <HOSTNAME>` — The hostname to bind the GraphQL HTTP server
* `--port <PORT>` — The port to bind the GraphQL HTTP server
* `--secure <SECURE>` — If true, HTTPS is enabled

  Possible values: `true`, `false`

* `--ssl-certificate-path <SSL_CERTIFICATE_PATH>` — The location of the certificate
* `--ssl-private-key-path <SSL_PRIVATE_KEY_PATH>` — The location of the private key
* `--shutdown-timeout <SHUTDOWN_TIMEOUT>` — Timeout for graceful workers shutdown in seconds. After receiving a stop signal, workers have this much time to finish serving requests. Workers still alive after the timeout are force dropped. By default shutdown timeout sets to 30 seconds
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




## `reactive-graph client`

Connects to a client

**Usage:** `reactive-graph client [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `execute-command` — Executes a command on the client
* `instance-info` — Prints information about the instance
* `plugins` — Manage plugins
* `remotes` — Manage remotes
* `shutdown` — Shutdown the runtime
* `components` — Manage components
* `entity-types` — Manage entity types
* `relation-types` — Manage entity types
* `entity-instances` — Manage entity instances

###### **Options:**

* `--hostname <HOSTNAME>` — The hostname to connect to
* `--port <PORT>` — The port to connect to
* `--secure <SECURE>` — If true, connects via HTTPS

  Possible values: `true`, `false`

* `--endpoint-graphql <ENDPOINT_GRAPHQL>` — The endpoint to use
* `--endpoint-dynamic-graph <ENDPOINT_DYNAMIC_GRAPH>` — The endpoint to use
* `--endpoint-runtime <ENDPOINT_RUNTIME>` — The endpoint to use
* `--endpoint-plugins <ENDPOINT_PLUGINS>` — The endpoint to use
* `--bearer <BEARER>` — The authentication token



## `reactive-graph client execute-command`

Executes a command on the client

**Usage:** `reactive-graph client execute-command <COMMAND_NAME> [COMMAND_ARGUMENTS]...`

###### **Arguments:**

* `<COMMAND_NAME>` — The command name
* `<COMMAND_ARGUMENTS>` — The command arguments



## `reactive-graph client instance-info`

Prints information about the instance

**Usage:** `reactive-graph client instance-info <COMMAND>`

###### **Subcommands:**

* `get` — Get instance information



## `reactive-graph client instance-info get`

Get instance information

**Usage:** `reactive-graph client instance-info get`



## `reactive-graph client plugins`

Manage plugins

**Usage:** `reactive-graph client plugins <COMMAND>`

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



## `reactive-graph client plugins list`

Lists all plugins

**Usage:** `reactive-graph client plugins list`



## `reactive-graph client plugins search`

Search for plugins by name, state or stem

**Usage:** `reactive-graph client plugins search [OPTIONS]`

###### **Options:**

* `--name <NAME>` — The plugin name
* `--state <STATE>` — The plugin state
* `--stem <STEM>` — The plugin file stem



## `reactive-graph client plugins get`

Prints a single plugin

**Usage:** `reactive-graph client plugins get <NAME>`

###### **Arguments:**

* `<NAME>` — The plugin name



## `reactive-graph client plugins dependencies`

Depends on

**Usage:** `reactive-graph client plugins dependencies <NAME>`

###### **Arguments:**

* `<NAME>` — The plugin name



## `reactive-graph client plugins dependents`

Dependent plugins

**Usage:** `reactive-graph client plugins dependents <NAME>`

###### **Arguments:**

* `<NAME>` — The plugin name



## `reactive-graph client plugins start`

Starts a plugin

**Usage:** `reactive-graph client plugins start <NAME>`

###### **Arguments:**

* `<NAME>` — The plugin name



## `reactive-graph client plugins stop`

Stops a plugin

**Usage:** `reactive-graph client plugins stop <NAME>`

###### **Arguments:**

* `<NAME>` — The plugin name



## `reactive-graph client plugins restart`

Restarts a plugin

**Usage:** `reactive-graph client plugins restart <NAME>`

###### **Arguments:**

* `<NAME>` — The plugin name



## `reactive-graph client plugins uninstall`

Uninstall a plugin

**Usage:** `reactive-graph client plugins uninstall <NAME>`

###### **Arguments:**

* `<NAME>` — The plugin name



## `reactive-graph client remotes`

Manage remotes

**Usage:** `reactive-graph client remotes <COMMAND>`

###### **Subcommands:**

* `list` — Lists the remotes
* `add` — Adds a remote
* `remove` — Removes a remote
* `remove-all` — Removes all remotes
* `update` — Updates a remote
* `update-all` — Updates all remotes
* `fetch-remotes-from-remote` — Fetches the remotes from the given remote
* `fetch-remotes-from-all-remotes` — Fetches all remotes from all remotes



## `reactive-graph client remotes list`

Lists the remotes

**Usage:** `reactive-graph client remotes list`



## `reactive-graph client remotes add`

Adds a remote

**Usage:** `reactive-graph client remotes add [OPTIONS] --hostname <HOSTNAME>`

###### **Options:**

* `--hostname <HOSTNAME>` — The hostname
* `--port <PORT>` — The port
* `--secure <SECURE>` — The protocol

  Possible values: `true`, `false`




## `reactive-graph client remotes remove`

Removes a remote

**Usage:** `reactive-graph client remotes remove [OPTIONS] --hostname <HOSTNAME>`

###### **Options:**

* `--hostname <HOSTNAME>` — The hostname
* `--port <PORT>` — The port
* `--secure <SECURE>` — The protocol

  Possible values: `true`, `false`




## `reactive-graph client remotes remove-all`

Removes all remotes

**Usage:** `reactive-graph client remotes remove-all`



## `reactive-graph client remotes update`

Updates a remote

**Usage:** `reactive-graph client remotes update [OPTIONS] --hostname <HOSTNAME>`

###### **Options:**

* `--hostname <HOSTNAME>` — The hostname
* `--port <PORT>` — The port
* `--secure <SECURE>` — The protocol

  Possible values: `true`, `false`




## `reactive-graph client remotes update-all`

Updates all remotes

**Usage:** `reactive-graph client remotes update-all`



## `reactive-graph client remotes fetch-remotes-from-remote`

Fetches the remotes from the given remote

**Usage:** `reactive-graph client remotes fetch-remotes-from-remote [OPTIONS] --hostname <HOSTNAME>`

###### **Options:**

* `--hostname <HOSTNAME>` — The hostname
* `--port <PORT>` — The port
* `--secure <SECURE>` — The protocol

  Possible values: `true`, `false`




## `reactive-graph client remotes fetch-remotes-from-all-remotes`

Fetches all remotes from all remotes

**Usage:** `reactive-graph client remotes fetch-remotes-from-all-remotes`



## `reactive-graph client shutdown`

Shutdown the runtime

**Usage:** `reactive-graph client shutdown`



## `reactive-graph client components`

Manage components

**Usage:** `reactive-graph client components [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `list` — List all components
* `get` — Prints a single component
* `list-properties` — List the properties of a component
* `list-extensions` — List the extensions of a component
* `create` — Creates a new component
* `delete` — Deletes a component
* `add-property` — Adds a property to a component
* `remove-property` — Removes a property from a component
* `add-extension` — Adds an extension to a component
* `remove-extension` — Removes an extension from a component
* `update-description` — Updates the description of a component

###### **Options:**

* `-o`, `--output-format <OUTPUT_FORMAT>`

  Possible values: `table`, `json`, `json5`, `toml`




## `reactive-graph client components list`

List all components

**Usage:** `reactive-graph client components list`



## `reactive-graph client components get`

Prints a single component

**Usage:** `reactive-graph client components get <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name



## `reactive-graph client components list-properties`

List the properties of a component

**Usage:** `reactive-graph client components list-properties <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name



## `reactive-graph client components list-extensions`

List the extensions of a component

**Usage:** `reactive-graph client components list-extensions <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name



## `reactive-graph client components create`

Creates a new component

**Usage:** `reactive-graph client components create <NAMESPACE> <NAME> [DESCRIPTION]`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name
* `<DESCRIPTION>` — The component description



## `reactive-graph client components delete`

Deletes a component

**Usage:** `reactive-graph client components delete <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name



## `reactive-graph client components add-property`

Adds a property to a component

**Usage:** `reactive-graph client components add-property <NAMESPACE> <NAME> <PROPERTY_NAME> <DATA_TYPE> <SOCKET_TYPE> <MUTABILITY> [DESCRIPTION]`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name
* `<PROPERTY_NAME>` — The name of the property
* `<DATA_TYPE>` — The data type of the property
* `<SOCKET_TYPE>` — The socket type of the property
* `<MUTABILITY>` — If the property is mutable or not
* `<DESCRIPTION>` — Description of the property



## `reactive-graph client components remove-property`

Removes a property from a component

**Usage:** `reactive-graph client components remove-property <NAMESPACE> <NAME> <PROPERTY_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name
* `<PROPERTY_NAME>` — The name of the property



## `reactive-graph client components add-extension`

Adds an extension to a component

**Usage:** `reactive-graph client components add-extension <NAMESPACE> <NAME> <EXTENSION_NAMESPACE> <EXTENSION_NAME> <DESCRIPTION> <EXTENSION>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name
* `<EXTENSION_NAMESPACE>` — The extension namespace
* `<EXTENSION_NAME>` — The extension name
* `<DESCRIPTION>` — Textual description of the extension
* `<EXTENSION>` — The extension as JSON representation



## `reactive-graph client components remove-extension`

Removes an extension from a component

**Usage:** `reactive-graph client components remove-extension <NAMESPACE> <NAME> <EXTENSION_NAMESPACE> <EXTENSION_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name
* `<EXTENSION_NAMESPACE>` — The extension namespace
* `<EXTENSION_NAME>` — The extension name



## `reactive-graph client components update-description`

Updates the description of a component

**Usage:** `reactive-graph client components update-description <NAMESPACE> <NAME> <DESCRIPTION>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name
* `<DESCRIPTION>` — The description to update



## `reactive-graph client entity-types`

Manage entity types

**Usage:** `reactive-graph client entity-types [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `list` — List all entity types
* `get` — Prints a single entity type
* `list-properties` — List the properties of an entity type
* `list-extensions` — List the extensions of an entity type
* `list-components` — List the components of an entity type
* `create` — Creates a new entity type
* `delete` — Deletes a entity type
* `add-property` — Adds a property to an entity type
* `remove-property` — Removes a property from an entity type
* `add-extension` — Adds an extension to an entity type
* `remove-extension` — Removes an extension from an entity type
* `add-component` — Adds a component to an entity type
* `remove-component` — Removes a component from an entity type
* `update-description` — Updates the description of an entity type

###### **Options:**

* `-o`, `--output-format <OUTPUT_FORMAT>`

  Possible values: `table`, `json`, `json5`, `toml`




## `reactive-graph client entity-types list`

List all entity types

**Usage:** `reactive-graph client entity-types list`



## `reactive-graph client entity-types get`

Prints a single entity type

**Usage:** `reactive-graph client entity-types get <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name



## `reactive-graph client entity-types list-properties`

List the properties of an entity type

**Usage:** `reactive-graph client entity-types list-properties <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name



## `reactive-graph client entity-types list-extensions`

List the extensions of an entity type

**Usage:** `reactive-graph client entity-types list-extensions <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name



## `reactive-graph client entity-types list-components`

List the components of an entity type

**Usage:** `reactive-graph client entity-types list-components <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name



## `reactive-graph client entity-types create`

Creates a new entity type

**Usage:** `reactive-graph client entity-types create <NAMESPACE> <NAME> [DESCRIPTION]`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<DESCRIPTION>` — The entity type description



## `reactive-graph client entity-types delete`

Deletes a entity type

**Usage:** `reactive-graph client entity-types delete <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name



## `reactive-graph client entity-types add-property`

Adds a property to an entity type

**Usage:** `reactive-graph client entity-types add-property <NAMESPACE> <NAME> <PROPERTY_NAME> <DATA_TYPE> <SOCKET_TYPE> <MUTABILITY> [DESCRIPTION]`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<PROPERTY_NAME>` — The name of the property
* `<DATA_TYPE>` — The data type of the property
* `<SOCKET_TYPE>` — The socket type of the property
* `<MUTABILITY>` — If the property is mutable or not
* `<DESCRIPTION>` — Description of the property



## `reactive-graph client entity-types remove-property`

Removes a property from an entity type

**Usage:** `reactive-graph client entity-types remove-property <NAMESPACE> <NAME> <PROPERTY_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<PROPERTY_NAME>` — The name of the property



## `reactive-graph client entity-types add-extension`

Adds an extension to an entity type

**Usage:** `reactive-graph client entity-types add-extension <NAMESPACE> <NAME> <EXTENSION_NAMESPACE> <EXTENSION_NAME> <DESCRIPTION> <EXTENSION>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<EXTENSION_NAMESPACE>` — The extension namespace
* `<EXTENSION_NAME>` — The extension name
* `<DESCRIPTION>` — Textual description of the extension
* `<EXTENSION>` — The extension as JSON representation



## `reactive-graph client entity-types remove-extension`

Removes an extension from an entity type

**Usage:** `reactive-graph client entity-types remove-extension <NAMESPACE> <NAME> <EXTENSION_NAMESPACE> <EXTENSION_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<EXTENSION_NAMESPACE>` — The extension namespace
* `<EXTENSION_NAME>` — The extension name



## `reactive-graph client entity-types add-component`

Adds a component to an entity type

**Usage:** `reactive-graph client entity-types add-component <NAMESPACE> <NAME> <COMPONENT_NAMESPACE> <COMPONENT_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<COMPONENT_NAMESPACE>` — The component namespace
* `<COMPONENT_NAME>` — The component name



## `reactive-graph client entity-types remove-component`

Removes a component from an entity type

**Usage:** `reactive-graph client entity-types remove-component <NAMESPACE> <NAME> <COMPONENT_NAMESPACE> <COMPONENT_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<COMPONENT_NAMESPACE>` — The component namespace
* `<COMPONENT_NAME>` — The component name



## `reactive-graph client entity-types update-description`

Updates the description of an entity type

**Usage:** `reactive-graph client entity-types update-description <NAMESPACE> <NAME> <DESCRIPTION>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<DESCRIPTION>` — The description to update



## `reactive-graph client relation-types`

Manage entity types

**Usage:** `reactive-graph client relation-types [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `list` — List all relation types
* `get` — Prints a single relation type
* `list-properties` — List the properties of an relation type
* `list-extensions` — List the extensions of an relation type
* `list-components` — List the components of an relation type
* `create` — Creates a new relation type
* `delete` — Deletes a relation type
* `add-property` — Adds a property to a relation type
* `remove-property` — Removes a property from a relation type
* `add-extension` — Adds an extension to a relation type
* `remove-extension` — Removes an extension from a relation type
* `add-component` — Adds a component to a relation type
* `remove-component` — Removes a component from a relation type
* `update-description` — Updates the description of a relation type

###### **Options:**

* `-o`, `--output-format <OUTPUT_FORMAT>`

  Possible values: `table`, `json`, `json5`, `toml`




## `reactive-graph client relation-types list`

List all relation types

**Usage:** `reactive-graph client relation-types list`



## `reactive-graph client relation-types get`

Prints a single relation type

**Usage:** `reactive-graph client relation-types get <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name



## `reactive-graph client relation-types list-properties`

List the properties of an relation type

**Usage:** `reactive-graph client relation-types list-properties <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name



## `reactive-graph client relation-types list-extensions`

List the extensions of an relation type

**Usage:** `reactive-graph client relation-types list-extensions <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name



## `reactive-graph client relation-types list-components`

List the components of an relation type

**Usage:** `reactive-graph client relation-types list-components <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name



## `reactive-graph client relation-types create`

Creates a new relation type

**Usage:** `reactive-graph client relation-types create <OUTBOUND_TYPE_NAMESPACE> <OUTBOUND_TYPE_NAME> <NAMESPACE> <NAME> <INBOUND_TYPE_NAMESPACE> <INBOUND_TYPE_NAME> [DESCRIPTION]`

###### **Arguments:**

* `<OUTBOUND_TYPE_NAMESPACE>` — The outbound entity type namespace
* `<OUTBOUND_TYPE_NAME>` — The outbound entity type name
* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<INBOUND_TYPE_NAMESPACE>` — The inbound entity type namespace
* `<INBOUND_TYPE_NAME>` — The inbound entity type name
* `<DESCRIPTION>` — The relation type description



## `reactive-graph client relation-types delete`

Deletes a relation type

**Usage:** `reactive-graph client relation-types delete <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name



## `reactive-graph client relation-types add-property`

Adds a property to a relation type

**Usage:** `reactive-graph client relation-types add-property <NAMESPACE> <NAME> <PROPERTY_NAME> <DATA_TYPE> <SOCKET_TYPE> <MUTABILITY> [DESCRIPTION]`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<PROPERTY_NAME>` — The name of the property
* `<DATA_TYPE>` — The data type of the property
* `<SOCKET_TYPE>` — The socket type of the property
* `<MUTABILITY>` — If the property is mutable or not
* `<DESCRIPTION>` — Description of the property



## `reactive-graph client relation-types remove-property`

Removes a property from a relation type

**Usage:** `reactive-graph client relation-types remove-property <NAMESPACE> <NAME> <PROPERTY_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<PROPERTY_NAME>` — The name of the property



## `reactive-graph client relation-types add-extension`

Adds an extension to a relation type

**Usage:** `reactive-graph client relation-types add-extension <NAMESPACE> <NAME> <EXTENSION_NAMESPACE> <EXTENSION_NAME> <DESCRIPTION> <EXTENSION>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<EXTENSION_NAMESPACE>` — The extension namespace
* `<EXTENSION_NAME>` — The extension name
* `<DESCRIPTION>` — Textual description of the extension
* `<EXTENSION>` — The extension as JSON representation



## `reactive-graph client relation-types remove-extension`

Removes an extension from a relation type

**Usage:** `reactive-graph client relation-types remove-extension <NAMESPACE> <NAME> <EXTENSION_NAMESPACE> <EXTENSION_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<EXTENSION_NAMESPACE>` — The extension namespace
* `<EXTENSION_NAME>` — The extension name



## `reactive-graph client relation-types add-component`

Adds a component to a relation type

**Usage:** `reactive-graph client relation-types add-component <NAMESPACE> <NAME> <COMPONENT_NAMESPACE> <COMPONENT_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<COMPONENT_NAMESPACE>` — The component namespace
* `<COMPONENT_NAME>` — The component name



## `reactive-graph client relation-types remove-component`

Removes a component from a relation type

**Usage:** `reactive-graph client relation-types remove-component <NAMESPACE> <NAME> <COMPONENT_NAMESPACE> <COMPONENT_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<COMPONENT_NAMESPACE>` — The component namespace
* `<COMPONENT_NAME>` — The component name



## `reactive-graph client relation-types update-description`

Updates the description of a relation type

**Usage:** `reactive-graph client relation-types update-description <NAMESPACE> <NAME> <DESCRIPTION>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<DESCRIPTION>` — The description to update



## `reactive-graph client entity-instances`

Manage entity instances

**Usage:** `reactive-graph client entity-instances [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `list` — List all entity instances
* `get` — Prints a single entity instance

###### **Options:**

* `-o`, `--output-format <OUTPUT_FORMAT>`

  Possible values: `table`, `json`, `json5`, `toml`




## `reactive-graph client entity-instances list`

List all entity instances

**Usage:** `reactive-graph client entity-instances list`



## `reactive-graph client entity-instances get`

Prints a single entity instance

**Usage:** `reactive-graph client entity-instances get <ID>`

###### **Arguments:**

* `<ID>` — The id of the entity instance



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

