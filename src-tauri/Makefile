# Password Generator Pro Makefile
# Maintainer: Sebastien Rousseau
# License: See LICENSE-APACHE and LICENSE-MIT files for details.

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
#	 help              		Show this help
#    install-pkgs      		Install required packages
#    lint              		Lint code
#    move-assets       		Move assets
#	 icon			  		Generate icon
#    package-windows   		Package Windows binary
#    package-macos     		Package macOS binary
#    package-linux-arm64 	Package Linux ARM64 binary
#    package-linux-amd64 	Package Linux AMD64 binary
#    package-linux-i686  	Package Linux I686 binary
#    rebuild           		Rebuild the project
#    release           		Build the project in release mode
#    remove-all-builds 		Remove all builds
#    run-linux         		Run Linux binary
#    run-macos         		Run macOS binary
#    run-windows       		Run Windows binary
#    security          		Security checks
#    test              		Run tests
#    udeps			 		Find unused dependencies
#    valgrind          		Run Valgrind
#
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

include Makefile.config.in
include Makefile.log.in

# Evaluating resource allocation of the project
bloat:
	$(call log,Evaluating resource allocation of ${PROJECT} 🤔)
	@$(CARGO) make --makefile Makefile.toml bloat ${BLOATARGS}
	$(call log, Completed evaluating resource allocation of ${PROJECT} ✅)

# Building the project
build:
	$(call log,Compiling ${PROJECT} and all of its dependencies 👷‍♀️)
	@$(CARGO) make --makefile Makefile.toml build
	$(call log, Build is done ✅)

# Building all the binaries
build-all:
	$(call log,Compiling ${PROJECT} for all the platforms and their dependencies 👷‍♀️)
	@make build-macos
	$(call log, MacOS binary is built 	✅)
	@make build-linux
	$(call log, Linux binary is built 	✅)
	@make build-windows
	$(call log, Windows binary is built ✅)
	$(call log, All ${PROJECT} binaries for macOS, Linux and Windows are built ✅)

