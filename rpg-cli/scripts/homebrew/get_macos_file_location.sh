#!/usr/bin/env zsh

# Get latest release and find the url of the macos archive
LOC=$(curl -s https://api.github.com/repos/jomy10/rpg-lang/releases/latest | grep browser_download_url | grep macos)
LOC=$(./scripts/homebrew/parse_file_loc.rb $LOC)

echo "$LOC"