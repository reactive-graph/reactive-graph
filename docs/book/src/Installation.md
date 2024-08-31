# 2. Installation

## Linux

### Ubuntu / Debian / Raspberry Pi OS

```admonish info "Supported Distributions"
The installation instructions below are tested on:
* Ubuntu 23.04
* Ubuntu 22.04
* Ubuntu 20.04
* Raspberry Pi OS (Bullseye)

Other debian based distributions will also work if the following requirement is satisfied:
* libc6 (>= 2.31)
```

```admonish info "APT Repository"
An APT repository is available at [https://apt.rgf.app/](https://apt.rgf.app/).

Packages are currently available for these architectures:
* amd64
* arm64
* armhf
```

#### Setup APT repository and download GPG keys

```shell
echo "deb https://apt.rgf.app/ focal main" | sudo tee -a /etc/apt/sources.list.d/inexor-rgf.list
gpg --recv-keys --keyserver keyserver.ubuntu.com 1F7F762FFE6BF816DB4C41D218D6C25399307BA5
gpg --recv-keys --keyserver keyserver.ubuntu.com 18D6C25399307BA5
gpg --export 1F7F762FFE6BF816DB4C41D218D6C25399307BA5 | sudo tee /etc/apt/trusted.gpg.d/apt.rgf.app.gpg
gpg --export 18D6C25399307BA5 | sudo tee /etc/apt/trusted.gpg.d/inexor-rgf.gpg
sudo apt update
```

### Install the runtime

```shell
sudo apt install inexor-rgf
```

### Create the first (default) instance of the runtime

```shell
sudo systemctl daemon-reload
sudo systemctl start inexor-rgf@default
```

### Configure the default instance

```admonish info "Configuration Files"
The configuration files are located at `/etc/inexor-rgf/{instance-name}/`.
```

```shell
sudo nano /etc/inexor-rgf/default/graphql.toml
sudo systemctl restart inexor-rgf@default
```

### Optional: Create further instances

```admonish info "Multi tenancy"
It's possible to run multiple instances by starting another instance with a different name. The necessary
configuration files are created at the first start. You propaply want to change the configuration files
since they are copied from the default instance.
```

```shell
sudo systemctl start inexor-rgf@second
sudo nano /etc/inexor-rgf/second/graphql.toml
```

### Optional: Install plugins

Once you have the runtime running you can install the plugins.

The package names starts with `libinexor-rgf-plugin`.

To get a list of available plugins, you can search for it like so:

```shell
apt search libinexor-rgf-plugin
```

To install a plugin for the default instance you just have to install the package and restart the service:

```shell
sudo apt install libreactive-graph-plugin-base
sudo systemctl start inexor-rgf@default
```

Similarly, you can just install all available plugins:

```shell
sudo apt install "libreactive-graph-plugin-*"
sudo systemctl start inexor-rgf@default
```

```admonish info "Plugins and multi tenancy"
If you want to run multiple instances, each instance has it's own set of plugins.

The installed plugins for the default instance are located at:
`/usr/share/inexor-rgf/default/plugins/installed`

Likewise plugins for another instance are located at:
`/usr/share/inexor-rgf/{instance-name}/plugins/installed`

To install a plugin you can copy it from the default instance install folder into the deploy folder
of the other instance like so:
`cp /usr/share/inexor-rgf/default/plugins/installed/{plugin.so} /usr/share/inexor-rgf/{instance-name}/plugins/deploy`

If the other instance is already running, it's not necessary to restart the instance runtime,
because the runtime is able to hot deploy plugins.
```

### Manually Install Debian Package

Another way is to install the debian packages manually:

1. Download the debian package from [GitHub](https://github.com/reactive-graph/reactive-graph/releases)

    ```shell
    mkdir -p /tmp/inexor-rgf
    cd /tmp/inexor-rgf
    wget https://github.com/reactive-graph/reactive-graph/releases/download/{version}/inexor-rgf_{version}_amd64.deb
    ```

2. Install the package via `dpkg`

    ```shell
    sudo dpkg -i inexor-rgf_0.9.1_amd64.deb
    ```

3. Reload systemctl units

    ```shell
    sudo systemctl daemon-reload
    ```

4. Start the `default` instance

    ```shell
    sudo systemctl start inexor-rgf@default
    ```

#### Configure an instance

```shell
# Name, Description
sudo nano /etc/inexor-rgf/instance-name/instance.toml
# Port
sudo nano /etc/inexor-rgf/instance-name/graphql.toml
# Log levels
sudo nano /etc/inexor-rgf/instance-name/logging.toml
# Enable / disable plugin(s)
sudo nano /etc/inexor-rgf/instance-name/plugins.toml
```

Then restart the service:

```shell
sudo systemctl restart inexor-rgf@instance-name
```

#### Create a new instance (system wide)

Create a new instance by using a new instance name:

```shell
sudo systemctl start inexor-rgf@instance-name
```

Adjust the GraphQL configuration (port, hostname) and the instance configuration (name). Then restart

```shell
sudo systemctl restart inexor-rgf@instance-name
```

### Other distributions / Manually Install Binary

You can install the binary packages manually:

1. Download the binary from https://github.com/reactive-graph/reactive-graph/releases

    ```shell
    wget https://github.com/reactive-graph/reactive-graph/releases/download/v{version}/reactive-graph-x86_64-unknown-linux-gnu.tar.gz
    ```

2. Extract the binary

    ```shell
    tar xvf reactive-graph-x86_64-unknown-linux-gnu.tar.gz
    ```

3. Start the binary

    ```shell
    cd reactive-graph-x86_64-unknown-linux-gnu
    ./reactive-graph
    ```

### From Source

Please follow the [Build Instructions](./Development_Build.md).

## Windows

### Windows Installer

```admonish warning "Coming soom(tm)"
We plan to provide an installer for windows.
```

### Manually install binaries

1. Download the latest binary from https://github.com/reactive-graph/reactive-graph/releases
2. Unzip the archive
3. Change into the directory
4. Execute `reactive-graph.exe`

### From Source

Please follow the [Build Instructions](./Development_Build.md).

## MacOS

### Manually install binaries

1. Download the latest binary from https://github.com/reactive-graph/reactive-graph/releases
2. Extract the archive
3. Change into the directory
4. Execute `reactive-graph`

### From Source

Please follow the [Build Instructions](./Development_Build.md).