# Generating Linux binary
build-linux:
	$(call log,Generating ${PROJECT} Linux binary 🐧)
	@rustup target add $(LINUX_TARGET)
	@$(CARGO) make --makefile Makefile.toml build-linux
	@make move-assets BUILD_DIR=target/linux/$(LINUX_TARGET)/release/assets
	$(call log, Linux binary is built 	✅)
	$(call log, Run ${GREEN}make run-linux${NC}to run the binary!")

# Generating MacOS binary
build-macos:
	$(call log,Generating ${PROJECT} MacOS binary 🍎)
	@rustup target add $(MACOS_TARGET)
	@$(CARGO) make --makefile Makefile.toml build-macos
	@make move-assets BUILD_DIR=target/macos/$(MACOS_TARGET)/release/assets
	$(call log, MacOs binary is built 	✅)
	$(call log, Run ${GREEN}make run-macos${NC}to run the binary!")

# Build the project in release mode
build-release:
	$(call log,Building ${PROJECT} in release mode)
	@$(CARGO) build-release
	$(call log, Build-release is done 	✅)

# Generating Windows binary
build-windows:
	$(call log,Generating ${PROJECT} Windows binary 🪟)
	@rustup target add $(WINDOWS_TARGET)
	@$(CARGO) make --makefile Makefile.toml build-windows
	@make move-assets BUILD_DIR=target/windows/$(WINDOWS_TARGET)/release/assets
	$(call log, Windows binary is built ✅)
	$(call log, Run ${GREEN}make run-windows${NC}to run the binary!")

# Clean the project
clean:
	$(call log,Cleaning ${PROJECT} 🧽)
	@$(CARGO) clean
	$(call log, Clean is done ✅)

# Fix code
fix:  ## Fix code 🛠
	$(call log,Fixing ${PROJECT} 🛠)
	@$(CARGO) fix --allow-dirty || cargo fix --allow-staged
	$(call log, Fix is done ✅)

# Install required packages
install-pkgs:
	$(call log,Installing ${RUST_PKGS})
	@rustup component add clippy
	$(call log, Clippy is installed ✅)
	@rustup component add rustfmt
	$(call log, Rustfmt is installed ✅)
	@$(CARGO) install cargo-make
	$(call log, Cargo-make is installed ✅)
	@$(CARGO) install cargo-audit
	$(call log, Cargo-audit is installed ✅)
	@$(CARGO) install cargo-bloat
	$(call log, Cargo-bloat is installed ✅)
	@$(CARGO) install cargo-udeps
	$(call log, Cargo-udeps is installed ✅)
	@$(CARGO) install cargo-valgrind
	$(call log, Cargo-valgrind is installed ✅)

# Lint code
lint:
	$(call log,Checks a package to catch common mistakes and improve ${PROJECT} 🧹)
	@$(CARGO) clippy --all-targets --all-features -- -D warnings
	$(call log, Lint is done ✅)

# Move assets
move-assets:
	$(call log,Moving assets 🔥)
	@cp -R $(ASSETS_DIR) $(BUILD_DIR)
	$(call log, Assets are moved ✅)

# Icon generation
icon:
	$(call log,Generating icon 🎨)
	@cargo tauri icon assets/icon.png
	$(call log, Icon is generated ✅)

# Package windows binaries (i686-pc-windows-msvc, x86_64-pc-windows-msvc)
package-windows:
	$(call log,Generating Windows binaries 🪟)
	@rustup target add i686-pc-windows-msvc
	$(call log, i686-pc-windows-msvc is added ✅)
	@rustup target add x86_64-pc-windows-msvc
	$(call log, x86_64-pc-windows-msvc is added ✅)
	@cargo tauri build --target i686-pc-windows-msvc
	$(call log, i686-pc-windows-msvc is built ✅)
	@cargo tauri build --target x86_64-pc-windows-msvc
	$(call log, x86_64-pc-windows-msvc is built ✅)

# Package macos binaries (aarch64-apple-darwin, x86_64-apple-darwin)
package-macos:
	$(call log,Generating MacOS binaries 🍎)
	@rustup target add aarch64-apple-darwin
	$(call log, aarch64-apple-darwin is added ✅)
	@rustup target add x86_64-apple-darwin
	$(call log, x86_64-apple-darwin is added ✅)
	@cargo-tauri build --target aarch64-apple-darwin
	$(call log, aarch64-apple-darwin is built ✅)
	@cargo-tauri build --target x86_64-apple-darwin
	$(call log, x86_64-apple-darwin is built ✅)
	@chmod +x /Users/runner/work/syng/syng/src/native/target/x86_64-apple-darwin/release/bundle/macos/Syng.app/Contents/MacOS/Syng
	$(call log, x86_64-apple-darwin is made executable ✅)
	@chmod +x /Users/runner/work/syng/syng/src/native/target/aarch64-apple-darwin/release/bundle/macos/Syng.app/Contents/MacOS/Syng
	$(call log, aarch64-apple-darwin is made executable ✅)

# Package linux arm64 binaries (aarch64-unknown-linux-gnu)
package-linux-arm64:
	$(call log,Packaging Linux arm64 binaries 🐧)
	@rustup target add aarch64-unknown-linux-gnu
	$(call log, aarch64-unknown-linux-gnu is added ✅)
	@cargo tauri build --target aarch64-unknown-linux-gnu
	$(call log, aarch64-unknown-linux-gnu is built ✅)

# Package linux amd64 binaries (x86_64-unknown-linux-gnu)
package-linux-amd64:
	$(call log,Packaging Linux x86_64 binaries 🐧)
	@rustup target add x86_64-unknown-linux-gnu
	$(call log, x86_64-unknown-linux-gnu is added ✅)
	@cargo tauri build --target x86_64-unknown-linux-gnu
	$(call log, x86_64-unknown-linux-gnu is built ✅)

# Package linux i686 binaries
package-linux-i686:
	$(call log,Packaging Linux i686 binaries 🐧)
	@rustup target add i686-unknown-linux-gnu
	$(call log, i686-unknown-linux-gnu is added ✅)
	@cargo tauri build --target i686-unknown-linux-gnu
	$(call log, i686-unknown-linux-gnu is built ✅)

# Rebuild the project
rebuild:
	$(call log,Rebuilding ${PROJECT} 🚀)
	@$(CARGO) rebuild
	$(call log, Rebuild is done ✅)

# Build the project in release mode
release:
	$(call log,Building ${PROJECT} in release mode 🚀)
	@$(CARGO) build --release
	$(call log, Release build is done ✅)

# Remove all builds
remove-all-builds:
	$(call log,Removing all builds 🧹)
	rm -rf target
	$(call log, All builds are removed ✅)

# Run Linux binary
run-linux:
	$(call log,Running Linux binary 🐧)
	./target/linux/x86_64-unknown-linux-musl/release/password-generator-pro
	$(call log, Running Linux binary! ✅)

# Run macOS binary
run-macos:
	$(call log,Running MacOS binary 🍎)
	./target/macos/aarch64-apple-darwin/release/password-generator-pro
	$(call log, Running MacOS binary! ✅)

# Run Windows binary
run-windows:
	$(call log,Running Windows binary 🪟)
	./target/windows/x86_64-pc-windows-gnu/release/password-generator-pro.exe
	$(call log, Running Windows binary! ✅)

# Check code for security issues
security:
	$(call log,Checking security of ${PROJECT} 🔐)
	@$(CARGO) security
	$(call log, Security check is done ✅)

# Run tests
test:
	$(call log,Testing ${PROJECT} 🧪)
	@$(CARGO) test
	$(call log, Tests are done ✅)

# Check unused dependencies
udeps:
	$(call log,Checking unused dependencies for ${PROJECT} 📦)
	@$(CARGO) udeps
	$(call log, Unused dependencies check is done ✅)

# Check memory leaks
valgrind:
	$(call log,Checking memory leaks for ${PROJECT} 🧠)
	@$(CARGO) valgrind
	$(call log, Memory leaks check is done ✅)

# Display the help menu
help:
	@echo "\n$$(tput bold)Welcome to Password Generator Pro!$$(tput sgr0)\nFast, Simple And Secure Password Generator$$(tput sgr0)\n"
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