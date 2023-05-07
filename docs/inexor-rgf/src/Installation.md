# 2. Installation

For now please follow the [Build Instructions](./Development_Build.md).

## Linux

### Install Debian Package

1. Download the debian package from [GitHub](https://github.com/inexorgame/inexor-rgf-application/releases)
2. Install the package via `dpkg`
3. Reload systemctl units
4. Start the `default` instance

```shell
mkdir -p /tmp/inexor-rgf
cd /tmp/inexor-rgf
wget https://github.com/inexorgame/inexor-rgf-application/releases/download/v0.9.2/inexor-rgf_0.9.2_amd64.deb
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
