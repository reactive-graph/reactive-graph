# Command-Line Help for `reactive-graph-client`

This document contains the help content for the `reactive-graph-client` command-line program.

**Command Overview:**

* [`reactive-graph-client`↴](#reactive-graph-client)
* [`reactive-graph-client shell-completions`↴](#reactive-graph-client-shell-completions)
* [`reactive-graph-client shell-completions print`↴](#reactive-graph-client-shell-completions-print)
* [`reactive-graph-client shell-completions install`↴](#reactive-graph-client-shell-completions-install)
* [`reactive-graph-client man-pages`↴](#reactive-graph-client-man-pages)
* [`reactive-graph-client man-pages print`↴](#reactive-graph-client-man-pages-print)
* [`reactive-graph-client man-pages install`↴](#reactive-graph-client-man-pages-install)
* [`reactive-graph-client print-markdown-help`↴](#reactive-graph-client-print-markdown-help)
* [`reactive-graph-client info`↴](#reactive-graph-client-info)
* [`reactive-graph-client execute-command`↴](#reactive-graph-client-execute-command)
* [`reactive-graph-client instance-info`↴](#reactive-graph-client-instance-info)
* [`reactive-graph-client instance-info get`↴](#reactive-graph-client-instance-info-get)
* [`reactive-graph-client plugins`↴](#reactive-graph-client-plugins)
* [`reactive-graph-client plugins list`↴](#reactive-graph-client-plugins-list)
* [`reactive-graph-client plugins search`↴](#reactive-graph-client-plugins-search)
* [`reactive-graph-client plugins get`↴](#reactive-graph-client-plugins-get)
* [`reactive-graph-client plugins dependencies`↴](#reactive-graph-client-plugins-dependencies)
* [`reactive-graph-client plugins dependents`↴](#reactive-graph-client-plugins-dependents)
* [`reactive-graph-client plugins start`↴](#reactive-graph-client-plugins-start)
* [`reactive-graph-client plugins stop`↴](#reactive-graph-client-plugins-stop)
* [`reactive-graph-client plugins restart`↴](#reactive-graph-client-plugins-restart)
* [`reactive-graph-client plugins uninstall`↴](#reactive-graph-client-plugins-uninstall)
* [`reactive-graph-client remotes`↴](#reactive-graph-client-remotes)
* [`reactive-graph-client remotes list`↴](#reactive-graph-client-remotes-list)
* [`reactive-graph-client remotes add`↴](#reactive-graph-client-remotes-add)
* [`reactive-graph-client remotes remove`↴](#reactive-graph-client-remotes-remove)
* [`reactive-graph-client remotes remove-all`↴](#reactive-graph-client-remotes-remove-all)
* [`reactive-graph-client remotes update`↴](#reactive-graph-client-remotes-update)
* [`reactive-graph-client remotes update-all`↴](#reactive-graph-client-remotes-update-all)
* [`reactive-graph-client remotes fetch-remotes-from-remote`↴](#reactive-graph-client-remotes-fetch-remotes-from-remote)
* [`reactive-graph-client remotes fetch-remotes-from-all-remotes`↴](#reactive-graph-client-remotes-fetch-remotes-from-all-remotes)
* [`reactive-graph-client shutdown`↴](#reactive-graph-client-shutdown)
* [`reactive-graph-client components`↴](#reactive-graph-client-components)
* [`reactive-graph-client components list`↴](#reactive-graph-client-components-list)
* [`reactive-graph-client components get`↴](#reactive-graph-client-components-get)
* [`reactive-graph-client components list-properties`↴](#reactive-graph-client-components-list-properties)
* [`reactive-graph-client components list-extensions`↴](#reactive-graph-client-components-list-extensions)
* [`reactive-graph-client components create`↴](#reactive-graph-client-components-create)
* [`reactive-graph-client components delete`↴](#reactive-graph-client-components-delete)
* [`reactive-graph-client components add-property`↴](#reactive-graph-client-components-add-property)
* [`reactive-graph-client components remove-property`↴](#reactive-graph-client-components-remove-property)
* [`reactive-graph-client components add-extension`↴](#reactive-graph-client-components-add-extension)
* [`reactive-graph-client components remove-extension`↴](#reactive-graph-client-components-remove-extension)
* [`reactive-graph-client components update-description`↴](#reactive-graph-client-components-update-description)
* [`reactive-graph-client components json-schema`↴](#reactive-graph-client-components-json-schema)
* [`reactive-graph-client entity-types`↴](#reactive-graph-client-entity-types)
* [`reactive-graph-client entity-types list`↴](#reactive-graph-client-entity-types-list)
* [`reactive-graph-client entity-types get`↴](#reactive-graph-client-entity-types-get)
* [`reactive-graph-client entity-types list-properties`↴](#reactive-graph-client-entity-types-list-properties)
* [`reactive-graph-client entity-types list-extensions`↴](#reactive-graph-client-entity-types-list-extensions)
* [`reactive-graph-client entity-types list-components`↴](#reactive-graph-client-entity-types-list-components)
* [`reactive-graph-client entity-types create`↴](#reactive-graph-client-entity-types-create)
* [`reactive-graph-client entity-types delete`↴](#reactive-graph-client-entity-types-delete)
* [`reactive-graph-client entity-types add-property`↴](#reactive-graph-client-entity-types-add-property)
* [`reactive-graph-client entity-types remove-property`↴](#reactive-graph-client-entity-types-remove-property)
* [`reactive-graph-client entity-types add-extension`↴](#reactive-graph-client-entity-types-add-extension)
* [`reactive-graph-client entity-types remove-extension`↴](#reactive-graph-client-entity-types-remove-extension)
* [`reactive-graph-client entity-types add-component`↴](#reactive-graph-client-entity-types-add-component)
* [`reactive-graph-client entity-types remove-component`↴](#reactive-graph-client-entity-types-remove-component)
* [`reactive-graph-client entity-types update-description`↴](#reactive-graph-client-entity-types-update-description)
* [`reactive-graph-client entity-types json-schema`↴](#reactive-graph-client-entity-types-json-schema)
* [`reactive-graph-client relation-types`↴](#reactive-graph-client-relation-types)
* [`reactive-graph-client relation-types list`↴](#reactive-graph-client-relation-types-list)
* [`reactive-graph-client relation-types get`↴](#reactive-graph-client-relation-types-get)
* [`reactive-graph-client relation-types list-properties`↴](#reactive-graph-client-relation-types-list-properties)
* [`reactive-graph-client relation-types list-extensions`↴](#reactive-graph-client-relation-types-list-extensions)
* [`reactive-graph-client relation-types list-components`↴](#reactive-graph-client-relation-types-list-components)
* [`reactive-graph-client relation-types create`↴](#reactive-graph-client-relation-types-create)
* [`reactive-graph-client relation-types delete`↴](#reactive-graph-client-relation-types-delete)
* [`reactive-graph-client relation-types add-property`↴](#reactive-graph-client-relation-types-add-property)
* [`reactive-graph-client relation-types remove-property`↴](#reactive-graph-client-relation-types-remove-property)
* [`reactive-graph-client relation-types add-extension`↴](#reactive-graph-client-relation-types-add-extension)
* [`reactive-graph-client relation-types remove-extension`↴](#reactive-graph-client-relation-types-remove-extension)
* [`reactive-graph-client relation-types add-component`↴](#reactive-graph-client-relation-types-add-component)
* [`reactive-graph-client relation-types remove-component`↴](#reactive-graph-client-relation-types-remove-component)
* [`reactive-graph-client relation-types update-description`↴](#reactive-graph-client-relation-types-update-description)
* [`reactive-graph-client relation-types json-schema`↴](#reactive-graph-client-relation-types-json-schema)
* [`reactive-graph-client flow-types`↴](#reactive-graph-client-flow-types)
* [`reactive-graph-client flow-types list`↴](#reactive-graph-client-flow-types-list)
* [`reactive-graph-client flow-types get`↴](#reactive-graph-client-flow-types-get)
* [`reactive-graph-client flow-types list-variables`↴](#reactive-graph-client-flow-types-list-variables)
* [`reactive-graph-client flow-types list-extensions`↴](#reactive-graph-client-flow-types-list-extensions)
* [`reactive-graph-client flow-types create`↴](#reactive-graph-client-flow-types-create)
* [`reactive-graph-client flow-types delete`↴](#reactive-graph-client-flow-types-delete)
* [`reactive-graph-client flow-types add-variable`↴](#reactive-graph-client-flow-types-add-variable)
* [`reactive-graph-client flow-types remove-variable`↴](#reactive-graph-client-flow-types-remove-variable)
* [`reactive-graph-client flow-types add-extension`↴](#reactive-graph-client-flow-types-add-extension)
* [`reactive-graph-client flow-types remove-extension`↴](#reactive-graph-client-flow-types-remove-extension)
* [`reactive-graph-client flow-types update-description`↴](#reactive-graph-client-flow-types-update-description)
* [`reactive-graph-client flow-types add-entity-instance`↴](#reactive-graph-client-flow-types-add-entity-instance)
* [`reactive-graph-client flow-types remove-entity-instance`↴](#reactive-graph-client-flow-types-remove-entity-instance)
* [`reactive-graph-client flow-types json-schema`↴](#reactive-graph-client-flow-types-json-schema)
* [`reactive-graph-client entity-instances`↴](#reactive-graph-client-entity-instances)
* [`reactive-graph-client entity-instances list`↴](#reactive-graph-client-entity-instances-list)
* [`reactive-graph-client entity-instances get`↴](#reactive-graph-client-entity-instances-get)
* [`reactive-graph-client entity-instances get-by-label`↴](#reactive-graph-client-entity-instances-get-by-label)
* [`reactive-graph-client entity-instances list-properties`↴](#reactive-graph-client-entity-instances-list-properties)
* [`reactive-graph-client entity-instances get-property`↴](#reactive-graph-client-entity-instances-get-property)
* [`reactive-graph-client entity-instances set-property`↴](#reactive-graph-client-entity-instances-set-property)
* [`reactive-graph-client entity-instances add-property`↴](#reactive-graph-client-entity-instances-add-property)
* [`reactive-graph-client entity-instances remove-property`↴](#reactive-graph-client-entity-instances-remove-property)
* [`reactive-graph-client entity-instances list-components`↴](#reactive-graph-client-entity-instances-list-components)
* [`reactive-graph-client entity-instances add-component`↴](#reactive-graph-client-entity-instances-add-component)
* [`reactive-graph-client entity-instances remove-component`↴](#reactive-graph-client-entity-instances-remove-component)
* [`reactive-graph-client entity-instances create`↴](#reactive-graph-client-entity-instances-create)
* [`reactive-graph-client entity-instances delete`↴](#reactive-graph-client-entity-instances-delete)
* [`reactive-graph-client entity-instances json-schema`↴](#reactive-graph-client-entity-instances-json-schema)
* [`reactive-graph-client relation-instances`↴](#reactive-graph-client-relation-instances)
* [`reactive-graph-client relation-instances list`↴](#reactive-graph-client-relation-instances-list)
* [`reactive-graph-client relation-instances get`↴](#reactive-graph-client-relation-instances-get)
* [`reactive-graph-client relation-instances list-properties`↴](#reactive-graph-client-relation-instances-list-properties)
* [`reactive-graph-client relation-instances get-property`↴](#reactive-graph-client-relation-instances-get-property)
* [`reactive-graph-client relation-instances set-property`↴](#reactive-graph-client-relation-instances-set-property)
* [`reactive-graph-client relation-instances add-property`↴](#reactive-graph-client-relation-instances-add-property)
* [`reactive-graph-client relation-instances remove-property`↴](#reactive-graph-client-relation-instances-remove-property)
* [`reactive-graph-client relation-instances list-components`↴](#reactive-graph-client-relation-instances-list-components)
* [`reactive-graph-client relation-instances add-component`↴](#reactive-graph-client-relation-instances-add-component)
* [`reactive-graph-client relation-instances remove-component`↴](#reactive-graph-client-relation-instances-remove-component)
* [`reactive-graph-client relation-instances create`↴](#reactive-graph-client-relation-instances-create)
* [`reactive-graph-client relation-instances delete`↴](#reactive-graph-client-relation-instances-delete)
* [`reactive-graph-client relation-instances json-schema`↴](#reactive-graph-client-relation-instances-json-schema)
* [`reactive-graph-client flow-instances`↴](#reactive-graph-client-flow-instances)
* [`reactive-graph-client flow-instances list`↴](#reactive-graph-client-flow-instances-list)
* [`reactive-graph-client flow-instances get`↴](#reactive-graph-client-flow-instances-get)
* [`reactive-graph-client flow-instances get-by-label`↴](#reactive-graph-client-flow-instances-get-by-label)
* [`reactive-graph-client flow-instances create-from-type`↴](#reactive-graph-client-flow-instances-create-from-type)
* [`reactive-graph-client flow-instances delete`↴](#reactive-graph-client-flow-instances-delete)
* [`reactive-graph-client flow-instances json-schema`↴](#reactive-graph-client-flow-instances-json-schema)
* [`reactive-graph-client introspection`↴](#reactive-graph-client-introspection)
* [`reactive-graph-client introspection reactive-graph`↴](#reactive-graph-client-introspection-reactive-graph)
* [`reactive-graph-client introspection dynamic-graph`↴](#reactive-graph-client-introspection-dynamic-graph)
* [`reactive-graph-client introspection reactive-graph-runtime`↴](#reactive-graph-client-introspection-reactive-graph-runtime)
* [`reactive-graph-client introspection reactive-graph-plugins`↴](#reactive-graph-client-introspection-reactive-graph-plugins)

## `reactive-graph-client`

Reactive Graph is a reactive runtime based on a graph database, empowering everyone to build reliable and efficient software.

**Usage:** `reactive-graph-client [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `shell-completions` — Prints or installs Shell completions
* `man-pages` — Prints or installs man pages
* `print-markdown-help` — Prints the markdown help to stdout
* `info` — Prints info about this binary
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

###### **Options:**

* `--client-hostname <CLIENT_HOSTNAME>` — The hostname to connect to
* `--client-port <CLIENT_PORT>` — The port to connect to
* `--client-secure <CLIENT_SECURE>` — If true, connects via HTTPS

  Possible values: `true`, `false`

* `--endpoint-graphql <ENDPOINT_GRAPHQL>` — The endpoint to use
* `--endpoint-dynamic-graph <ENDPOINT_DYNAMIC_GRAPH>` — The endpoint to use
* `--endpoint-runtime <ENDPOINT_RUNTIME>` — The endpoint to use
* `--endpoint-plugins <ENDPOINT_PLUGINS>` — The endpoint to use
* `--bearer <BEARER>` — The authentication token

## `reactive-graph-client shell-completions`

Prints or installs Shell completions

**Usage:** `reactive-graph-client shell-completions <COMMAND>`

###### **Subcommands:**

* `print` — Prints the shell completions to stdout
* `install` — Installs the shell completions

## `reactive-graph-client shell-completions print`

Prints the shell completions to stdout

**Usage:** `reactive-graph-client shell-completions print <SHELL>`

###### **Arguments:**

* `<SHELL>` — The shell

  Possible values: `bash`, `elvish`, `fish`, `powershell`, `zsh`

## `reactive-graph-client shell-completions install`

Installs the shell completions

**Usage:** `reactive-graph-client shell-completions install <SHELL>`

###### **Arguments:**

* `<SHELL>` — The shell

  Possible values: `bash`, `elvish`, `fish`, `powershell`, `zsh`

## `reactive-graph-client man-pages`

Prints or installs man pages

**Usage:** `reactive-graph-client man-pages <COMMAND>`

###### **Subcommands:**

* `print` — Prints the man pages to stdout
* `install` — Installs the man pages

## `reactive-graph-client man-pages print`

Prints the man pages to stdout

**Usage:** `reactive-graph-client man-pages print`

## `reactive-graph-client man-pages install`

Installs the man pages

**Usage:** `reactive-graph-client man-pages install`

## `reactive-graph-client print-markdown-help`

Prints the markdown help to stdout

**Usage:** `reactive-graph-client print-markdown-help`

## `reactive-graph-client info`

Prints info about this binary

**Usage:** `reactive-graph-client info [OPTIONS]`

###### **Options:**

* `--output-format <OUTPUT_FORMAT>` — The output format

  Possible values: `table`, `html-table`, `markdown-table`, `count`, `json`, `json5`, `toml`

## `reactive-graph-client execute-command`

Executes a command on the client

**Usage:** `reactive-graph-client execute-command <COMMAND_NAME> [COMMAND_ARGUMENTS]...`

###### **Arguments:**

* `<COMMAND_NAME>` — The command name
* `<COMMAND_ARGUMENTS>` — The command arguments

## `reactive-graph-client instance-info`

Prints information about the instance

**Usage:** `reactive-graph-client instance-info <COMMAND>`

###### **Subcommands:**

* `get` — Get instance information

## `reactive-graph-client instance-info get`

Get instance information

**Usage:** `reactive-graph-client instance-info get`

## `reactive-graph-client plugins`

Manage plugins

**Usage:** `reactive-graph-client plugins <COMMAND>`

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

## `reactive-graph-client plugins list`

Lists all plugins

**Usage:** `reactive-graph-client plugins list`

## `reactive-graph-client plugins search`

Search for plugins by name, state or stem

**Usage:** `reactive-graph-client plugins search [OPTIONS]`

###### **Options:**

* `--name <NAME>` — The plugin name
* `--state <STATE>` — The plugin state
* `--stem <STEM>` — The plugin file stem

## `reactive-graph-client plugins get`

Prints a single plugin

**Usage:** `reactive-graph-client plugins get <NAME>`

###### **Arguments:**

* `<NAME>` — The plugin name

## `reactive-graph-client plugins dependencies`

Depends on

**Usage:** `reactive-graph-client plugins dependencies <NAME>`

###### **Arguments:**

* `<NAME>` — The plugin name

## `reactive-graph-client plugins dependents`

Dependent plugins

**Usage:** `reactive-graph-client plugins dependents <NAME>`

###### **Arguments:**

* `<NAME>` — The plugin name

## `reactive-graph-client plugins start`

Starts a plugin

**Usage:** `reactive-graph-client plugins start <NAME>`

###### **Arguments:**

* `<NAME>` — The plugin name

## `reactive-graph-client plugins stop`

Stops a plugin

**Usage:** `reactive-graph-client plugins stop <NAME>`

###### **Arguments:**

* `<NAME>` — The plugin name

## `reactive-graph-client plugins restart`

Restarts a plugin

**Usage:** `reactive-graph-client plugins restart <NAME>`

###### **Arguments:**

* `<NAME>` — The plugin name

## `reactive-graph-client plugins uninstall`

Uninstall a plugin

**Usage:** `reactive-graph-client plugins uninstall <NAME>`

###### **Arguments:**

* `<NAME>` — The plugin name

## `reactive-graph-client remotes`

Manage remotes

**Usage:** `reactive-graph-client remotes <COMMAND>`

###### **Subcommands:**

* `list` — Lists the remotes
* `add` — Adds a remote
* `remove` — Removes a remote
* `remove-all` — Removes all remotes
* `update` — Updates a remote
* `update-all` — Updates all remotes
* `fetch-remotes-from-remote` — Fetches the remotes from the given remote
* `fetch-remotes-from-all-remotes` — Fetches all remotes from all remotes

## `reactive-graph-client remotes list`

Lists the remotes

**Usage:** `reactive-graph-client remotes list`

## `reactive-graph-client remotes add`

Adds a remote

**Usage:** `reactive-graph-client remotes add [OPTIONS] --hostname <HOSTNAME>`

###### **Options:**

* `--hostname <HOSTNAME>` — The hostname
* `--port <PORT>` — The port
* `--secure <SECURE>` — The protocol

  Possible values: `true`, `false`

## `reactive-graph-client remotes remove`

Removes a remote

**Usage:** `reactive-graph-client remotes remove [OPTIONS] --hostname <HOSTNAME>`

###### **Options:**

* `--hostname <HOSTNAME>` — The hostname
* `--port <PORT>` — The port
* `--secure <SECURE>` — The protocol

  Possible values: `true`, `false`

## `reactive-graph-client remotes remove-all`

Removes all remotes

**Usage:** `reactive-graph-client remotes remove-all`

## `reactive-graph-client remotes update`

Updates a remote

**Usage:** `reactive-graph-client remotes update [OPTIONS] --hostname <HOSTNAME>`

###### **Options:**

* `--hostname <HOSTNAME>` — The hostname
* `--port <PORT>` — The port
* `--secure <SECURE>` — The protocol

  Possible values: `true`, `false`

## `reactive-graph-client remotes update-all`

Updates all remotes

**Usage:** `reactive-graph-client remotes update-all`

## `reactive-graph-client remotes fetch-remotes-from-remote`

Fetches the remotes from the given remote

**Usage:** `reactive-graph-client remotes fetch-remotes-from-remote [OPTIONS] --hostname <HOSTNAME>`

###### **Options:**

* `--hostname <HOSTNAME>` — The hostname
* `--port <PORT>` — The port
* `--secure <SECURE>` — The protocol

  Possible values: `true`, `false`

## `reactive-graph-client remotes fetch-remotes-from-all-remotes`

Fetches all remotes from all remotes

**Usage:** `reactive-graph-client remotes fetch-remotes-from-all-remotes`

## `reactive-graph-client shutdown`

Shutdown the runtime

**Usage:** `reactive-graph-client shutdown`

## `reactive-graph-client components`

Manage components

**Usage:** `reactive-graph-client components [OPTIONS] <COMMAND>`

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
* `json-schema` — Prints the JSON Schema of components

###### **Options:**

* `-o`, `--output-format <OUTPUT_FORMAT>`

  Possible values: `table`, `html-table`, `markdown-table`, `count`, `json`, `json5`, `toml`

## `reactive-graph-client components list`

List all components

**Usage:** `reactive-graph-client components list`

## `reactive-graph-client components get`

Prints a single component

**Usage:** `reactive-graph-client components get <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name

## `reactive-graph-client components list-properties`

List the properties of a component

**Usage:** `reactive-graph-client components list-properties <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name

## `reactive-graph-client components list-extensions`

List the extensions of a component

**Usage:** `reactive-graph-client components list-extensions <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name

## `reactive-graph-client components create`

Creates a new component

**Usage:** `reactive-graph-client components create <NAMESPACE> <NAME> [DESCRIPTION]`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name
* `<DESCRIPTION>` — The component description

## `reactive-graph-client components delete`

Deletes a component

**Usage:** `reactive-graph-client components delete <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name

## `reactive-graph-client components add-property`

Adds a property to a component

**Usage:** `reactive-graph-client components add-property <NAMESPACE> <NAME> <PROPERTY_NAME> <DATA_TYPE> <SOCKET_TYPE> <MUTABILITY> [DESCRIPTION]`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name
* `<PROPERTY_NAME>` — The name of the property
* `<DATA_TYPE>` — The data type of the property
* `<SOCKET_TYPE>` — The socket type of the property
* `<MUTABILITY>` — If the property is mutable or not
* `<DESCRIPTION>` — Description of the property

## `reactive-graph-client components remove-property`

Removes a property from a component

**Usage:** `reactive-graph-client components remove-property <NAMESPACE> <NAME> <PROPERTY_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name
* `<PROPERTY_NAME>` — The name of the property

## `reactive-graph-client components add-extension`

Adds an extension to a component

**Usage:** `reactive-graph-client components add-extension <NAMESPACE> <NAME> <EXTENSION_NAMESPACE> <EXTENSION_NAME> <DESCRIPTION> <EXTENSION>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name
* `<EXTENSION_NAMESPACE>` — The extension namespace
* `<EXTENSION_NAME>` — The extension name
* `<DESCRIPTION>` — Textual description of the extension
* `<EXTENSION>` — The extension as JSON representation

## `reactive-graph-client components remove-extension`

Removes an extension from a component

**Usage:** `reactive-graph-client components remove-extension <NAMESPACE> <NAME> <EXTENSION_NAMESPACE> <EXTENSION_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name
* `<EXTENSION_NAMESPACE>` — The extension namespace
* `<EXTENSION_NAME>` — The extension name

## `reactive-graph-client components update-description`

Updates the description of a component

**Usage:** `reactive-graph-client components update-description <NAMESPACE> <NAME> <DESCRIPTION>`

###### **Arguments:**

* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name
* `<DESCRIPTION>` — The description to update

## `reactive-graph-client components json-schema`

Prints the JSON Schema of components

**Usage:** `reactive-graph-client components json-schema`

## `reactive-graph-client entity-types`

Manage entity types

**Usage:** `reactive-graph-client entity-types [OPTIONS] <COMMAND>`

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
* `json-schema` — Prints the JSON Schema of entity types

###### **Options:**

* `-o`, `--output-format <OUTPUT_FORMAT>`

  Possible values: `table`, `html-table`, `markdown-table`, `count`, `json`, `json5`, `toml`

## `reactive-graph-client entity-types list`

List all entity types

**Usage:** `reactive-graph-client entity-types list`

## `reactive-graph-client entity-types get`

Prints a single entity type

**Usage:** `reactive-graph-client entity-types get <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name

## `reactive-graph-client entity-types list-properties`

List the properties of an entity type

**Usage:** `reactive-graph-client entity-types list-properties <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name

## `reactive-graph-client entity-types list-extensions`

List the extensions of an entity type

**Usage:** `reactive-graph-client entity-types list-extensions <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name

## `reactive-graph-client entity-types list-components`

List the components of an entity type

**Usage:** `reactive-graph-client entity-types list-components <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name

## `reactive-graph-client entity-types create`

Creates a new entity type

**Usage:** `reactive-graph-client entity-types create <NAMESPACE> <NAME> [DESCRIPTION]`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<DESCRIPTION>` — The entity type description

## `reactive-graph-client entity-types delete`

Deletes a entity type

**Usage:** `reactive-graph-client entity-types delete <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name

## `reactive-graph-client entity-types add-property`

Adds a property to an entity type

**Usage:** `reactive-graph-client entity-types add-property <NAMESPACE> <NAME> <PROPERTY_NAME> <DATA_TYPE> <SOCKET_TYPE> <MUTABILITY> [DESCRIPTION]`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<PROPERTY_NAME>` — The name of the property
* `<DATA_TYPE>` — The data type of the property
* `<SOCKET_TYPE>` — The socket type of the property
* `<MUTABILITY>` — If the property is mutable or not
* `<DESCRIPTION>` — Description of the property

## `reactive-graph-client entity-types remove-property`

Removes a property from an entity type

**Usage:** `reactive-graph-client entity-types remove-property <NAMESPACE> <NAME> <PROPERTY_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<PROPERTY_NAME>` — The name of the property

## `reactive-graph-client entity-types add-extension`

Adds an extension to an entity type

**Usage:** `reactive-graph-client entity-types add-extension <NAMESPACE> <NAME> <EXTENSION_NAMESPACE> <EXTENSION_NAME> <DESCRIPTION> <EXTENSION>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<EXTENSION_NAMESPACE>` — The extension namespace
* `<EXTENSION_NAME>` — The extension name
* `<DESCRIPTION>` — Textual description of the extension
* `<EXTENSION>` — The extension as JSON representation

## `reactive-graph-client entity-types remove-extension`

Removes an extension from an entity type

**Usage:** `reactive-graph-client entity-types remove-extension <NAMESPACE> <NAME> <EXTENSION_NAMESPACE> <EXTENSION_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<EXTENSION_NAMESPACE>` — The extension namespace
* `<EXTENSION_NAME>` — The extension name

## `reactive-graph-client entity-types add-component`

Adds a component to an entity type

**Usage:** `reactive-graph-client entity-types add-component <NAMESPACE> <NAME> <COMPONENT_NAMESPACE> <COMPONENT_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<COMPONENT_NAMESPACE>` — The component namespace
* `<COMPONENT_NAME>` — The component name

## `reactive-graph-client entity-types remove-component`

Removes a component from an entity type

**Usage:** `reactive-graph-client entity-types remove-component <NAMESPACE> <NAME> <COMPONENT_NAMESPACE> <COMPONENT_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<COMPONENT_NAMESPACE>` — The component namespace
* `<COMPONENT_NAME>` — The component name

## `reactive-graph-client entity-types update-description`

Updates the description of an entity type

**Usage:** `reactive-graph-client entity-types update-description <NAMESPACE> <NAME> <DESCRIPTION>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name
* `<DESCRIPTION>` — The description to update

## `reactive-graph-client entity-types json-schema`

Prints the JSON Schema of entity types

**Usage:** `reactive-graph-client entity-types json-schema`

## `reactive-graph-client relation-types`

Manage entity types

**Usage:** `reactive-graph-client relation-types [OPTIONS] <COMMAND>`

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
* `json-schema` — Prints the JSON Schema of relation types

###### **Options:**

* `-o`, `--output-format <OUTPUT_FORMAT>`

  Possible values: `table`, `html-table`, `markdown-table`, `count`, `json`, `json5`, `toml`

## `reactive-graph-client relation-types list`

List all relation types

**Usage:** `reactive-graph-client relation-types list`

## `reactive-graph-client relation-types get`

Prints a single relation type

**Usage:** `reactive-graph-client relation-types get <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name

## `reactive-graph-client relation-types list-properties`

List the properties of an relation type

**Usage:** `reactive-graph-client relation-types list-properties <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name

## `reactive-graph-client relation-types list-extensions`

List the extensions of an relation type

**Usage:** `reactive-graph-client relation-types list-extensions <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name

## `reactive-graph-client relation-types list-components`

List the components of an relation type

**Usage:** `reactive-graph-client relation-types list-components <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name

## `reactive-graph-client relation-types create`

Creates a new relation type

**Usage:**
`reactive-graph-client relation-types create <OUTBOUND_TYPE_NAMESPACE> <OUTBOUND_TYPE_NAME> <NAMESPACE> <NAME> <INBOUND_TYPE_NAMESPACE> <INBOUND_TYPE_NAME> [DESCRIPTION]`

###### **Arguments:**

* `<OUTBOUND_TYPE_NAMESPACE>` — The outbound entity type namespace
* `<OUTBOUND_TYPE_NAME>` — The outbound entity type name
* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<INBOUND_TYPE_NAMESPACE>` — The inbound entity type namespace
* `<INBOUND_TYPE_NAME>` — The inbound entity type name
* `<DESCRIPTION>` — The relation type description

## `reactive-graph-client relation-types delete`

Deletes a relation type

**Usage:** `reactive-graph-client relation-types delete <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name

## `reactive-graph-client relation-types add-property`

Adds a property to a relation type

**Usage:** `reactive-graph-client relation-types add-property <NAMESPACE> <NAME> <PROPERTY_NAME> <DATA_TYPE> <SOCKET_TYPE> <MUTABILITY> [DESCRIPTION]`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<PROPERTY_NAME>` — The name of the property
* `<DATA_TYPE>` — The data type of the property
* `<SOCKET_TYPE>` — The socket type of the property
* `<MUTABILITY>` — If the property is mutable or not
* `<DESCRIPTION>` — Description of the property

## `reactive-graph-client relation-types remove-property`

Removes a property from a relation type

**Usage:** `reactive-graph-client relation-types remove-property <NAMESPACE> <NAME> <PROPERTY_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<PROPERTY_NAME>` — The name of the property

## `reactive-graph-client relation-types add-extension`

Adds an extension to a relation type

**Usage:** `reactive-graph-client relation-types add-extension <NAMESPACE> <NAME> <EXTENSION_NAMESPACE> <EXTENSION_NAME> <DESCRIPTION> <EXTENSION>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<EXTENSION_NAMESPACE>` — The extension namespace
* `<EXTENSION_NAME>` — The extension name
* `<DESCRIPTION>` — Textual description of the extension
* `<EXTENSION>` — The extension as JSON representation

## `reactive-graph-client relation-types remove-extension`

Removes an extension from a relation type

**Usage:** `reactive-graph-client relation-types remove-extension <NAMESPACE> <NAME> <EXTENSION_NAMESPACE> <EXTENSION_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<EXTENSION_NAMESPACE>` — The extension namespace
* `<EXTENSION_NAME>` — The extension name

## `reactive-graph-client relation-types add-component`

Adds a component to a relation type

**Usage:** `reactive-graph-client relation-types add-component <NAMESPACE> <NAME> <COMPONENT_NAMESPACE> <COMPONENT_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<COMPONENT_NAMESPACE>` — The component namespace
* `<COMPONENT_NAME>` — The component name

## `reactive-graph-client relation-types remove-component`

Removes a component from a relation type

**Usage:** `reactive-graph-client relation-types remove-component <NAMESPACE> <NAME> <COMPONENT_NAMESPACE> <COMPONENT_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<COMPONENT_NAMESPACE>` — The component namespace
* `<COMPONENT_NAME>` — The component name

## `reactive-graph-client relation-types update-description`

Updates the description of a relation type

**Usage:** `reactive-graph-client relation-types update-description <NAMESPACE> <NAME> <DESCRIPTION>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<DESCRIPTION>` — The description to update

## `reactive-graph-client relation-types json-schema`

Prints the JSON Schema of relation types

**Usage:** `reactive-graph-client relation-types json-schema`

## `reactive-graph-client flow-types`

Manage entity types

**Usage:** `reactive-graph-client flow-types [OPTIONS] <COMMAND>`

###### **Subcommands:**

* `list` — List all flow types
* `get` — Prints a single flow type
* `list-variables` — List the variables of a flow type
* `list-extensions` — List the extensions of a flow type
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

## `reactive-graph-client flow-types list`

List all flow types

**Usage:** `reactive-graph-client flow-types list`

## `reactive-graph-client flow-types get`

Prints a single flow type

**Usage:** `reactive-graph-client flow-types get <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name

## `reactive-graph-client flow-types list-variables`

List the variables of a flow type

**Usage:** `reactive-graph-client flow-types list-variables <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name

## `reactive-graph-client flow-types list-extensions`

List the extensions of a flow type

**Usage:** `reactive-graph-client flow-types list-extensions <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name

## `reactive-graph-client flow-types create`

Creates a new flow type

**Usage:**
`reactive-graph-client flow-types create <NAMESPACE> <NAME> <ENTITY_TYPE_NAMESPACE> <ENTITY_TYPE_NAME> <WRAPPER_ENTITY_INSTANCE_ID> [DESCRIPTION] [WRAPPER_ENTITY_INSTANCE_DESCRIPTION]`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name
* `<ENTITY_TYPE_NAMESPACE>` — The namespace of the entity type of the wrapper entity instance
* `<ENTITY_TYPE_NAME>` — The type name of the entity type of the wrapper entity instance
* `<WRAPPER_ENTITY_INSTANCE_ID>` — The id of the wrapper entity instance
* `<DESCRIPTION>` — The flow type description
* `<WRAPPER_ENTITY_INSTANCE_DESCRIPTION>` — The description of the wrapper entity instance

## `reactive-graph-client flow-types delete`

Deletes a flow type

**Usage:** `reactive-graph-client flow-types delete <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name

## `reactive-graph-client flow-types add-variable`

Adds a property to a flow type

**Usage:** `reactive-graph-client flow-types add-variable <NAMESPACE> <NAME> <PROPERTY_NAME> <DATA_TYPE> <SOCKET_TYPE> <MUTABILITY> [DESCRIPTION]`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name
* `<PROPERTY_NAME>` — The name of the property
* `<DATA_TYPE>` — The data type of the property
* `<SOCKET_TYPE>` — The socket type of the property
* `<MUTABILITY>` — If the property is mutable or not
* `<DESCRIPTION>` — Description of the property

## `reactive-graph-client flow-types remove-variable`

Removes a property from a flow type

**Usage:** `reactive-graph-client flow-types remove-variable <NAMESPACE> <NAME> <VARIABLE_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name
* `<VARIABLE_NAME>` — The name of the variable

## `reactive-graph-client flow-types add-extension`

Adds an extension to a flow type

**Usage:** `reactive-graph-client flow-types add-extension <NAMESPACE> <NAME> <EXTENSION_NAMESPACE> <EXTENSION_NAME> <DESCRIPTION> <EXTENSION>`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name
* `<EXTENSION_NAMESPACE>` — The extension namespace
* `<EXTENSION_NAME>` — The extension name
* `<DESCRIPTION>` — Textual description of the extension
* `<EXTENSION>` — The extension as JSON representation

## `reactive-graph-client flow-types remove-extension`

Removes an extension from a flow type

**Usage:** `reactive-graph-client flow-types remove-extension <NAMESPACE> <NAME> <EXTENSION_NAMESPACE> <EXTENSION_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name
* `<EXTENSION_NAMESPACE>` — The extension namespace
* `<EXTENSION_NAME>` — The extension name

## `reactive-graph-client flow-types update-description`

Updates the description of a flow type

**Usage:** `reactive-graph-client flow-types update-description <NAMESPACE> <NAME> <DESCRIPTION>`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name
* `<DESCRIPTION>` — The description to update

## `reactive-graph-client flow-types add-entity-instance`

Adds a new entity instance to a flow type

**Usage:** `reactive-graph-client flow-types add-entity-instance [OPTIONS] <NAMESPACE> <NAME> <ENTITY_TYPE_NAMESPACE> <ENTITY_TYPE_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name
* `<ENTITY_TYPE_NAMESPACE>` — The entity type namespace
* `<ENTITY_TYPE_NAME>` — The entity type name

###### **Options:**

* `-i`, `--id <ID>` — The entity instance id
* `-d`, `--description <DESCRIPTION>` — The entity instance description
* `-p`, `--properties <PROPERTIES>` — The entity instance properties

## `reactive-graph-client flow-types remove-entity-instance`

Removes an entity instance to a flow type

**Usage:** `reactive-graph-client flow-types remove-entity-instance <NAMESPACE> <NAME> <ID>`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name
* `<ID>` — The entity instance to remove from the flow type

## `reactive-graph-client flow-types json-schema`

Prints the JSON Schema of flow types

**Usage:** `reactive-graph-client flow-types json-schema`

## `reactive-graph-client entity-instances`

Manage entity instances

**Usage:** `reactive-graph-client entity-instances [OPTIONS] <COMMAND>`

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

## `reactive-graph-client entity-instances list`

List all entity instances

**Usage:** `reactive-graph-client entity-instances list [OPTIONS]`

###### **Options:**

* `--namespace <NAMESPACE>` — The entity type namespace
* `-n`, `--name <NAME>` — The entity type name
* `-i`, `--id <ID>` — The id of the entity instance
* `-l`, `--label <LABEL>` — The label of the entity instance
* `-p`, `--properties <PROPERTIES>` — The properties to search for
* `-c`, `--components <COMPONENTS>` — The components to search for

## `reactive-graph-client entity-instances get`

Prints a single entity instance

**Usage:** `reactive-graph-client entity-instances get <ID>`

###### **Arguments:**

* `<ID>` — The id of the entity instance

## `reactive-graph-client entity-instances get-by-label`

Prints a single entity instance

**Usage:** `reactive-graph-client entity-instances get-by-label <LABEL>`

###### **Arguments:**

* `<LABEL>` — The label of the reactive instance

## `reactive-graph-client entity-instances list-properties`

Lists the properties of an entity instance

**Usage:** `reactive-graph-client entity-instances list-properties <ID>`

###### **Arguments:**

* `<ID>` — The id of the entity instance

## `reactive-graph-client entity-instances get-property`

Prints the value of a property of an entity instance

**Usage:** `reactive-graph-client entity-instances get-property <ID> <PROPERTY_NAME>`

###### **Arguments:**

* `<ID>` — The id of the entity instance
* `<PROPERTY_NAME>` — The name of the property

## `reactive-graph-client entity-instances set-property`

Sets the value of a property of an entity instance

**Usage:** `reactive-graph-client entity-instances set-property <ID> <NAME> <VALUE>`

###### **Arguments:**

* `<ID>` — The id of the reactive instance
* `<NAME>` — The name of the property
* `<VALUE>` — The new JSON value of the property.

  'true' is boolean true, '"true"' is the string "true"

## `reactive-graph-client entity-instances add-property`

Adds a new property to an entity instance

**Usage:** `reactive-graph-client entity-instances add-property <ID> <PROPERTY_NAME> <DATA_TYPE> <SOCKET_TYPE> <MUTABILITY> [DESCRIPTION]`

###### **Arguments:**

* `<ID>` — The id of the reactive instance
* `<PROPERTY_NAME>` — The name of the property
* `<DATA_TYPE>` — The data type of the property
* `<SOCKET_TYPE>` — The socket type of the property
* `<MUTABILITY>` — If the property is mutable or not
* `<DESCRIPTION>` — Description of the property

## `reactive-graph-client entity-instances remove-property`

Removes a property from an entity instance

**Usage:** `reactive-graph-client entity-instances remove-property <ID> <PROPERTY_NAME>`

###### **Arguments:**

* `<ID>` — The id of the entity instance
* `<PROPERTY_NAME>` — The name of the property

## `reactive-graph-client entity-instances list-components`

Lists the components of an entity instance

**Usage:** `reactive-graph-client entity-instances list-components <ID>`

###### **Arguments:**

* `<ID>` — The id of the entity instance

## `reactive-graph-client entity-instances add-component`

Adds a component to an entity instance

**Usage:** `reactive-graph-client entity-instances add-component <ID> <NAMESPACE> <NAME>`

###### **Arguments:**

* `<ID>` — The id of the reactive instance
* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name

## `reactive-graph-client entity-instances remove-component`

Removes a component from an entity instance

**Usage:** `reactive-graph-client entity-instances remove-component <ID> <NAMESPACE> <NAME>`

###### **Arguments:**

* `<ID>` — The id of the reactive instance
* `<NAMESPACE>` — The component namespace
* `<NAME>` — The component name

## `reactive-graph-client entity-instances create`

Creates a new entity type

**Usage:** `reactive-graph-client entity-instances create [OPTIONS] <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The entity type namespace
* `<NAME>` — The entity type name

###### **Options:**

* `-i`, `--id <ID>` — The entity instance id
* `-d`, `--description <DESCRIPTION>` — The entity instance description
* `-p`, `--properties <PROPERTIES>` — The entity instance properties

## `reactive-graph-client entity-instances delete`

CLI argument which identifies an entity instance by its id

**Usage:** `reactive-graph-client entity-instances delete <ID>`

###### **Arguments:**

* `<ID>` — The id of the entity instance

## `reactive-graph-client entity-instances json-schema`

Prints the JSON Schema of entity instances

**Usage:** `reactive-graph-client entity-instances json-schema`

## `reactive-graph-client relation-instances`

Manage relation instances

**Usage:** `reactive-graph-client relation-instances [OPTIONS] <COMMAND>`

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

## `reactive-graph-client relation-instances list`

List all relation instances

**Usage:** `reactive-graph-client relation-instances list [OPTIONS]`

###### **Options:**

* `--outbound-id <OUTBOUND_ID>` — The id of the outbound entity instance
* `--namespace <NAMESPACE>` — The relation type namespace
* `-n`, `--name <NAME>` — The relation type name
* `-i`, `--inbound-id <INBOUND_ID>` — The id of the inbound entity instance
* `-p`, `--properties <PROPERTIES>` — The properties to search for
* `-c`, `--components <COMPONENTS>` — The components to search for

## `reactive-graph-client relation-instances get`

Prints a single relation instance

**Usage:** `reactive-graph-client relation-instances get --outbound-id <OUTBOUND_ID> --inbound-id <INBOUND_ID> <NAMESPACE> <NAME> <INSTANCE_ID>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<INSTANCE_ID>` — The instance id

###### **Options:**

* `--outbound-id <OUTBOUND_ID>` — The id of the outbound entity instance
* `-i`, `--inbound-id <INBOUND_ID>` — The id of the inbound entity instance

## `reactive-graph-client relation-instances list-properties`

Lists the properties of a relation instance

**Usage:** `reactive-graph-client relation-instances list-properties --outbound-id <OUTBOUND_ID> --inbound-id <INBOUND_ID> <NAMESPACE> <NAME> <INSTANCE_ID>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<INSTANCE_ID>` — The instance id

###### **Options:**

* `--outbound-id <OUTBOUND_ID>` — The id of the outbound entity instance
* `-i`, `--inbound-id <INBOUND_ID>` — The id of the inbound entity instance

## `reactive-graph-client relation-instances get-property`

Prints the value of a property of a relation instance

**Usage:**
`reactive-graph-client relation-instances get-property --outbound-id <OUTBOUND_ID> --inbound-id <INBOUND_ID> <NAMESPACE> <NAME> <INSTANCE_ID> <PROPERTY_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<INSTANCE_ID>` — The instance id
* `<PROPERTY_NAME>` — The name of the property

###### **Options:**

* `--outbound-id <OUTBOUND_ID>` — The id of the outbound entity instance
* `-i`, `--inbound-id <INBOUND_ID>` — The id of the inbound entity instance

## `reactive-graph-client relation-instances set-property`

Sets the value of a property of a relation instance

**Usage:**
`reactive-graph-client relation-instances set-property --outbound-id <OUTBOUND_ID> --inbound-id <INBOUND_ID> <NAMESPACE> <NAME> <INSTANCE_ID> <PROPERTY_NAME> <PROPERTY_VALUE>`

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

## `reactive-graph-client relation-instances add-property`

Adds a new property to a relation instance

**Usage:**
`reactive-graph-client relation-instances add-property --outbound-id <OUTBOUND_ID> --inbound-id <INBOUND_ID> <NAMESPACE> <NAME> <INSTANCE_ID> <PROPERTY_NAME> <DATA_TYPE> <SOCKET_TYPE> <MUTABILITY> [DESCRIPTION]`

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

## `reactive-graph-client relation-instances remove-property`

Removes a property from a relation instance

**Usage:**
`reactive-graph-client relation-instances remove-property --outbound-id <OUTBOUND_ID> --inbound-id <INBOUND_ID> <NAMESPACE> <NAME> <INSTANCE_ID> <PROPERTY_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<INSTANCE_ID>` — The instance id
* `<PROPERTY_NAME>` — The name of the property

###### **Options:**

* `--outbound-id <OUTBOUND_ID>` — The id of the outbound entity instance
* `-i`, `--inbound-id <INBOUND_ID>` — The id of the inbound entity instance

## `reactive-graph-client relation-instances list-components`

Lists the components of a relation instance

**Usage:** `reactive-graph-client relation-instances list-components --outbound-id <OUTBOUND_ID> --inbound-id <INBOUND_ID> <NAMESPACE> <NAME> <INSTANCE_ID>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<INSTANCE_ID>` — The instance id

###### **Options:**

* `--outbound-id <OUTBOUND_ID>` — The id of the outbound entity instance
* `-i`, `--inbound-id <INBOUND_ID>` — The id of the inbound entity instance

## `reactive-graph-client relation-instances add-component`

Adds a component to a relation instance

**Usage:**
`reactive-graph-client relation-instances add-component --outbound-id <OUTBOUND_ID> --inbound-id <INBOUND_ID> <NAMESPACE> <NAME> <INSTANCE_ID> <COMPONENT_NAMESPACE> <COMPONENT_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<INSTANCE_ID>` — The instance id
* `<COMPONENT_NAMESPACE>` — The component namespace
* `<COMPONENT_NAME>` — The component name

###### **Options:**

* `--outbound-id <OUTBOUND_ID>` — The id of the outbound entity instance
* `-i`, `--inbound-id <INBOUND_ID>` — The id of the inbound entity instance

## `reactive-graph-client relation-instances remove-component`

Removes a component from a relation instance

**Usage:**
`reactive-graph-client relation-instances remove-component --outbound-id <OUTBOUND_ID> --inbound-id <INBOUND_ID> <NAMESPACE> <NAME> <INSTANCE_ID> <COMPONENT_NAMESPACE> <COMPONENT_NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<INSTANCE_ID>` — The instance id
* `<COMPONENT_NAMESPACE>` — The component namespace
* `<COMPONENT_NAME>` — The component name

###### **Options:**

* `--outbound-id <OUTBOUND_ID>` — The id of the outbound entity instance
* `-i`, `--inbound-id <INBOUND_ID>` — The id of the inbound entity instance

## `reactive-graph-client relation-instances create`

Creates a new relation type

**Usage:** `reactive-graph-client relation-instances create [OPTIONS] --outbound-id <OUTBOUND_ID> --inbound-id <INBOUND_ID> <NAMESPACE> <NAME> <INSTANCE_ID>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<INSTANCE_ID>` — The instance id

###### **Options:**

* `--outbound-id <OUTBOUND_ID>` — The id of the outbound entity instance
* `-i`, `--inbound-id <INBOUND_ID>` — The id of the inbound entity instance
* `-d`, `--description <DESCRIPTION>` — The relation instance description
* `-p`, `--properties <PROPERTIES>` — The relation instance properties

## `reactive-graph-client relation-instances delete`

CLI argument which identifies an relation instance by its id

**Usage:** `reactive-graph-client relation-instances delete --outbound-id <OUTBOUND_ID> --inbound-id <INBOUND_ID> <NAMESPACE> <NAME> <INSTANCE_ID>`

###### **Arguments:**

* `<NAMESPACE>` — The relation type namespace
* `<NAME>` — The relation type name
* `<INSTANCE_ID>` — The instance id

###### **Options:**

* `--outbound-id <OUTBOUND_ID>` — The id of the outbound entity instance
* `-i`, `--inbound-id <INBOUND_ID>` — The id of the inbound entity instance

## `reactive-graph-client relation-instances json-schema`

Prints the JSON Schema of relation instances

**Usage:** `reactive-graph-client relation-instances json-schema`

## `reactive-graph-client flow-instances`

Manage flow instances

**Usage:** `reactive-graph-client flow-instances [OPTIONS] <COMMAND>`

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

## `reactive-graph-client flow-instances list`

List all flow instances

**Usage:** `reactive-graph-client flow-instances list [OPTIONS]`

###### **Options:**

* `--namespace <NAMESPACE>` — The entity type namespace
* `-n`, `--name <NAME>` — The entity type name
* `-i`, `--id <ID>` — The id of the entity instance
* `-l`, `--label <LABEL>` — The label of the entity instance

## `reactive-graph-client flow-instances get`

Prints a single flow instance

**Usage:** `reactive-graph-client flow-instances get <ID>`

###### **Arguments:**

* `<ID>` — The id of the flow instance

## `reactive-graph-client flow-instances get-by-label`

Prints a single flow instance

**Usage:** `reactive-graph-client flow-instances get-by-label <LABEL>`

###### **Arguments:**

* `<LABEL>` — The label of the reactive instance

## `reactive-graph-client flow-instances create-from-type`

Creates a new flow from the given type

**Usage:** `reactive-graph-client flow-instances create-from-type [OPTIONS] <NAMESPACE> <NAME>`

###### **Arguments:**

* `<NAMESPACE>` — The flow type namespace
* `<NAME>` — The flow type name

###### **Options:**

* `-i`, `--id <ID>` — The id of the flow instance and the wrapper entity instance
* `-v`, `--variables <VARIABLES>` — The entity instance properties
* `-p`, `--properties <PROPERTIES>` — The entity instance properties

## `reactive-graph-client flow-instances delete`

CLI argument which identifies a flow instance by its id

**Usage:** `reactive-graph-client flow-instances delete <ID>`

###### **Arguments:**

* `<ID>` — The id of the flow instance

## `reactive-graph-client flow-instances json-schema`

Prints the JSON Schema of flow instances

**Usage:** `reactive-graph-client flow-instances json-schema`

## `reactive-graph-client introspection`

Execute GraphQL introspection queries

**Usage:** `reactive-graph-client introspection <COMMAND>`

###### **Subcommands:**

* `reactive-graph` — Get the GraphQL schema of the reactive graph
* `dynamic-graph` — Get the GraphQL schema of the dynamic graph
* `reactive-graph-runtime` — Get the GraphQL schema of the reactive graph runtime
* `reactive-graph-plugins` — Get the GraphQL schema of the plugin system of reactive graph

## `reactive-graph-client introspection reactive-graph`

Get the GraphQL schema of the reactive graph

**Usage:** `reactive-graph-client introspection reactive-graph`

## `reactive-graph-client introspection dynamic-graph`

Get the GraphQL schema of the dynamic graph

**Usage:** `reactive-graph-client introspection dynamic-graph`

## `reactive-graph-client introspection reactive-graph-runtime`

Get the GraphQL schema of the reactive graph runtime

**Usage:** `reactive-graph-client introspection reactive-graph-runtime`

## `reactive-graph-client introspection reactive-graph-plugins`

Get the GraphQL schema of the plugin system of reactive graph

**Usage:** `reactive-graph-client introspection reactive-graph-plugins`



<hr/>

<small><i>
This document was generated automatically by
<a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>
