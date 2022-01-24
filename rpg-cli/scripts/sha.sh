#!/usr/bin/env zsh

SHA=$(shasum -a 256 "build/macos-x86_64.tar.gz")
SHA=$(SHA=$SHA parse_sha.swift)

echo "[34mSHA for macos gzip:[34m"
echo SHA