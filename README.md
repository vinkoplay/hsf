# hsf
Simple Hosts file utility for Linux

# How to install?
## Installing dependencies
For build need rustup, glibc
### Ubuntu | Debian | Linux Mint | Pop!_OS
​```bash
sudo apt update
sudo apt install build-essential curl
curl --proto '=https' --tlsv1.2 -sSf https://rustup.rs | sh
​```
reload terminal
### Arch Linux
​```bash
sudo pacman -S rustup base-devel
​```
### Fedora | Red Hat | CentOS
​```bash
sudo dnf groupinstall "Development Tools"
sudo dnf install rustup
​```
### openSUSE
​```bash
sudo zypper install -t pattern devel_basis
sudo zypper install rustup
​```
### Alpine Linux
​```bash
apk add build-base rustup
​```
---
### Installing rust compier and cargo
​```bash
rustup default stable
​```
## Installing hsf
Clone the github project
​```bash
git clone https://github.com/vinkoplay/hsf.git
cd hsf
​```
Run the installing script
​```bash
./install.sh
​```