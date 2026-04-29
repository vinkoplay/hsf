# hsf
Simple Hosts file utility for Linux

# How to install?
## Installing dependencies
Build Requirements rustup, glibc
### Ubuntu | Debian | Linux Mint | Pop!_OS
```sh
sudo apt update
sudo apt install build-essential curl
curl --proto '=https' --tlsv1.2 -sSf https://rustup.rs | sh
```
reload terminal
### Arch Linux
```sh
sudo pacman -S rustup base-devel
```
### Fedora | Red Hat | CentOS
```sh
sudo dnf groupinstall "Development Tools"
sudo dnf install rustup
```
### openSUSE
```sh
sudo zypper install -t pattern devel_basis
sudo zypper install rustup
```
### Alpine Linux
```sh
apk add build-base rustup
```
---
### Installing rust compiler and cargo
```sh
rustup default stable
```
## Installing hsf
Clone the github project

```sh
git clone https://github.com/vinkoplay/hsf.git
cd hsf
```

Run the installation script

```sh
chmod +x install.sh
./install.sh
```

# Usage Example
```sh
hsf help
hsf version
sudo hsf base
```

# Info
To help use:
```sh
hsf --help
```

or

```sh
man hsf
```

[Presets doc](https://github.com/vinkoplay/hsf/blob/main/docs/presets.md)