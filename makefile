dev-tauri:
	pnpm install &&	RUST_DEBUT=1 pnpm tauri dev

build-bundle: c wasm
	cd src-web && make build-web && cd .. && cargo tauri build --target universal-apple-darwin

build-debug-bundle: c wasm
	cd src-web && make build-web && cd .. && cargo tauri build --debug --target universal-apple-darwin

c:
	cargo install tauri-cli
	cargo clippy --fix --allow-dirty --allow-staged

TARGET_PATH_BASE = target/universal-apple-darwin/release/bundle/macos/Echoo
APP_SIGN_IDENTITY = "3rd Party Mac Developer Application: Chiang Hwang (6T8X94ZY3T)"
INSTALLER_SIGN_IDENTITY = "3rd Party Mac Developer Installer: Chiang Hwang (6T8X94ZY3T)"
ENTITLEMENTS_PLIST = ./entitlements.plist

build-mac:
	plutil -convert xml1 ${ENTITLEMENTS_PLIST}
	codesign --force --verbose --deep --sign ${APP_SIGN_IDENTITY}  ./${TARGET_PATH_BASE}.app
	codesign --force --verbose --deep --sign ${APP_SIGN_IDENTITY} --entitlements ${ENTITLEMENTS_PLIST} ./${TARGET_PATH_BASE}.app
	productbuild --component ${TARGET_PATH_BASE}.app /Applications ${TARGET_PATH_BASE}.pkg --sign ${INSTALLER_SIGN_IDENTITY} --product ${ENTITLEMENTS_PLIST}