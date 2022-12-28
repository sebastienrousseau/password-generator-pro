# Password Generator Pro Makefile
# Maintainer: Sebastien Rousseau
# License: See LICENSE-APACHE and LICENSE-MIT files for details.

# Description: This Makefile is the root Makefile of the project.
# The main Makefile is located in the src-tauri/ directory.
# This Makefile is just used to run the main commands in the src-tauri/ directory.

# Usage: make [command]
#
# Commands:
#   build: Build the project
#   build-all: Build all the binaries
#   build-linux: Build Linux binary
#   build-macos: Build MacOS binary
#   build-release: Build the project in release mode
#   build-windows: Build Windows binary
#   clean: Clean the project
#   fix: Fix code
#   help: Show this help
#   install-pkgs: Install required packages
#   lint: Lint code
#   move-assets: Move assets
#   icon: Generate icon
#   package-windows: Package Windows binary
#   package-macos: Package macOS binary
#   package-linux-arm64: Package Linux ARM64 binary
#   package-linux-amd64: Package Linux AMD64 binary
#   package-linux-i686: Package Linux I686 binary
#   rebuild: Rebuild the project
#   release: Build the project in release mode
#   remove-all-builds: Remove all builds
#   run-linux: Run Linux binary
#   run-macos: Run macOS binary
#   run-windows: Run Windows binary
#   audit: Audit dependencies
#   test: Run tests
#   udeps: Find unused dependencies
#   valgrind: Run Valgrind
#

.DEFAULT_GOAL := help

# bloat: Evaluating resource allocation of the project
bloat:
	cd src-tauri/ && $(MAKE) bloat

# build: Building the project
build:
	cd src-tauri/ && $(MAKE) build

# build-all: Building all the binaries
build-all:
	cd src-tauri/ && $(MAKE) build-all

# build-linux: Generating Linux binary
build-linux:
	cd src-tauri/ && $(MAKE) build-linux

# build-macos: Generating MacOS binary
build-macos:
	cd src-tauri/ && $(MAKE) build-macos

# build-release: Build the project in release mode
build-release:
	cd src-tauri/ && $(MAKE) build-release

# build-windows: Generating Windows binary
build-windows:
	cd src-tauri/ && $(MAKE) build-windows

# clean: Clean the project
clean:
	cd src-tauri/ && $(MAKE) clean

# fix: Fix code
fix:
	cd src-tauri/ && $(MAKE) fix

# help: Show this help
help:
	cd src-tauri/ && $(MAKE) help

# install-pkgs: Install required packages
install-pkgs:
	cd src-tauri/ && $(MAKE) install-pkgs

# lint: Lint code
lint:
	cd src-tauri/ && $(MAKE) lint

# move-assets: Move assets
move-assets:
	cd src-tauri/ && $(MAKE) move-assets

# icon: Generate icon
icon:
	cd src-tauri/ && $(MAKE) icon

# package-windows: Package Windows binary
package-windows:
	cd src-tauri/ && $(MAKE) package-windows

# package-macos: Package macOS binary
package-macos:
	cd src-tauri/ && $(MAKE) package-macos

# package-linux-arm64: Package Linux ARM64 binary
package-linux-arm64:
	cd src-tauri/ && $(MAKE) package-linux-arm64

# package-linux-amd64: Package Linux AMD64 binary
package-linux-amd64:
	cd src-tauri/ && $(MAKE) package-linux-amd64

# package-linux-i686 : Package Linux I686 binary
package-linux-i686:
	cd src-tauri/ && $(MAKE) package-linux-i686

# rebuild: Rebuild the project
rebuild:
	cd src-tauri/ && $(MAKE) rebuild

# release: Build the project in release mode
release:
	cd src-tauri/ && $(MAKE) release

# remove-all-builds: 	Remove all builds
remove-all-builds:
	cd src-tauri/ && $(MAKE) remove-all-builds

# run-linux: Run Linux binary
run-linux:
	cd src-tauri/ && $(MAKE) run-linux

# run-macos: Run macOS binary
run-macos:
	cd src-tauri/ && $(MAKE) run-macos

# run-windows: Run Windows binary
run-windows:
	cd src-tauri/ && $(MAKE) run-windows

# audit: Audit dependencies
audit:
	cd src-tauri/ && $(MAKE) audit

# test: Run tests
test:
	cd src-tauri/ && $(MAKE) test

# udeps: Find unused dependencies
udeps:
	cd src-tauri/ && $(MAKE) udeps

# valgrind: Run Valgrind
valgrind:
	cd src-tauri/ && $(MAKE) valgrind
