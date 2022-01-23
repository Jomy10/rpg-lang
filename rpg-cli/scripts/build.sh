#!/usr/bin/env zsh

echo "Building for MacOs..."
cargo build --release --target x86_64-apple-darwin

echo "Building for Linux..."
docker run --rm -it -v `pwd`:/app rust/linux-aarch64

echo "Building for Windows..."
docker run --rm -it -v `pwd`:/app rust/windows-x86_64

echo "[34mBuilding complete[0m"
