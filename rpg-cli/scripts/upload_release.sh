#!/usr/bin/env zsh
# ENVIRONMENT VARIABLES
# - VERSION
# - BODY
# TODO! ??????

echo "Releasing new version $VERSION..."

token=$(./scripts/get_token.swift)
repo="jomy10/rpg-lang"

upload_url=$(curl -i -u jomy10:$token \
  -d "{\"tag_name\": \"v$VERSION\", \"name\":\"v$VERSION\", \"body\":\"Release version $VERSION\n$BODY\"}" \
  "https://api.github.com/repos/$repo/releases" | jq -r '.upload_url')

echo $upload_url

upload_url="${upload_url%\{*}"

echo "Uploading assets to release to url: $upload_url..."

curl -s -H "Authorization: token $token" \
        -H "Content-Type: application/zip" \
        --data-binary @build/linux-aarch64.tar.gz \
        "$upload_url?name=linux-aarch64.tar.gz&label=linux-aarch64.tar.gz"

curl -s -H "Authorization: token $token" \
        -H "Content-Type: application/zip" \
        --data-binary @build/macos-x86_64.tar.gz \
        "$upload_url?name=macos-x86_64.tar.gz&label=macos-x86_64.tar.gz"

curl -s -H "Authorization: token $token" \
        -H "Content-Type: application/zip" \
        --data-binary @build/windows-x86_64.tar.gz \
        "$upload_url?name=windows-x86_64.tar.gz&label=windows-x86_64.tar.gz"
