#  SYNOPSIS
#
#    make <target> [flags...]
#
#  TARGETS
#    bloat             		Evaluating resource allocation of the project
#    build             		Building the project
#    build-all         		Building all the binaries
#    build-linux       		Generating Linux binary
#    build-macos       		Generating MacOS binary
#    build-release     		Build the project in release mode
#    build-windows     		Generating Windows binary
#    clean             		Clean the project
#    fix               		Fix code
#    install-pkgs      		Install required packages
#    lint              		Lint code
#    move-assets       		Move assets
#    package-macos     		Package macOS binary
#    rebuild           		Rebuild the project
#    release           		Build the project in release mode
#    remove-all-builds 		Remove all builds
#    run-linux         		Run Linux binary
#    run-macos         		Run macOS binary
#    run-windows       		Run Windows binary
#    test              		Run tests
#    test-all          		Run all tests
#    test-linux        		Run Linux tests
#    test-macos        		Run macOS tests
#    test-windows      		Run Windows tests
#    update            		Update dependencies
#    update-all        		Update all dependencies
#    update-linux      		Update Linux dependencies
#    update-macos      		Update macOS dependencies
#    update-windows    		Update Windows dependencies
#    version           		Print version
#
#  FLAGS
#    BLOATARGS         		Arguments to pass to cargo bloat
#    BUILD_DIR         		Build directory
#    RUST_PKGS         		Rust packages to install
#    TESTARGS          		Arguments to pass to cargo test
#    TESTARGS_LINUX    		Arguments to pass to cargo test on Linux
#    TESTARGS_MACOS    		Arguments to pass to cargo test on macOS
#    TESTARGS_WINDOWS  		Arguments to pass to cargo test on Windows
#    UPDATEARGS        		Arguments to pass to cargo update
#    UPDATEARGS_LINUX  		Arguments to pass to cargo update on Linux
#    UPDATEARGS_MACOS  		Arguments to pass to cargo update on macOS
#    UPDATEARGS_WINDOWS 	Arguments to pass to cargo update on Windows
#

# Include config.mk
include config.mk

.DEFAULT_GOAL := help

# Banner function
define banner
	@printf "\n"
	@string="$(1)" && printf "$$(tput bold)[$@]: %-$$((76))s *\n" "$$string"
	@printf "\n"
endef

# Evaluating resource allocation of the project
bloat:
	$(call banner,Evaluating resource allocation of ${PROJECT} 🤔)
	@$(CARGO) bloat ${BLOATARGS}
	@echo "Bloat is done ✅"

# Building the project
build:
	$(call banner,Building ${PROJECT} 👷‍♀️)
	@$(CARGO) build
	@echo "Build is done ✅"

# Building all the binaries
build-all:
	$(call banner,Building all the binaries 🚀)
	@make build-macos
	@echo "MacOS binary is built ✅"
	@make build-linux
	@echo "Linux binary is built ✅"
	@make build-windows
	@echo "Windows binary is built ✅"
	@echo "All binaries are now built ✅"

# Generating Linux binary
build-linux:
	$(call banner,Generating Linux binary 🐧)
	@$(CARGO) build --release --target $(LINUX_TARGET) --target-dir target/linux
	@make move-assets BUILD_DIR=target/linux/$(LINUX_TARGET)/release/assets
	@echo "Linux binary is built ✅"
	@echo "Run ${GREEN}make run-linux${NC}to run the binary!"

# Generating MacOS binary
build-macos:
	$(call banner,Generating MacOS binary 🍎)
	@$(CARGO) build --release --target $(MACOS_TARGET) --target-dir target/macos
	@make move-assets BUILD_DIR=target/macos/$(MACOS_TARGET)/release/assets
	@echo "MacOs binary is built ✅"
	@echo "Run ${GREEN}make run-macos${NC}to run the binary!"

# Build the project in release mode
build-release:
	$(call banner,Building ${PROJECT} in release mode)
	@$(CARGO) build-release
	@echo "Build-release is done ✅"

