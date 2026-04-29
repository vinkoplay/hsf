#!/bin/bash
set -e

# Checking depends
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust/Cargo is not installed. Please install it first (sudo pacman -S rust)."
    exit 1
fi

RUSTFLAGS="-C target-cpu=native" cargo build --release --locked

sudo install -Dm755 "target/release/hsf" "/usr/local/bin/hsf"
sudo install -Dm644 "hsf.1" "/usr/share/man/man1/hsf.1"
sudo install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"

sudo mandb &> /dev/null