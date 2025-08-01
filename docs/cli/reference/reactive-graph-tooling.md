# Command-Line Help for `reactive-graph-tooling`

This document contains the help content for the `reactive-graph-tooling` command-line program.

**Command Overview:**

* [`reactive-graph-tooling`↴](#reactive-graph-tooling)
* [`reactive-graph-tooling shell-completions`↴](#reactive-graph-tooling-shell-completions)
* [`reactive-graph-tooling shell-completions print`↴](#reactive-graph-tooling-shell-completions-print)
* [`reactive-graph-tooling shell-completions install`↴](#reactive-graph-tooling-shell-completions-install)
* [`reactive-graph-tooling man-pages`↴](#reactive-graph-tooling-man-pages)
* [`reactive-graph-tooling man-pages print`↴](#reactive-graph-tooling-man-pages-print)
* [`reactive-graph-tooling man-pages install`↴](#reactive-graph-tooling-man-pages-install)
* [`reactive-graph-tooling print-markdown-help`↴](#reactive-graph-tooling-print-markdown-help)
* [`reactive-graph-tooling info`↴](#reactive-graph-tooling-info)
* [`reactive-graph-tooling instances`↴](#reactive-graph-tooling-instances)
* [`reactive-graph-tooling instances config`↴](#reactive-graph-tooling-instances-config)
* [`reactive-graph-tooling instances config graphql`↴](#reactive-graph-tooling-instances-config-graphql)
* [`reactive-graph-tooling instances config instance`↴](#reactive-graph-tooling-instances-config-instance)
* [`reactive-graph-tooling instances config plugins`↴](#reactive-graph-tooling-instances-config-plugins)
* [`reactive-graph-tooling instances generate-certificate`↴](#reactive-graph-tooling-instances-generate-certificate)
* [`reactive-graph-tooling instances init`↴](#reactive-graph-tooling-instances-init)
* [`reactive-graph-tooling instances plugins`↴](#reactive-graph-tooling-instances-plugins)
* [`reactive-graph-tooling instances plugins install`↴](#reactive-graph-tooling-instances-plugins-install)
* [`reactive-graph-tooling instances plugins uninstall`↴](#reactive-graph-tooling-instances-plugins-uninstall)
* [`reactive-graph-tooling instances repository`↴](#reactive-graph-tooling-instances-repository)
* [`reactive-graph-tooling instances repository init`↴](#reactive-graph-tooling-instances-repository-init)
* [`reactive-graph-tooling instances repository remove`↴](#reactive-graph-tooling-instances-repository-remove)
* [`reactive-graph-tooling update`↴](#reactive-graph-tooling-update)
* [`reactive-graph-tooling update info`↴](#reactive-graph-tooling-update-info)
* [`reactive-graph-tooling update list`↴](#reactive-graph-tooling-update-list)

## `reactive-graph-tooling`

Reactive Graph is a reactive runtime based on a graph database, empowering everyone to build reliable and efficient software.

**Usage:** `reactive-graph-tooling [COMMAND]`

###### **Subcommands:**

* `shell-completions` — Prints or installs Shell completions
* `man-pages` — Prints or installs man pages
* `print-markdown-help` — Prints the markdown help to stdout
* `info` — Prints info about this binary
* `instances` — Manage instances
* `update` — Update the Reactive Graph binary



## `reactive-graph-tooling shell-completions`

Prints or installs Shell completions

**Usage:** `reactive-graph-tooling shell-completions <COMMAND>`

###### **Subcommands:**

* `print` — Prints the shell completions to stdout
* `install` — Installs the shell completions



## `reactive-graph-tooling shell-completions print`

Prints the shell completions to stdout

**Usage:** `reactive-graph-tooling shell-completions print <SHELL>`

###### **Arguments:**

* `<SHELL>` — The shell

  Possible values: `bash`, `elvish`, `fish`, `powershell`, `zsh`




## `reactive-graph-tooling shell-completions install`

Installs the shell completions

**Usage:** `reactive-graph-tooling shell-completions install <SHELL>`

###### **Arguments:**

* `<SHELL>` — The shell

  Possible values: `bash`, `elvish`, `fish`, `powershell`, `zsh`




## `reactive-graph-tooling man-pages`

Prints or installs man pages

**Usage:** `reactive-graph-tooling man-pages <COMMAND>`

###### **Subcommands:**

* `print` — Prints the man pages to stdout
* `install` — Installs the man pages



## `reactive-graph-tooling man-pages print`

Prints the man pages to stdout

**Usage:** `reactive-graph-tooling man-pages print`



## `reactive-graph-tooling man-pages install`

Installs the man pages

**Usage:** `reactive-graph-tooling man-pages install`



## `reactive-graph-tooling print-markdown-help`

Prints the markdown help to stdout

**Usage:** `reactive-graph-tooling print-markdown-help`



## `reactive-graph-tooling info`

Prints info about this binary

**Usage:** `reactive-graph-tooling info [OPTIONS]`

###### **Options:**

* `--output-format <OUTPUT_FORMAT>` — The output format

  Possible values: `table`, `html-table`, `markdown-table`, `count`, `json`, `json5`, `toml`




## `reactive-graph-tooling instances`

Manage instances

**Usage:** `reactive-graph-tooling instances [WORKING_DIRECTORY] <COMMAND>`

###### **Subcommands:**

* `config` — Configures a local instance,
* `generate-certificate` — Generates certificate of a local instance
* `init` — Initialize the filesystem structure of a new local instance
* `plugins` — Manage the plugins of a local instance
* `repository` — Manage the repositories of a local instance

###### **Arguments:**

* `<WORKING_DIRECTORY>` — The working directory of the instance. Defaults to the current directory



## `reactive-graph-tooling instances config`

Configures a local instance,

**Usage:** `reactive-graph-tooling instances config <COMMAND>`

###### **Subcommands:**

* `graphql` — Configures the GraphQL server
* `instance` — Configures the instance
* `plugins` — Configures the instance



## `reactive-graph-tooling instances config graphql`

Configures the GraphQL server

**Usage:** `reactive-graph-tooling instances config graphql [OPTIONS]`

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



## `reactive-graph-tooling instances config instance`

Configures the instance

**Usage:** `reactive-graph-tooling instances config instance [OPTIONS]`

###### **Options:**

* `-n`, `--instance-name <NAME>` — The name of the instance
* `-d`, `--instance-description <DESCRIPTION>` — The description of the instance



## `reactive-graph-tooling instances config plugins`

Configures the instance

**Usage:** `reactive-graph-tooling instances config plugins [OPTIONS]`

###### **Options:**

* `-x`, `--disable-all-plugins <DISABLE_ALL_PLUGINS>` — If true, all plugins will be disabled

  Possible values: `true`, `false`

* `-p`, `--disabled-plugins <DISABLED_PLUGINS>` — The list of plugins to disable
* `-P`, `--enabled-plugins <ENABLED_PLUGINS>` — The list of plugins to enable
* `--disable-hot-deploy <DISABLE_HOT_DEPLOY>` — If true, hot deployment will be disabled

  Possible values: `true`, `false`

* `--hot-deploy-location <HOT_DEPLOY_LOCATION>` — The folder which is watched for hot deployment
* `--install-location <INSTALL_LOCATION>` — The folder which plugins are installed permanently



## `reactive-graph-tooling instances generate-certificate`

Generates certificate of a local instance

**Usage:** `reactive-graph-tooling instances generate-certificate [COUNTRY_NAME] [ORGANIZATION_NAME] [COMMON_NAME]`

###### **Arguments:**

* `<COUNTRY_NAME>` — Country name
* `<ORGANIZATION_NAME>` — Organization name
* `<COMMON_NAME>` — Common name



## `reactive-graph-tooling instances init`

Initialize the filesystem structure of a new local instance

**Usage:** `reactive-graph-tooling instances init [OPTIONS] [COUNTRY_NAME] [ORGANIZATION_NAME] [COMMON_NAME]`

###### **Arguments:**

* `<COUNTRY_NAME>` — Country name
* `<ORGANIZATION_NAME>` — Organization name
* `<COMMON_NAME>` — Common name

###### **Options:**

* `--uid <UID>` — The numeric user id of the owner user
* `--gid <GID>` — The numeric group id of the owner group



## `reactive-graph-tooling instances plugins`

Manage the plugins of a local instance

**Usage:** `reactive-graph-tooling instances plugins <COMMAND>`

###### **Subcommands:**

* `install` — Installs a plugin
* `uninstall` — Uninstalls a plugin



## `reactive-graph-tooling instances plugins install`

Installs a plugin

**Usage:** `reactive-graph-tooling instances plugins install <PLUGIN_NAME>`

###### **Arguments:**

* `<PLUGIN_NAME>` — The name of the plugin



## `reactive-graph-tooling instances plugins uninstall`

Uninstalls a plugin

**Usage:** `reactive-graph-tooling instances plugins uninstall <PLUGIN_NAME>`

###### **Arguments:**

* `<PLUGIN_NAME>` — The name of the plugin



## `reactive-graph-tooling instances repository`

Manage the repositories of a local instance

**Usage:** `reactive-graph-tooling instances repository <COMMAND>`

###### **Subcommands:**

* `init` — Initializes a new local repository in a local instance
* `remove` — Removes a local repository



## `reactive-graph-tooling instances repository init`

Initializes a new local repository in a local instance

**Usage:** `reactive-graph-tooling instances repository init [OPTIONS] <LOCAL_NAME> [URL]`

###### **Arguments:**

* `<LOCAL_NAME>` — The local name of the repository
* `<URL>` — The remote URL of the repository

###### **Options:**

* `--uid <UID>` — The numeric user id of the owner user
* `--gid <GID>` — The numeric group id of the owner group



## `reactive-graph-tooling instances repository remove`

Removes a local repository

**Usage:** `reactive-graph-tooling instances repository remove <LOCAL_NAME> [FORCE]`

###### **Arguments:**

* `<LOCAL_NAME>` — The local name of the repository
* `<FORCE>` — If true, the default repository will be deleted

  Possible values: `true`, `false`




## `reactive-graph-tooling update`

Update the Reactive Graph binary

**Usage:** `reactive-graph-tooling update [OPTIONS] [COMMAND]`

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



## `reactive-graph-tooling update info`

Shows information about the selected release

**Usage:** `reactive-graph-tooling update info [OPTIONS]`

###### **Options:**

* `--output-format <OUTPUT_FORMAT>` — The output format

  Possible values: `table`, `html-table`, `markdown-table`, `count`, `json`, `json5`, `toml`




## `reactive-graph-tooling update list`

Lists the releases

**Usage:** `reactive-graph-tooling update list [OPTIONS]`

###### **Options:**

* `--output-format <OUTPUT_FORMAT>` — The output format

  Possible values: `table`, `html-table`, `markdown-table`, `count`, `json`, `json5`, `toml`




<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

