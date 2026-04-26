# Maintainer: Vinkoplay
pkgname=hsf-git
pkgver=1.0.0
pkgrel=1
pkgdesc="A fast and secure /etc/hosts manager written in Rust"
arch=('x86_64')
url="https://github.com"
license=('MIT')
depends=('gcc-libs' 'glibc')
makedepends=('rust' 'cargo')
provides=('hsf')
conflicts=('hsf')
source=("git+${url}.git")
sha256sums=('SKIP')

build() {
    cd "$srcdir/${pkgname%-git}"
    export RUSTFLAGS="-C target-cpu=native"
    cargo build --release --locked
}

package() {
    cd "$srcdir/${pkgname%-git}"
    
    install -Dm755 "target/release/hsf" "$pkgdir/usr/bin/hsf"
    
    install -Dm644 "hsf.1" "$pkgdir/usr/share/man/man1/hsf.1"
    
}
