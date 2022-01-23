#!/usr/bin/env zsh
# Packages the 3 targets by archiving them with gzip
echo "Bundling binaries..."
tar -czf "/target/build/linux-aarch64.tar.gz" "/target/aarch64-unknown-linux-gnu/release/rpg-cli"
tar -czf "/target/build/windows-x86_64.tar.gz" "/target/x86_64-pc-windows-gnu/release/rpg-cli.exe"
tar -czf "/target/build/macos-x86_64.tar.gz" "/target/x86_64-apple-darwin/release/rpg-cli"

