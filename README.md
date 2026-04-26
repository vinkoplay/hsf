# hsf
Simple Hosts file utility for Linux

# How to install?
## Installing dependencies
For build need rustup, glibc
### Ubuntu | Debian | Linux Mint | Pop!_OS
​```sh
sudo apt update
sudo apt install build-essential curl
curl --proto '=https' --tlsv1.2 -sSf https://rustup.rs | sh
​```
reload terminal
### Arch Linux
​```sh
sudo pacman -S rustup base-devel
​```
### Fedora | Red Hat | CentOS
​```sh
sudo dnf groupinstall "Development Tools"
sudo dnf install rustup
​```
### openSUSE
​```sh
sudo zypper install -t pattern devel_basis
sudo zypper install rustup
​```
### Alpine Linux
​```sh
apk add build-base rustup
​```
---
### Installing rust compier and cargo
​```sh
rustup default stable
​```
## Installing hsf
Clone the github project

​```sh
git clone https://github.com/vinkoplay/hsf.git
cd hsf
​```

Run the installing script

​```sh
./install.sh
​```