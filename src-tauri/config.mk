# Makefile for Password Generator Pro
# Maintainer: Sebastien Rousseau
# License: See LICENSE file for copyright and license details.

# Valid COLOR options: {always, auto, never}
COLOR ?= auto

# Avoid funny character set dependencies
unexport LC_ALL
export LC_COLLATE=C
export LC_NUMERIC=C

# Make variables for the project
export ASSETS_DIR = ./assets
export BLOATARGS =  --release --crates --all-features
export CARGO = cargo --color $(COLOR)
export GREEN=\033[0;32m
export LINUX_TARGET = x86_64-unknown-linux-musl
export MACOS_TARGET = aarch64-apple-darwin
export MACOS_TARGET = arm64-apple-darwin
export NC=\033[0m
export PROJECT = Password Generator Pro
export VERSION = 0.0.1
export WINDOWS_TARGET = x86_64-pc-windows-gnu
