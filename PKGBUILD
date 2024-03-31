# Maintainer: Ilyas Turki <turki.ilyass@gmail.com>
pkgname=prayer-times
pkgver=0.2.0
pkgrel=1
makedepends=('cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
pkgdesc="Islamic Prayer Times Informations and Notifications"
url="https://github.com/Yasso9/prayer-times"
license=('MIT')
source=("$pkgname-$pkgver.tar.gz::https://github.com/Yasso9/prayer-times/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
	cd "$pkgname-$pkgver"
	cargo build --release --locked
}

check() {
	cd "$pkgname-$pkgver"
	cargo check
}

package() {
	cd "$pkgname-$pkgver"

	install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"

	# Shell completions
	install -Dm644 "target/completions/_$pkgname" "$pkgdir/usr/share/zsh/site-functions/_$pkgname"
	install -Dm644 "target/completions/$pkgname.bash" "$pkgdir/usr/share/bash-completion/completions/$pkgname"
	install -Dm644 "target/completions/$pkgname.fish" "$pkgdir/usr/share/fish/vendor_completions.d/$pkgname.fish"

	install -Dm644 "README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
	install -Dm644 "assets/mosque-svgrepo-com.png" "$pkgdir/usr/share/icons/mosque-svgrepo-com.png"
}
