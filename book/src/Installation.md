# 2. Installation

## Linux

### Docker

Pull the docker image:

```shell
docker pull ghcr.io/reactive-graph/reactive-graph:latest
```

Create and start a container:

```shell
docker run --name reactive-graph -p 0.0.0.0:31415:31415 ghcr.io/reactive-graph/reactive-graph:latest
```

### Ubuntu / Debian / Raspberry Pi OS

```admonish info "Supported Distributions"
The installation instructions below are tested on:
* Ubuntu 24.04
* Ubuntu 22.04
* Raspberry Pi OS (Bullseye)

Other debian based distributions will also work if the following requirement is satisfied:
* libc6 (>= 2.35)
```

```admonish info "APT Repository"
An APT repository is available at [https://apt.reactive-graph.io/](https://apt.reactive-graph.io/).

Packages are currently available for these architectures:
* amd64
* arm64
* armhf
* i386
* ppc64
* ppc64el
* riscv64
```

#### Setup APT repository and download GPG keys

```shell
echo "deb https://apt.reactive-graph.io/ focal main" | sudo tee -a /etc/apt/sources.list.d/reactive-graph.list
gpg --recv-keys --keyserver keyserver.ubuntu.com 1F7F762FFE6BF816DB4C41D218D6C25399307BA5
gpg --recv-keys --keyserver keyserver.ubuntu.com 18D6C25399307BA5
gpg --export 1F7F762FFE6BF816DB4C41D218D6C25399307BA5 | sudo tee /etc/apt/trusted.gpg.d/apt.reactive-graph.io.gpg
gpg --export 18D6C25399307BA5 | sudo tee /etc/apt/trusted.gpg.d/reactive-graph.gpg
sudo apt update
```

### Install Reactive Graph

```shell
sudo apt install reactive-graph
```

### Create the first (default) instance of the runtime

```shell
sudo systemctl daemon-reload
sudo systemctl start reactive-graph@default
```

### Configure the default instance

```admonish info "Configuration Files"
The configuration files are located at `/etc/reactive-graph/{instance-name}/`.
```

```shell
sudo nano /etc/reactive-graph/default/graphql.toml
sudo systemctl restart reactive-graph@default
```

### Optional: Create further instances

```admonish info "Multi tenancy"
It's possible to run multiple instances by starting another instance with a different name. The necessary
configuration files are created at the first start. You propaply want to change the configuration files
since they are copied from the default instance.
```

```shell
sudo systemctl start reactive-graph@second
sudo nano /etc/reactive-graph/second/graphql.toml
```

### Optional: Install plugins

Once you have the runtime running you can install the plugins.

The package names starts with `libreactive-graph-plugin`.

To get a list of available plugins, you can search for it like so:

```shell
apt search libreactive-graph-plugin
```

To install a plugin for the default instance you just have to install the package and restart the service:

```shell
sudo apt install libreactive-graph-plugin-base
sudo systemctl start reactive-graph@default
```

Similarly, you can just install all available plugins:

```shell
sudo apt install "libreactive-graph-plugin-*"
sudo systemctl start reactive-graph@default
```

```admonish info "Plugins and multi tenancy"
If you want to run multiple instances, each instance has it's own set of plugins.

The installed plugins for the default instance are located at:
`/usr/share/reactive-graph/default/plugins/installed`

Likewise plugins for another instance are located at:
`/usr/share/reactive-graph/{instance-name}/plugins/installed`

To install a plugin you can copy it from the default instance install folder into the deploy folder
of the other instance like so:
`cp /usr/share/reactive-graph/default/plugins/installed/{plugin.so} /usr/share/reactive-graph/{instance-name}/plugins/deploy`

If the other instance is already running, it's not necessary to restart the instance runtime,
because the runtime is able to hot deploy plugins.
```

### Manually Install Debian Package

Another way is to install the debian packages manually:

1. Download the debian package from [GitHub](https://github.com/reactive-graph/reactive-graph/releases)

    ```shell
    mkdir -p /tmp/reactive-graph
    cd /tmp/reactive-graph
    wget https://github.com/reactive-graph/reactive-graph/releases/download/{version}/reactive-graph_{version}_amd64.deb
    ```

2. Install the package via `dpkg`

    ```shell
    sudo dpkg -i reactive-graph_0.10.0_amd64.deb
    ```

3. Reload systemctl units

    ```shell
    sudo systemctl daemon-reload
    ```

4. Start the `default` instance

    ```shell
    sudo systemctl start reactive-graph@default
    ```

#### Configure an instance

```shell
# Name, Description
sudo nano /etc/reactive-graph/instance-name/instance.toml
# Port
sudo nano /etc/reactive-graph/instance-name/graphql.toml
# Log levels
sudo nano /etc/reactive-graph/instance-name/logging.toml
# Enable / disable plugin(s)
sudo nano /etc/reactive-graph/instance-name/plugins.toml
# Manage remotes
sudo nano /etc/reactive-graph/instance-name/remotes.toml
```

Then restart the service:

```shell
sudo systemctl restart reactive-graph@instance-name
```

#### Create a new instance (system wide)

Create a new instance by using a new instance name:

```shell
sudo systemctl start reactive-graph@instance-name
```

Adjust the GraphQL configuration (port, hostname) and the instance configuration (name). Then restart

```shell
sudo systemctl restart reactive-graph@instance-name
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
