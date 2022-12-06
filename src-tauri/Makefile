include .config.mk

MACOS_TARGET = aarch64-apple-darwin
LINUX_TARGET = x86_64-unknown-linux-musl
WINDOWS_TARGET = x86_64-pc-windows-gnu
ASSETS_DIR = ./assets
GREEN=\033[0;32m
NC=\033[0m

install-pkgs:
	@echo "[$@]: Installing ${RUST_PKGS}"
	@rustup component add clippy
	@rustup component add rustfmt
	@cargo install cargo-make
	@cargo install cargo-audit
	@cargo install cargo-bloat
	@cargo install cargo-udeps
	@cargo install cargo-valgrind

rebuild:
	@echo "[$@]: Rebuilding ${PROJECT}"
	@cargo make --makefile Makefile.toml rebuild

build:
	@echo "[$@]: Building ${PROJECT}"
	@cargo make --makefile Makefile.toml build

build-release:
	@echo "[$@]: Building ${PROJECT} in release mode"
	@cargo make --makefile Makefile.toml build-release

.PHONY: fix
fix:  ## Fix code
	cargo fix --allow-dirty || cargo fix --allow-staged

lint:
	@echo "[$@]: Linting ${PROJECT}"
	@cargo make --makefile Makefile.toml lint

security:
	@echo "[$@]: Checking security of ${PROJECT}"
	@cargo make --makefile Makefile.toml security

bloat:
	@echo "[$@]: Evaluating resource allocation of ${PROJECT}"
	@cargo make --makefile Makefile.toml bloat ${BLOATARGS}

udeps:
	@echo "[$@]: Checking unused dependencies for ${PROJECT}"
	@cargo make --makefile Makefile.toml udeps

valgrind:
	@echo "[$@]: Checking memory leaks for ${PROJECT}"
	@cargo make --makefile Makefile.toml valgrind

test:
	@echo "[$@]: Testing ${PROJECT}"
	@cargo make --makefile Makefile.toml test


.PHONY: clean
clean:
	cargo clean

release:
	cargo build --release

debug:
	cargo build

move-assets:
	@echo "Moving assets 🔥"
	@cp -R $(ASSETS_DIR) $(BUILD_DIR)
	@echo "Assets are moved ✅"

build-macos:
	@echo "Generating MacOS binary 🍎"
	@cargo build --release --target $(MACOS_TARGET) --target-dir target/macos
	@make move-assets BUILD_DIR=target/macos/$(MACOS_TARGET)/release/assets
	@echo "MacOs binary is generated ✅"
	@echo "Run ${GREEN}make run-macos${NC}to run the binary!"

build-linux:
	@echo "Generating Linux binary 🐧"
	@cargo build --release --target $(LINUX_TARGET) --target-dir target/linux
	@make move-assets BUILD_DIR=target/linux/$(LINUX_TARGET)/release/assets
	@echo "Linux binary is generated ✅"
	@echo "Run ${GREEN}make run-linux${NC}to run the binary!"

build-windows:
	@echo "Generating Windows binary 🪟"
	@cargo build --release --target $(WINDOWS_TARGET) --target-dir target/windows
	@make move-assets BUILD_DIR=target/windows/$(WINDOWS_TARGET)/release/assets
	@echo "Windows binary is generated ✅"
	@echo "Run ${GREEN}make run-windows${NC}to run the binary!"

run-macos:
	@echo "Running MacOS binary 🍎"
	./target/macos/aarch64-apple-darwin/release/password-generator-pro

run-linux:
	@echo "Running Linux binary 🐧"
	./target/linux/x86_64-unknown-linux-musl/release/password-generator-pro

run-windows:
	@echo "Running Windows binary 🪟"
	./target/windows/x86_64-pc-windows-gnu/release/password-generator-pro.exe

package-macos:
	@echo "Packaging MacOS binary 🍎"
	@cargo make package-macos

remove-all-builds:
	@echo "Removing all builds 🧹"
	rm -rf target

build-all:
	@echo "Building all binaries 🚀"
	@make build-macos
