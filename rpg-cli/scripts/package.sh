#!/usr/bin/env zsh
# Packages the 3 targets by archiving them with gzip
echo "Bundling binaries..."

cp "target/aarch64-unknown-linux-gnu/release/rpg-cli" "build/linux-aarch64/rpgc"
tar -czf "build/linux-aarch64.tar.gz" "build/linux-aarch64/rpgc"

cp "target/x86_64-pc-windows-gnu/release/rpg-cli.exe" "build/windows-x86_64/rpgc.exe"
tar -czf "build/windows-x86_64.tar.gz" "build/windows-x86_64/rpgc.exe"

cp "target/x86_64-apple-darwin/release/rpg-cli" "build/macos-x86_64/rpgc"
tar -czf "build/macos-x86_64.tar.gz" "build/macos-x86_64/rpgc"
