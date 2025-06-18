#!/usr/bin/env just --justfile

# List all of the available commands.
default:
  just --list

node:
    cargo build --bin mojave

    cargo run --bin mojave

# Fix some issues
fix flags="":
	cargo fix --allow-staged --all-targets --all-features {{flags}}
	cargo clippy --fix --allow-staged --all-targets --all-features {{flags}}
	cargo fmt --all

	# requires: cargo install cargo-shear
	cargo shear --fix

	# requires: cargo install cargo-sort
	cargo sort --workspace

	# requires: cargo install cargo-audit
	cargo audit

	# Update any patch versions
	cargo update

	# cargo install taplo-cli --locked
	taplo fmt

# Upgrade any tooling
upgrade:
	# Update any patch versions
	cargo update

	# Requires: cargo install cargo-upgrades cargo-edit
	cargo upgrade --incompatible

# Build the packages
build:
	cargo build
