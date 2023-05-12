# 2. Installation

For now please follow the [Build Instructions](./Development_Build.md).

## Linux

### Ubuntu / Debian

```shell
echo "deb https://apt.rgf.app/ focal main" | sudo tee -a /etc/apt/sources.list.d/inexor-rgf.list
gpg --recv-keys --keyserver keyserver.ubuntu.com 1F7F762FFE6BF816DB4C41D218D6C25399307BA5
gpg --recv-keys --keyserver keyserver.ubuntu.com 18D6C25399307BA5
gpg --export 1F7F762FFE6BF816DB4C41D218D6C25399307BA5 | sudo tee /etc/apt/trusted.gpg.d/apt.rgf.app.gpg
gpg --export 18D6C25399307BA5 | sudo tee /etc/apt/trusted.gpg.d/inexor-rgf.gpg
sudo apt update
sudo apt install inexor-rgf "libinexor-rgf-plugin-*"
sudo systemctl daemon-reload
sudo systemctl start inexor-rgf@default
```

### Manually Install Debian Package

1. Download the debian package from [GitHub](https://github.com/inexorgame/inexor-rgf-application/releases)
2. Install the package via `dpkg`
3. Reload systemctl units
4. Start the `default` instance

```shell
mkdir -p /tmp/inexor-rgf
cd /tmp/inexor-rgf
wget https://github.com/inexorgame/inexor-rgf-application/releases/download/v0.9.2/inexor-rgf_0.9.1_amd64.deb
sudo dpkg -i inexor-rgf_0.9.1_amd64.deb
```

#### Start the default instance (system wide)

```shell
sudo systemctl daemon-reload
sudo systemctl start inexor-rgf@default
```

#### Start the default instance (system wide)

```shell
sudo systemctl daemon-reload
sudo systemctl start inexor-rgf@default
```

#### Configure an instance

```shell
nano /etc/inexor-rgf/instance-name/instance.toml
nano /etc/inexor-rgf/instance-name/graphql.toml
nano /etc/inexor-rgf/instance-name/logging.toml
nano /etc/inexor-rgf/instance-name/plugins.toml
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

## Raspberry Pi

```admonish warning "TODO"
Describe how to install Inexor Reactive Graph Flow on a raspberry pi
```

## Windows

```admonish warning "TODO"
Describe how to install Inexor Reactive Graph Flow on windows
```