# Generating Windows binary
build-windows:
	$(call banner,Generating Windows binary 🪟)
	@$(CARGO) build --release --target $(WINDOWS_TARGET) --target-dir target/windows
	@make move-assets BUILD_DIR=target/windows/$(WINDOWS_TARGET)/release/assets
	@echo "Windows binary is built ✅"
	@echo "Run ${GREEN}make run-windows${NC}to run the binary!"

# Clean the project
clean:
	$(call banner,Cleaning ${PROJECT} 🧽)
	@$(CARGO) clean
	@echo "Clean is done ✅"

# Fix code
fix:  ## Fix code 🛠
	$(call banner,Fixing ${PROJECT} 🛠)
	@$(CARGO) fix --allow-dirty || cargo fix --allow-staged
	@echo "Fix is done ✅"

# Install required packages
install-pkgs:
	$(call banner,Installing ${RUST_PKGS})
	@rustup component add clippy
	@echo "Clippy is installed ✅"
	@rustup component add rustfmt
	@echo "Rustfmt is installed ✅"
	@$(CARGO) install cargo-make
	@echo "Cargo-make is installed ✅"
	@$(CARGO) install cargo-audit
	@echo "Cargo-audit is installed ✅"
	@$(CARGO) install cargo-bloat
	@echo "Cargo-bloat is installed ✅"
	@$(CARGO) install cargo-udeps
	@echo "Cargo-udeps is installed ✅"
	@$(CARGO) install cargo-valgrind
	@echo "Cargo-valgrind is installed ✅"

# Lint code
lint:
	$(call banner,Linting ${PROJECT} 🧹)
	@$(CARGO) lint
	@echo "Lint is done ✅"

# Move assets
move-assets:
	$(call banner,Moving assets 🔥)
	@cp -R $(ASSETS_DIR) $(BUILD_DIR)
	@echo "Assets are moved ✅"

# Icon generation
icon:
	$(call banner,Generating icon 🎨)
	@cargo tauri icon assets/icon.png
	@echo "Icon is generated ✅"

# Package windows binaries (i686-pc-windows-msvc, x86_64-pc-windows-msvc)
package-windows:
	$(call banner,Generating Windows binaries 🪟)
	@rustup target add i686-pc-windows-msvc
	@echo "i686-pc-windows-msvc is added ✅"
	@rustup target add x86_64-pc-windows-msvc
	@echo "x86_64-pc-windows-msvc is added ✅"
	@cargo tauri build --target i686-pc-windows-msvc
	@echo "i686-pc-windows-msvc is built ✅"
	@cargo tauri build --target x86_64-pc-windows-msvc
	@echo "x86_64-pc-windows-msvc is built ✅"

# Package macos binaries (aarch64-apple-darwin, x86_64-apple-darwin)
package-macos:
	$(call banner,Generating MacOS binaries 🍎)
	@rustup target add aarch64-apple-darwin
	@echo "aarch64-apple-darwin is added ✅"
	@rustup target add x86_64-apple-darwin
	@echo "x86_64-apple-darwin is added ✅"
	@cargo-tauri build --target aarch64-apple-darwin
	@echo "aarch64-apple-darwin is built ✅"
	@cargo-tauri build --target x86_64-apple-darwin
	@echo "x86_64-apple-darwin is built ✅"
	@chmod +x /Users/runner/work/syng/syng/src/native/target/x86_64-apple-darwin/release/bundle/macos/Syng.app/Contents/MacOS/Syng
	@echo "x86_64-apple-darwin is made executable ✅"
	@chmod +x /Users/runner/work/syng/syng/src/native/target/aarch64-apple-darwin/release/bundle/macos/Syng.app/Contents/MacOS/Syng
	@echo "aarch64-apple-darwin is made executable ✅"

# Package linux arm64 binaries (aarch64-unknown-linux-gnu)
package-linux-arm64:
	$(call banner,Packaging Linux arm64 binaries 🐧)
	@rustup target add aarch64-unknown-linux-gnu
	@echo "aarch64-unknown-linux-gnu is added ✅"
	@cargo tauri build --target aarch64-unknown-linux-gnu
	@echo "aarch64-unknown-linux-gnu is built ✅"

