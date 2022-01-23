FILE="build/linux-aarch64.tar.gz"
TOKEN=./scripts/get_token.swift

curl -i -H "Authorization: jomy10:$TOKEN" \
     -H "Accept: application/vnd.github.manifold-preview" \
     "https://uploads.github.com/repos/jomy10/rpg-lang/releases/tag/v0.0.1"