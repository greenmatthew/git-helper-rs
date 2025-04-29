# Justfile for git-helper-rs

# Define variables
name := "git-helper"
target_dir := "target"
release_dir := target_dir / "release"
debug_dir := target_dir / "debug"
release_binary := release_dir / name
debug_binary := debug_dir / name
install_dir := "/usr/local/bin"

# Default recipe to run when just is called without arguments
default: help

# Display available commands and their descriptions
help:
    just -l

# Check code without building
check:
    cargo check

# Build the project in debug mode
build:
    cargo build

# Build the project in release mode
build-release:
    cargo build --release

# Run the binary in debug mode
run:
    cargo run

# Run the binary in release mode
run-release:
    cargo run --release

# Clean the build artifacts
clean:
    cargo clean

# Run tests
test:
    cargo test

# Run clippy with pedantic lints
clippy:
    cargo clippy -- -W clippy::pedantic

# Install the binary to the user's PATH
install: build-release
    @echo "Installing {{name}} to {{install_dir}}"
    @sudo cp {{release_binary}} {{install_dir}}
    @echo "Installation complete!"

# Uninstall the application
uninstall:
    @echo "Removing {{name}} from {{install_dir}}"
    @sudo rm -f {{install_dir}}/{{name}}
    @echo "Uninstallation complete!"