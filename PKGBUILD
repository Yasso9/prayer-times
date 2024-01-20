# Maintainer: Ilyas Turki
pkgname=prayer-times
pkgver=0.1.0
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
pkgdesc="Islamic Prayer Times Notification"

build() {
    return 0
}

package() {
    cargo install --root="$pkgdir" prayer-times
}