# Package linux amd64 binaries (x86_64-unknown-linux-gnu)
package-linux-amd64:
	$(call banner,Packaging Linux x86_64 binaries 🐧)
	@rustup target add x86_64-unknown-linux-gnu
	@echo "x86_64-unknown-linux-gnu is added ✅"
	@cargo tauri build --target x86_64-unknown-linux-gnu
	@echo "x86_64-unknown-linux-gnu is built ✅"

# Package linux i686 binaries
package-linux-i686:
	$(call banner,Packaging Linux i686 binaries 🐧)
	@rustup target add i686-unknown-linux-gnu
	@echo "i686-unknown-linux-gnu is added ✅"
	@cargo tauri build --target i686-unknown-linux-gnu
	@echo "i686-unknown-linux-gnu is built ✅"

# Rebuild the project
rebuild:
	$(call banner,Rebuilding ${PROJECT} 🚀)
	@$(CARGO) rebuild
	@echo "Rebuild is done ✅"

# Build the project in release mode
release:
	$(call banner,Building ${PROJECT} in release mode 🚀)
	@$(CARGO) build --release
	@echo "Release build is done ✅"

# Remove all builds
remove-all-builds:
	$(call banner,Removing all builds 🧹)
	rm -rf target
	@echo "All builds are removed ✅"

# Run Linux binary
run-linux:
	$(call banner,Running Linux binary 🐧)
	./target/linux/x86_64-unknown-linux-musl/release/password-generator-pro
	@echo "Running Linux binary! ✅"

# Run macOS binary
run-macos:
	$(call banner,Running MacOS binary 🍎)
	./target/macos/aarch64-apple-darwin/release/password-generator-pro
	@echo "Running MacOS binary! ✅"

# Run Windows binary
run-windows:
	$(call banner,Running Windows binary 🪟)
	./target/windows/x86_64-pc-windows-gnu/release/password-generator-pro.exe
	@echo "Running Windows binary! ✅"

# Check code for security issues
security:
	$(call banner,Checking security of ${PROJECT} 🔐)
	@$(CARGO) security
	@echo "Security check is done ✅"

# Run tests
test:
	$(call banner,Testing ${PROJECT} 🧪)
	@$(CARGO) test
	@echo "Tests are done ✅"

# Check unused dependencies
udeps:
	$(call banner,Checking unused dependencies for ${PROJECT} 📦)
	@$(CARGO) udeps
	@echo "Unused dependencies check is done ✅"

# Check memory leaks
valgrind:
	$(call banner,Checking memory leaks for ${PROJECT} 🧠)
	@$(CARGO) valgrind
	@echo "Memory leaks check is done ✅"

# Display the help menu
help:
	@echo "\n$$(tput bold)Welcome to Password Generator Pro! - A Fast, Simple And Secure Password Generator$$(tput sgr0)\n"
	@ echo '  Usage:'
	@ echo ''
	@ echo '    make <target> [flags...]'
	@ echo ''
	@ echo '  Targets:'
	@ echo ''
	@ awk '/^#/{ comment = substr($$0,3) } comment && /^[a-zA-Z][a-zA-Z0-9_-]+ ?:/{ print "   ", $$1, comment }' $(MAKEFILE_LIST) | column -t -s ':' | sort
	@ echo ''
	@ echo '  Flags:'
	@ echo ''
	@ echo '    BLOATARGS 		Arguments to pass to cargo bloat'
	@ echo '    BUILD_DIR 		Directory to build the project'
	@ echo '    PROJECT 		Name of the project'
	@ echo '    RUST_PKGS 		Rust packages to install'
	@ echo '    ASSETS_DIR 		Directory containing assets'
	@ echo '    LINUX_TARGET 	Target for Linux'
	@ echo '    MACOS_TARGET 	Target for MacOS'
	@ echo '    WINDOWS_TARGET 	Target for Windows'
	@ echo ''

.PHONY: bloat build build-all build-linux build-macos build-release build-windows clean fix help install-pkgs lint move-assets package-macos rebuild release remove-all-builds run-linux run-macos run-windows security test udeps valgrind